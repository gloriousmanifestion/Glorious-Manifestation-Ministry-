import { test, expect } from '@playwright/test';

test.describe('Terms page', () => {
	test('page loads with terms content', async ({ page }) => {
		await page.goto('/terms');
		await expect(page).toHaveTitle(/Terms of Service — Pastoral School/);
		await expect(page.getByRole('heading', { name: 'Terms of Service', level: 1 })).toBeVisible();
	});

	test('Back to home link navigates to home', async ({ page }) => {
		await page.goto('/terms');
		const backLink = page.getByRole('link', { name: 'Back to home' });
		await expect(backLink).toBeVisible();
		await expect(backLink).toHaveAttribute('href', '/');
	});

	test('Pastoral School branding links to home', async ({ page }) => {
		await page.goto('/terms');
		const logoLink = page.getByRole('link', { name: /Prophetic Glorious Manifestation \| Pastoral School/ }).first();
		await expect(logoLink).toBeVisible();
		await expect(logoLink).toHaveAttribute('href', '/');
	});
});
