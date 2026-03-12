import type { PageLoad } from './$types';

// Dynamic route: token is request-specific; do not prerender.
export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	return { token: params.token };
};
