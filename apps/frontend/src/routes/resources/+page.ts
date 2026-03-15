import { getResources } from '$lib/api/resources';
import type { PageLoad } from './$types';

export const prerender = false;

export const load: PageLoad = async ({ url, fetch }) => {
	const courseId = url.searchParams.get('course_id') ?? undefined;
	const resources = await getResources(courseId ?? null, fetch);
	return { resources, courseId: courseId ?? null };
};
