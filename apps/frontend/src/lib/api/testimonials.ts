/**
 * Testimonials API: public list + admin CRUD.
 */

import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

function getAuthHeaders(): HeadersInit {
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = { 'Content-Type': 'application/json' };
	if (token) headers['Authorization'] = `Bearer ${token}`;
	return headers;
}

export interface Testimonial {
	id: string;
	author_name: string;
	role_or_title: string | null;
	body: string;
	image_url: string | null;
	sort_order: number;
	created_at: string;
}

export async function getTestimonials(): Promise<Testimonial[]> {
	const res = await fetch(`${API_BASE}/testimonials`);
	const json = await safeJson<Testimonial[] | { message?: string }>(res);
	if (!res.ok) {
		const err = json as { message?: string };
		throw new Error(err.message ?? 'Failed to fetch testimonials');
	}
	return Array.isArray(json) ? json : [];
}

export async function listTestimonialsAdmin(): Promise<Testimonial[]> {
	const res = await fetch(`${API_BASE}/admin/testimonials`, { headers: getAuthHeaders() });
	const json = await safeJson<Testimonial[] | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to list testimonials');
	return Array.isArray(json) ? json : [];
}

export async function createTestimonial(body: {
	author_name: string;
	role_or_title?: string | null;
	body: string;
	image_url?: string | null;
	sort_order?: number;
}): Promise<Testimonial> {
	const res = await fetch(`${API_BASE}/admin/testimonials`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<Testimonial | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to create testimonial');
	return json as Testimonial;
}

export async function updateTestimonial(
	id: string,
	body: {
		author_name?: string;
		role_or_title?: string | null;
		body?: string;
		image_url?: string | null;
		sort_order?: number;
	}
): Promise<Testimonial> {
	const res = await fetch(`${API_BASE}/admin/testimonials/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<Testimonial | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to update testimonial');
	return json as Testimonial;
}

export async function deleteTestimonial(id: string): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/testimonials/${encodeURIComponent(id)}`, {
		method: 'DELETE',
		headers: getAuthHeaders()
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json.message ?? 'Failed to delete testimonial');
	}
}
