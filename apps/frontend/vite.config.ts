import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],
	server: {
		port: 5180,
		proxy: {
			// Contact form and API: proxy to backend during local dev (backend must run on 6830)
			'/api/v1': {
				target: process.env.BACKEND_URL || 'http://127.0.0.1:6830',
				changeOrigin: true
			}
		}
	}
});
