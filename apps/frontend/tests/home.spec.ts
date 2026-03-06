import { test, expect } from '@playwright/test';
import { PLUS_STAGING, BUSINESS_STAGING } from './helpers';

test.describe('Home page', () => {
	test('page loads with 1nga Solutions branding', async ({ page }) => {
		await page.goto('/');
		await expect(page).toHaveTitle(/1nga Solutions/);
		await expect(page.getByText('1nga Solutions').first()).toBeVisible();
		await expect(page.getByText('Digital transformation Hub').first()).toBeVisible();
	});

	test('Sign in link points to staging Plus URL', async ({ page }) => {
		await page.goto('/');
		const signIn = page.getByRole('link', { name: 'Sign in', exact: true });
		await expect(signIn).toBeVisible();
		await expect(signIn).toHaveAttribute('href', PLUS_STAGING);
	});

	test('Get started link (hero) points to staging Plus URL', async ({ page }) => {
		await page.goto('/');
		const getStarted = page.getByRole('link', { name: 'Get started', exact: true });
		await expect(getStarted).toBeVisible();
		await expect(getStarted).toHaveAttribute('href', PLUS_STAGING);
	});

	test('Get started at 1nga Plus CTA points to staging Plus URL', async ({ page }) => {
		await page.goto('/');
		const cta = page.getByRole('link', { name: 'Get started at 1nga Plus' });
		await expect(cta).toBeVisible();
		await expect(cta).toHaveAttribute('href', PLUS_STAGING);
	});

	test('Partner with us CTA points to staging Business URL', async ({ page }) => {
		await page.goto('/');
		const cta = page.getByRole('link', { name: 'Partner with us' });
		await expect(cta).toBeVisible();
		await expect(cta).toHaveAttribute('href', BUSINESS_STAGING);
	});

	test('footer has Contact, Careers, Terms, Privacy, and Cookies links', async ({ page }) => {
		await page.goto('/');
		const footer = page.locator('footer');
		await expect(footer.getByRole('link', { name: 'Contact' })).toHaveAttribute('href', '/contact');
		await expect(footer.getByRole('link', { name: 'Careers' })).toHaveAttribute('href', '/careers');
		await expect(footer.getByRole('link', { name: 'Terms' })).toHaveAttribute('href', '/terms');
		await expect(footer.getByRole('link', { name: 'Privacy' })).toHaveAttribute('href', '/privacy');
		await expect(footer.getByRole('link', { name: 'Cookies' })).toHaveAttribute('href', '/cookies');
	});

	test('Our solutions link scrolls to solutions section', async ({ page }) => {
		await page.goto('/');
		const ourSolutions = page.getByRole('link', { name: 'Our solutions' });
		await expect(ourSolutions).toBeVisible();
		await expect(ourSolutions).toHaveAttribute('href', '#solutions');
	});

	test('footer Careers link navigates to careers page', async ({ page }) => {
		await page.goto('/');
		await page.locator('footer').getByRole('link', { name: 'Careers' }).click();
		await expect(page).toHaveURL(/\/careers/);
		await expect(page.getByRole('heading', { name: 'Build the future with us' })).toBeVisible();
	});
});
