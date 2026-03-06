import { getCourseBySlug } from '$lib/courses';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const course = getCourseBySlug(params.slug);
	if (!course) {
		return { course: null };
	}
	return { course };
};
