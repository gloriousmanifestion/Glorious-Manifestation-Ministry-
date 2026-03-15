/**
 * Public API: course curriculum / syllabus (modules per course)
 */
import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

export interface CourseModule {
	id: string;
	course_id: string;
	title: string;
	sort_order: number;
	link: string | null;
	description: string | null;
	created_at: string;
}

/** Pass fetch from load event in SSR so relative URL resolves. */
export async function getCourseModules(
	courseId: string,
	fetchFn: typeof fetch = fetch
): Promise<CourseModule[]> {
	const url = `${API_BASE}/courses/${encodeURIComponent(courseId)}/modules`;
	const res = await fetchFn(url);
	if (!res.ok) return [];
	const data = await safeJson<CourseModule[]>(res).catch(() => []);
	return Array.isArray(data) ? data : [];
}
