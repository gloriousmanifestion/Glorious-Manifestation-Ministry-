/**
 * Church services and site content API.
 * Public: GET /church/services, GET /church/content/:key
 * Admin: CRUD services, upload artwork, set content (uses auth token).
 */

import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

function getAuthHeaders(): HeadersInit {
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = { 'Content-Type': 'application/json' };
	if (token) headers['Authorization'] = `Bearer ${token}`;
	return headers;
}

// --- Public types (match backend snake_case) ---

export interface ChurchService {
	id: string;
	name: string;
	description: string | null;
	day_of_week: string;
	time: string;
	sort_order: number;
	image_url: string | null;
	features: string[] | null;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

// --- Public API ---

export async function getChurchServices(): Promise<ChurchService[]> {
	const res = await fetch(`${API_BASE}/church/services`);
	const json = await safeJson<ChurchService[] | { message?: string }>(res);
	if (!res.ok) {
		const err = json as { message?: string };
		throw new Error(err.message ?? 'Failed to fetch church services');
	}
	return Array.isArray(json) ? json : [];
}

export async function getChurchContent(key: string): Promise<string | null> {
	const res = await fetch(`${API_BASE}/church/content/${encodeURIComponent(key)}`);
	const json = await safeJson<{ key: string; value: string } | { error?: string }>(res);
	if (!res.ok || !json || typeof (json as { value?: string }).value !== 'string') {
		return null;
	}
	return (json as { key: string; value: string }).value;
}

// --- Admin API ---

export async function listChurchServicesAdmin(): Promise<ChurchService[]> {
	const res = await fetch(`${API_BASE}/admin/church/services`, { headers: getAuthHeaders() });
	const json = await safeJson<ChurchService[] | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to list services');
	return Array.isArray(json) ? json : [];
}

export interface CreateChurchServiceBody {
	name: string;
	description?: string;
	day_of_week: string;
	time: string;
	sort_order?: number;
	features?: string[];
	is_active?: boolean;
}

export async function createChurchService(body: CreateChurchServiceBody): Promise<ChurchService> {
	const res = await fetch(`${API_BASE}/admin/church/services`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<ChurchService | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to create service');
	return json as ChurchService;
}

export interface UpdateChurchServiceBody {
	name?: string;
	description?: string;
	day_of_week?: string;
	time?: string;
	sort_order?: number;
	image_url?: string | null;
	features?: string[];
	is_active?: boolean;
}

export async function updateChurchService(
	id: string,
	body: UpdateChurchServiceBody
): Promise<ChurchService> {
	const res = await fetch(`${API_BASE}/admin/church/services/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<ChurchService | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to update service');
	return json as ChurchService;
}

export async function deleteChurchService(id: string): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/church/services/${encodeURIComponent(id)}`, {
		method: 'DELETE',
		headers: getAuthHeaders()
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json.message ?? 'Failed to delete service');
	}
}

/** Upload artwork; returns image_url (filename to store on service) and url (for display). */
export async function uploadChurchArtwork(file: File): Promise<{ image_url: string; url: string }> {
	const form = new FormData();
	form.append('file', file);
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = {};
	if (token) headers['Authorization'] = `Bearer ${token}`;

	const res = await fetch(`${API_BASE}/admin/church/services/upload`, {
		method: 'POST',
		headers,
		body: form
	});
	const json = await safeJson<{ image_url?: string; url?: string } | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Upload failed');
	const out = json as { image_url?: string; url?: string };
	if (!out.image_url) throw new Error('Upload response missing image_url');
	return {
		image_url: out.image_url,
		url: out.url ?? `${API_BASE}/uploads/church/${out.image_url}`
	};
}

/** Resolve full URL for an image (filename or path) returned by the API. */
export function churchImageUrl(path: string | null | undefined): string {
	if (!path) return '';
	if (path.startsWith('http') || path.startsWith('/')) return path;
	return `${API_BASE}/uploads/church/${path.replace(/^.*\//, '')}`;
}

export async function setChurchContent(key: string, value: string): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/church/content/${encodeURIComponent(key)}`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({ value })
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json.message ?? 'Failed to save content');
	}
}
