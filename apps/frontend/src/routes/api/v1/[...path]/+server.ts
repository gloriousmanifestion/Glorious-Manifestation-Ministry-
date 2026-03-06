/**
 * Catch-all proxy: /api/v1/* -> backend (e.g. http://127.0.0.1:6830/api/v1/*).
 * Browser always calls same-origin /api/v1/... so no CORS. Server forwards to backend.
 */
import { proxyToBackend } from '$lib/api-client';
import type { RequestHandler } from './$types';

export const prerender = false;

function backendPath(path: string): string {
	const segment = path?.trim() || '';
	return segment ? `/api/v1/${segment}` : '/api/v1';
}

export const GET: RequestHandler = (event) => {
	return proxyToBackend(event, backendPath(event.params.path));
};
export const POST: RequestHandler = (event) => {
	return proxyToBackend(event, backendPath(event.params.path));
};
export const PUT: RequestHandler = (event) => {
	return proxyToBackend(event, backendPath(event.params.path));
};
export const PATCH: RequestHandler = (event) => {
	return proxyToBackend(event, backendPath(event.params.path));
};
export const DELETE: RequestHandler = (event) => {
	return proxyToBackend(event, backendPath(event.params.path));
};
export const OPTIONS: RequestHandler = (event) => {
	return proxyToBackend(event, backendPath(event.params.path));
};
