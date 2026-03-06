import { defineConfig, devices } from '@playwright/test';

const isCI = !!process.env.CI;
const port = isCI ? parseInt(process.env.PLAYWRIGHT_TEST_PORT ?? '5181', 10) : 5180;
const baseURL = `http://localhost:${port}`;

export default defineConfig({
	testDir: './tests',
	fullyParallel: true,
	forbidOnly: isCI,
	retries: isCI ? 2 : 0,
	workers: isCI ? 4 : undefined,
	reporter: 'html',
	timeout: 30_000,
	expect: { timeout: 10_000 },
	use: {
		baseURL,
		actionTimeout: 10_000,
		navigationTimeout: 15_000,
		trace: 'on-first-retry'
	},
	projects: [
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'] }
		}
	],
	webServer: {
		command:
			isCI && process.env.CI_USE_PREBUILT
				? `PORT=${port} node serve.mjs`
				: isCI
					? `PUBLIC_PLUS_URL=https://beta-plus.1nga.online PUBLIC_BUSINESS_URL=https://beta-bus.1nga.online pnpm build && PORT=${port} node serve.mjs`
					: 'pnpm dev',
		url: baseURL,
		reuseExistingServer: !isCI,
		timeout: 180_000
	}
});
