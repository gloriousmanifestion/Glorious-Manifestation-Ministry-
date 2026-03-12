import { test, expect } from '@playwright/test';

test.describe('Password reset flow (UI)', () => {
	test('login page links to forgot password', async ({ page }) => {
		await page.goto('/login');

		const forgotLink = page.getByRole('link', { name: 'Forgot your password?' });
		await expect(forgotLink).toBeVisible();
		await expect(forgotLink).toHaveAttribute('href', '/forgot-password');
	});

	test('forgot password page renders email form', async ({ page }) => {
		await page.goto('/forgot-password');

		await expect(page.getByRole('heading', { name: 'Forgot your password?' })).toBeVisible();
		const emailInput = page.getByLabel('Email');
		await expect(emailInput).toBeVisible();
	});

	test('reset password page renders password fields', async ({ page }) => {
		// Use a dummy token; we only verify UI renders and fields exist.
		await page.goto('/reset-password?token=dummy-token');

		await expect(page.getByRole('heading', { name: 'Reset your password' })).toBeVisible();
		await expect(page.getByLabel('New password', { exact: true }).first()).toBeVisible();
		await expect(page.getByLabel('Confirm new password', { exact: true }).first()).toBeVisible();
	});
});
