/**
 * Blog/News API: public list + get by slug, admin CRUD.
 */

import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

function getAuthHeaders(): HeadersInit {
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = { 'Content-Type': 'application/json' };
	if (token) headers['Authorization'] = `Bearer ${token}`;
	return headers;
}

export interface BlogPost {
	id: string;
	title: string;
	slug: string;
	body: string;
	author_name: string;
	image_url: string | null;
	published_at: string | null;
	created_at: string;
	updated_at: string;
}

export async function getBlogPosts(): Promise<BlogPost[]> {
	const res = await fetch(`${API_BASE}/blog`);
	const json = await safeJson<BlogPost[] | { message?: string }>(res);
	if (!res.ok) {
		const err = json as { message?: string };
		throw new Error(err.message ?? 'Failed to fetch posts');
	}
	return Array.isArray(json) ? json : [];
}

export async function getBlogPostBySlug(slug: string): Promise<BlogPost | null> {
	const res = await fetch(`${API_BASE}/blog/${encodeURIComponent(slug)}`);
	const json = await safeJson<BlogPost | { message?: string }>(res);
	if (res.status === 404) return null;
	if (!res.ok) {
		const err = json as { message?: string };
		throw new Error(err.message ?? 'Failed to fetch post');
	}
	return json as BlogPost;
}

export async function listBlogPostsAdmin(): Promise<BlogPost[]> {
	const res = await fetch(`${API_BASE}/admin/blog`, { headers: getAuthHeaders() });
	const json = await safeJson<BlogPost[] | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to list posts');
	return Array.isArray(json) ? json : [];
}

export async function createBlogPost(body: {
	title: string;
	slug?: string | null;
	body: string;
	author_name: string;
	image_url?: string | null;
	published_at?: string | null;
}): Promise<BlogPost> {
	const res = await fetch(`${API_BASE}/admin/blog`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<BlogPost | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to create post');
	return json as BlogPost;
}

export async function updateBlogPost(
	id: string,
	body: {
		title?: string;
		slug?: string;
		body?: string;
		author_name?: string;
		image_url?: string | null;
		published_at?: string | null;
	}
): Promise<BlogPost> {
	const res = await fetch(`${API_BASE}/admin/blog/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<BlogPost | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to update post');
	return json as BlogPost;
}

export async function deleteBlogPost(id: string): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/blog/${encodeURIComponent(id)}`, {
		method: 'DELETE',
		headers: getAuthHeaders()
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json.message ?? 'Failed to delete post');
	}
}

// --- Comments (public list, auth to add) ---

export interface BlogComment {
	id: string;
	blog_post_id: string;
	user_id: string;
	author_name: string;
	body: string;
	created_at: string;
}

export async function getBlogComments(slug: string): Promise<BlogComment[]> {
	const res = await fetch(`${API_BASE}/blog/${encodeURIComponent(slug)}/comments`);
	const json = await safeJson<BlogComment[] | { message?: string }>(res);
	if (!res.ok) {
		throw new Error((json as { message?: string }).message ?? 'Failed to load comments');
	}
	return Array.isArray(json) ? json : [];
}

export async function addBlogComment(slug: string, body: string): Promise<BlogComment> {
	const res = await fetch(`${API_BASE}/blog/${encodeURIComponent(slug)}/comments`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({ body })
	});
	const json = await safeJson<BlogComment | { message?: string }>(res);
	if (!res.ok) {
		throw new Error((json as { message?: string }).message ?? 'Failed to add comment');
	}
	return json as BlogComment;
}
