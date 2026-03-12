import { test, expect } from '@playwright/test';

test.describe('Home page', () => {
	test('page loads with Pastoral School branding', async ({ page }) => {
		await page.goto('/');
		await expect(page).toHaveTitle(/Pastoral School — Prophetic Glorious Manifestation/);
		await expect(page.getByRole('heading', { name: /Pastoral School/, level: 1 })).toBeVisible();
	});

	test('hero CTAs link to register and programs', async ({ page }) => {
		await page.goto('/');
		const register = page.getByRole('link', { name: 'Register now' }).first();
		await expect(register).toBeVisible();
		await expect(register).toHaveAttribute('href', '/register');

		const learnMore = page.getByRole('link', { name: 'Learn more' });
		await expect(learnMore).toBeVisible();
		await expect(learnMore).toHaveAttribute('href', '#programs');
	});

	test('footer has quick and legal links', async ({ page }) => {
		await page.goto('/');
		const footer = page.locator('footer');
		await expect(footer.getByRole('link', { name: 'Programs' })).toHaveAttribute(
			'href',
			'/programs'
		);
		await expect(footer.getByRole('link', { name: 'Admissions' })).toHaveAttribute(
			'href',
			'/admissions'
		);
		await expect(footer.getByRole('link', { name: 'Events' })).toHaveAttribute('href', '/events');
		await expect(footer.getByRole('link', { name: 'Contact' })).toHaveAttribute('href', '/contact');
		await expect(footer.getByRole('link', { name: 'Privacy' })).toHaveAttribute('href', '/privacy');
		await expect(footer.getByRole('link', { name: 'Terms' })).toHaveAttribute('href', '/terms');
		await expect(footer.getByRole('link', { name: 'Cookies' })).toHaveAttribute('href', '/cookies');
	});
});
