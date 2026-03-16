/**
 * Public API: resource library (optional filter by course).
 */
import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

export interface Resource {
	id: string;
	course_id: string | null;
	title: string;
	url: string;
	resource_type: string;
	sort_order: number;
	created_at: string;
}

export async function getResources(
	courseId?: string | null,
	fetchFn: typeof fetch = fetch
): Promise<Resource[]> {
	const url = courseId
		? `${API_BASE}/resources?course_id=${encodeURIComponent(courseId)}`
		: `${API_BASE}/resources`;
	const res = await fetchFn(url);
	const data = await safeJson<Resource[]>(res).catch(() => []);
	return Array.isArray(data) ? data : [];
}
