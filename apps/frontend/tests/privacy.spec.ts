import { test, expect } from '@playwright/test';

test.describe('Privacy page', () => {
	test('page loads with privacy content', async ({ page }) => {
		await page.goto('/privacy');
		await expect(page).toHaveTitle(/Privacy Policy — Pastoral School/);
		await expect(page.getByRole('heading', { name: 'Privacy Policy', level: 1 })).toBeVisible();
	});

	test('Back to home link navigates to home', async ({ page }) => {
		await page.goto('/privacy');
		const backLink = page.getByRole('link', { name: 'Back to home' });
		await expect(backLink).toBeVisible();
		await expect(backLink).toHaveAttribute('href', '/');
	});

	test('Pastoral School branding links to home', async ({ page }) => {
		await page.goto('/privacy');
		const logoLink = page.getByRole('link', { name: /Prophetic Glorious Manifestation \| Pastoral School/ }).first();
		await expect(logoLink).toBeVisible();
		await expect(logoLink).toHaveAttribute('href', '/');
	});
});
