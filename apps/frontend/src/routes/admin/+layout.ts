// Admin uses $page.url.pathname and auth; must not be prerendered.
export const prerender = false;

export function load() {
	// Auth check is done client-side in +layout.svelte (admin uses same token; backend can enforce role later)
	return {};
}
