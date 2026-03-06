/**
 * Server-side API client for the backend.
 * Used only by the /api/v1/* proxy route. The browser always calls same-origin /api/v1/...;
 * this module forwards those requests to the backend (e.g. http://127.0.0.1:6830 on the VM).
 */
import type { RequestEvent } from '@sveltejs/kit';

export function getApiUrl(): string {
	if (typeof process === 'undefined') return 'http://127.0.0.1:6830';
	return process.env.BACKEND_URL || process.env.PUBLIC_API_URL || 'http://127.0.0.1:6830';
}

/**
 * Proxy a request to the backend and return the response.
 */
export async function proxyToBackend(
	event: RequestEvent,
	path: string,
	init: RequestInit = {}
): Promise<Response> {
	const url = event.url;
	const query = url.searchParams.toString();
	const fullPath = query ? `${path}${path.includes('?') ? '&' : '?'}${query}` : path;
	const method = (init.method ?? event.request.method) as string;
	const hasBody = method !== 'GET' && method !== 'HEAD';
	const body = init.body ?? (hasBody ? await event.request.arrayBuffer() : undefined);
	const headers = new Headers(init.headers);
	if (hasBody && body && !headers.has('Content-Type')) {
		const ct = event.request.headers.get('Content-Type');
		if (ct) headers.set('Content-Type', ct);
	}
	headers.delete('Host');

	const base = getApiUrl();
	const backendUrl = fullPath.startsWith('http')
		? fullPath
		: `${base}${fullPath.startsWith('/') ? '' : '/'}${fullPath}`;

	const timeoutMs = Number(process.env.BACKEND_PROXY_TIMEOUT_MS) || 25_000;
	const controller = new AbortController();
	const timeoutId = setTimeout(() => controller.abort(), timeoutMs);

	let res: Response;
	try {
		res = await fetch(backendUrl, {
			...init,
			method,
			body,
			headers,
			signal: controller.signal
		});
	} catch (err) {
		const message =
			err instanceof Error && err.name === 'AbortError'
				? 'Backend did not respond in time.'
				: 'Backend is temporarily unavailable.';
		return new Response(JSON.stringify({ error: 'service_unavailable', message }), {
			status: 503,
			headers: { 'Content-Type': 'application/json' }
		});
	} finally {
		clearTimeout(timeoutId);
	}

	const outHeaders = new Headers();
	res.headers.forEach((value, key) => {
		const lower = key.toLowerCase();
		if (lower === 'access-control-allow-origin') return;
		outHeaders.set(key, value);
	});

	return new Response(await res.arrayBuffer(), {
		status: res.status,
		statusText: res.statusText,
		headers: outHeaders
	});
}
