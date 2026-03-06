import { test, expect } from '@playwright/test';

test.describe('Contact page', () => {
	test('page loads with contact form', async ({ page }) => {
		await page.goto('/contact');
		await expect(page).toHaveTitle(/Contact — Prophetic Glorious Manifestation Pastoral School/);
		await expect(
			page.getByRole('heading', { name: 'Contact the Pastoral School', level: 1 })
		).toBeVisible();
		await expect(page.getByLabel('Name')).toBeVisible();
		await expect(page.getByLabel('Email')).toBeVisible();
		await expect(page.getByLabel('Subject (optional)')).toBeVisible();
		await expect(page.getByLabel('Message')).toBeVisible();
		await expect(page.getByRole('button', { name: 'Send message' })).toBeVisible();
	});

	test('footer has quick and legal links', async ({ page }) => {
		await page.goto('/contact');
		const footer = page.locator('footer');
		await expect(footer.getByRole('link', { name: 'Programs' })).toHaveAttribute(
			'href',
			'/programs'
		);
		await expect(footer.getByRole('link', { name: 'Admissions' })).toHaveAttribute(
			'href',
			'/admissions'
		);
		await expect(footer.getByRole('link', { name: 'Events' })).toHaveAttribute(
			'href',
			'/events'
		);
		await expect(footer.getByRole('link', { name: 'Contact' })).toHaveAttribute(
			'href',
			'/contact'
		);
		await expect(footer.getByRole('link', { name: 'Privacy' })).toHaveAttribute(
			'href',
			'/privacy'
		);
		await expect(footer.getByRole('link', { name: 'Terms' })).toHaveAttribute('href', '/terms');
		await expect(footer.getByRole('link', { name: 'Cookies' })).toHaveAttribute(
			'href',
			'/cookies'
		);
	});
});
