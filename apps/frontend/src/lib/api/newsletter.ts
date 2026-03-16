/**
 * Newsletter API - subscribe visitor email
 */
import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

export interface SubscribeResponse {
	subscribed: boolean;
	message: string;
}

export async function subscribeNewsletter(email: string): Promise<SubscribeResponse> {
	const res = await fetch(`${API_BASE}/newsletter/subscribe`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ email: email.trim() })
	});
	const json = await safeJson<SubscribeResponse & { error?: string; message?: string }>(res);
	if (!res.ok) {
		throw new Error(json.message || json.error || 'Failed to subscribe');
	}
	return json as SubscribeResponse;
}
