import { defineConfig } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite'

export default defineConfig({
  plugins: [sveltekit()],
  server: {
    proxy: {
      '/api': {
        target: 'https://glorious-manifestation-ministry-s6co.onrender.com',
        changeOrigin: true
      }
    }
  }
})
