import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			// Exclude dynamic API proxy from static build; contact form uses PUBLIC_API_URL when deployed.
			strict: false
		})
	}
};

export default config;
