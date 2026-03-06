#!/usr/bin/env node
/**
 * Production entry point for frontend on VM (systemd) and Playwright tests.
 * - adapter-node (SSR): starts build/index.js (SvelteKit Node server)
 * - adapter-static (legacy): serves build/ as static files (Node built-ins only)
 * Usage: node serve.mjs (run from deploy dir, e.g. /opt/pgm/frontend/staging|production)
 */

import { readFileSync, existsSync, createReadStream, statSync } from 'node:fs';
import { join, dirname, extname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createServer, request as httpRequest } from 'node:http';
import { request as httpsRequest } from 'node:https';

const __dirname = dirname(fileURLToPath(import.meta.url));

const MIME = {
	'.html': 'text/html',
	'.css': 'text/css',
	'.js': 'application/javascript',
	'.json': 'application/json',
	'.ico': 'image/x-icon',
	'.png': 'image/png',
	'.jpg': 'image/jpeg',
	'.jpeg': 'image/jpeg',
	'.gif': 'image/gif',
	'.svg': 'image/svg+xml',
	'.woff': 'font/woff',
	'.woff2': 'font/woff2',
	'.webp': 'image/webp'
};

// Load .env if present (PORT, BACKEND_URL, etc.)
const envPath = join(__dirname, '.env');
if (existsSync(envPath)) {
	const content = readFileSync(envPath, 'utf8');
	for (const line of content.split('\n')) {
		const trimmed = line.trim();
		if (trimmed && !trimmed.startsWith('#')) {
			const eq = trimmed.indexOf('=');
			if (eq > 0) {
				const key = trimmed.slice(0, eq).trim();
				let value = trimmed.slice(eq + 1).trim();
				if (
					(value.startsWith('"') && value.endsWith('"')) ||
					(value.startsWith("'") && value.endsWith("'"))
				) {
					value = value.slice(1, -1);
				}
				process.env[key] = value;
			}
		}
	}
}

// Default port for website (override via .env or systemd EnvironmentFile)
if (!process.env.PORT) {
	process.env.PORT = '5180';
}

const buildDir = join(__dirname, 'build');
const nodeServerPath = join(buildDir, 'index.js');

if (existsSync(nodeServerPath)) {
	// adapter-node: start the built SvelteKit server
	await import(new URL('./build/index.js', import.meta.url).href);
} else if (existsSync(buildDir)) {
	// adapter-static: serve build/ as static files; proxy /api/v1/* to backend
	const port = parseInt(process.env.PORT, 10);
	const backendUrl = process.env.BACKEND_URL || '';
	const proxyApiPrefix = '/api/v1/';

	const server = createServer((req, res) => {
		let path = req.url?.split('?')[0] || '/';
		const query = req.url?.includes('?') ? '?' + req.url.split('?')[1] : '';

		// API requests: proxy when BACKEND_URL is set; otherwise return 503 (contact form needs backend)
		if (path.startsWith(proxyApiPrefix)) {
			if (!backendUrl) {
				res.writeHead(503, { 'Content-Type': 'application/json' });
				res.end(
					JSON.stringify({
						error: 'service_unavailable',
						message:
							'The contact form is temporarily unavailable. Please email us directly at hello@1nga.com.'
					})
				);
				return;
			}
			try {
				const backend = new URL(backendUrl);
				const proxyPath = path + query;
				const proxyOpts = {
					hostname: backend.hostname,
					port: backend.port || (backend.protocol === 'https:' ? 443 : 80),
					path: proxyPath,
					method: req.method,
					headers: { ...req.headers, host: backend.host }
				};
				const reqFn = backend.protocol === 'https:' ? httpsRequest : httpRequest;
				const proxyReq = reqFn(proxyOpts, (proxyRes) => {
					res.writeHead(proxyRes.statusCode || 500, proxyRes.headers);
					proxyRes.pipe(res);
				});
				proxyReq.on('error', (err) => {
					console.error('Proxy error:', err.message);
					res.writeHead(502, { 'Content-Type': 'application/json' });
					res.end(JSON.stringify({ error: 'Bad Gateway', message: 'Backend unavailable' }));
				});
				req.pipe(proxyReq);
			} catch (err) {
				console.error('Proxy setup error:', err.message);
				res.writeHead(502, { 'Content-Type': 'application/json' });
				res.end(JSON.stringify({ error: 'Bad Gateway', message: 'Invalid BACKEND_URL' }));
			}
			return;
		}

		// Static file serving
		if (path === '/') path = '/index.html';
		// Prevent path traversal
		if (path.includes('..')) {
			res.writeHead(400).end('Bad Request');
			return;
		}
		const seg = path.slice(1).replace(/\/$/, '') || 'index';
		let file = join(buildDir, seg);
		// Clean URLs: /careers -> careers.html, /_app/... -> as-is
		if (!extname(file) && !existsSync(file)) {
			const withHtml = file + '.html';
			if (existsSync(withHtml)) file = withHtml;
		}
		if (!existsSync(file) || !statSync(file).isFile()) {
			res.writeHead(404).end('Not Found');
			return;
		}
		const ext = extname(file);
		const type = MIME[ext] || 'application/octet-stream';
		res.setHeader('Content-Type', type);
		createReadStream(file).pipe(res);
	});
	server.listen(port, () => {
		console.log(`Serving static build at http://localhost:${port}`);
		if (backendUrl) {
			console.log(`Proxying ${proxyApiPrefix}* to ${backendUrl}`);
		}
	});
} else {
	console.error('Error: build/ directory not found. Run pnpm build first.');
	process.exit(1);
}
