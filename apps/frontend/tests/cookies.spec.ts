import { test, expect } from '@playwright/test';

test.describe('Cookies page', () => {
	test('page loads with cookie policy content', async ({ page }) => {
		await page.goto('/cookies');
		await expect(page).toHaveTitle(/Cookie Policy — Pastoral School/);
		await expect(page.getByRole('heading', { name: 'Cookie Policy', level: 1 })).toBeVisible();
	});

	test('Back to home link navigates to home', async ({ page }) => {
		await page.goto('/cookies');
		const backLink = page.getByRole('link', { name: 'Back to home' });
		await expect(backLink).toBeVisible();
		await expect(backLink).toHaveAttribute('href', '/');
	});

	test('Pastoral School branding links to home', async ({ page }) => {
		await page.goto('/cookies');
		const logoLink = page.getByRole('link', { name: /PGM \| Pastoral School/ }).first();
		await expect(logoLink).toBeVisible();
		await expect(logoLink).toHaveAttribute('href', '/');
	});
});
