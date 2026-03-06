import { test, expect } from '@playwright/test';
import { PLUS_STAGING } from './helpers';

test.describe('Contact page', () => {
	test('page loads with contact form', async ({ page }) => {
		await page.goto('/contact');
		await expect(page).toHaveTitle(/Contact — 1nga Solutions/);
		await expect(page.getByRole('heading', { name: 'Get in touch' })).toBeVisible();
		await expect(page.getByLabel('Name')).toBeVisible();
		await expect(page.getByLabel('Email')).toBeVisible();
		await expect(page.getByLabel('Subject')).toBeVisible();
		await expect(page.getByLabel('Message')).toBeVisible();
		await expect(page.getByRole('button', { name: 'Send message' })).toBeVisible();
	});

	test('Sign in link points to staging Plus URL', async ({ page }) => {
		await page.goto('/contact');
		const signIn = page.getByRole('link', { name: 'Sign in' });
		await expect(signIn).toBeVisible();
		await expect(signIn).toHaveAttribute('href', PLUS_STAGING);
	});

	test('footer has Contact, Careers, Terms, Privacy, and Cookies links', async ({ page }) => {
		await page.goto('/contact');
		const footer = page.locator('footer');
		await expect(footer.getByRole('link', { name: 'Contact' })).toHaveAttribute('href', '/contact');
		await expect(footer.getByRole('link', { name: 'Careers' })).toHaveAttribute('href', '/careers');
		await expect(footer.getByRole('link', { name: 'Terms' })).toHaveAttribute('href', '/terms');
		await expect(footer.getByRole('link', { name: 'Privacy' })).toHaveAttribute('href', '/privacy');
		await expect(footer.getByRole('link', { name: 'Cookies' })).toHaveAttribute('href', '/cookies');
	});

	test('Subject select has expected options', async ({ page }) => {
		await page.goto('/contact');
		const subject = page.getByLabel('Subject');
		await expect(subject).toBeVisible();
		await expect(subject.locator('option')).toHaveCount(5);
		await expect(subject).toHaveValue('general');
	});
});
