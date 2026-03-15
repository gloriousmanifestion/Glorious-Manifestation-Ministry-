/**
 * Announcements API: public list + admin CRUD.
 * GET /api/v1/announcements (public), GET/POST /admin/announcements, PATCH/DELETE /admin/announcements/:id
 */

import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

function getAuthHeaders(): HeadersInit {
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = { 'Content-Type': 'application/json' };
	if (token) headers['Authorization'] = `Bearer ${token}`;
	return headers;
}

export interface Announcement {
	id: string;
	title: string;
	body: string;
	created_at: string;
}

export async function getAnnouncements(): Promise<Announcement[]> {
	const res = await fetch(`${API_BASE}/announcements`);
	const json = await safeJson<Announcement[] | { message?: string }>(res);
	if (!res.ok) {
		const err = json as { message?: string };
		throw new Error(err.message ?? 'Failed to fetch announcements');
	}
	return Array.isArray(json) ? json : [];
}

export async function listAnnouncementsAdmin(): Promise<Announcement[]> {
	const res = await fetch(`${API_BASE}/admin/announcements`, { headers: getAuthHeaders() });
	const json = await safeJson<Announcement[] | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to list announcements');
	return Array.isArray(json) ? json : [];
}

export async function createAnnouncement(body: {
	title: string;
	body: string;
}): Promise<Announcement> {
	const res = await fetch(`${API_BASE}/admin/announcements`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<Announcement | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to create announcement');
	return json as Announcement;
}

export async function updateAnnouncement(
	id: string,
	body: { title?: string; body?: string }
): Promise<Announcement> {
	const res = await fetch(`${API_BASE}/admin/announcements/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<Announcement | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to update announcement');
	return json as Announcement;
}

export async function deleteAnnouncement(id: string): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/announcements/${encodeURIComponent(id)}`, {
		method: 'DELETE',
		headers: getAuthHeaders()
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json.message ?? 'Failed to delete announcement');
	}
}
