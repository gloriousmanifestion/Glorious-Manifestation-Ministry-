import { test, expect } from '@playwright/test';
import { PLUS_STAGING, BUSINESS_STAGING } from './helpers';

test.describe('Careers page', () => {
	test('page loads with careers content', async ({ page }) => {
		await page.goto('/careers');
		await expect(page).toHaveTitle(/Careers — 1nga Solutions/);
		await expect(page.getByRole('heading', { name: 'Build the future with us' })).toBeVisible();
	});

	test('Sign in link points to staging Plus URL', async ({ page }) => {
		await page.goto('/careers');
		const signIn = page.getByRole('link', { name: 'Sign in' });
		await expect(signIn).toBeVisible();
		await expect(signIn).toHaveAttribute('href', PLUS_STAGING);
	});

	test('Partner with us link points to staging Business URL', async ({ page }) => {
		await page.goto('/careers');
		const partner = page.getByRole('link', { name: 'Partner with us' });
		await expect(partner).toBeVisible();
		await expect(partner).toHaveAttribute('href', BUSINESS_STAGING);
	});

	test('footer has Contact, Careers, Terms, Privacy, and Cookies links', async ({ page }) => {
		await page.goto('/careers');
		const footer = page.locator('footer');
		await expect(footer.getByRole('link', { name: 'Contact' })).toHaveAttribute('href', '/contact');
		await expect(footer.getByRole('link', { name: 'Careers' })).toHaveAttribute('href', '/careers');
		await expect(footer.getByRole('link', { name: 'Terms' })).toHaveAttribute('href', '/terms');
		await expect(footer.getByRole('link', { name: 'Privacy' })).toHaveAttribute('href', '/privacy');
		await expect(footer.getByRole('link', { name: 'Cookies' })).toHaveAttribute('href', '/cookies');
	});
});
