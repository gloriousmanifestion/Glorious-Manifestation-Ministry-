/**
 * Contact API - submits contact form to backend
 */
import { safeJson } from '$lib/api/safe-json';

export interface ContactRequest {
	name: string;
	email: string;
	subject?: string;
	message: string;
}

const API_BASE = '/api/v1';

export async function submitContact(
	data: ContactRequest
): Promise<{ success: boolean; message: string }> {
	const res = await fetch(`${API_BASE}/contact`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(data)
	});
	const json = await safeJson<{ success?: boolean; message?: string; error?: string }>(res);
	if (!res.ok) {
		throw new Error(json.message || json.error || 'Failed to send message');
	}
	return json as { success: boolean; message: string };
}
