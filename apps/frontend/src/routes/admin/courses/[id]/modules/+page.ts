import * as adminApi from '$lib/admin-api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const courses = await adminApi.getAdminCourses();
	const course = courses.find((c) => c.id === params.id) ?? null;
	return { courseId: params.id, courseName: course?.name ?? params.id };
};
