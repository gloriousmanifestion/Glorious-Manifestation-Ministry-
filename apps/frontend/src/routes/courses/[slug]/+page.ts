import { getCourseModules } from '$lib/api/course-modules';
import { getCourseBySlug } from '$lib/courses';
import type { PageLoad } from './$types';

// Dynamic route: load fetches modules from API; needs request context.
export const prerender = false;

export const load: PageLoad = async ({ params, fetch }) => {
	const course = getCourseBySlug(params.slug);
	if (!course) {
		return { course: null, modules: [] };
	}
	const modules = await getCourseModules(course.id, fetch);
	return { course, modules };
};
