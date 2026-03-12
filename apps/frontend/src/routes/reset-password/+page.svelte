<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { SITE } from '$lib/config';
	import { auth } from '$lib/auth';

	const token = $derived($page.url.searchParams.get('token') ?? '');

	let password = $state('');
	let confirmPassword = $state('');
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (loading) return;
		error = '';
		success = '';

		if (!token) {
			error = 'Invalid or missing reset token. Please use the link from your email.';
			return;
		}

		if (password.length < 8) {
			error = 'Password must be at least 8 characters long.';
			return;
		}
		if (password !== confirmPassword) {
			error = 'Passwords do not match.';
			return;
		}

		loading = true;
		try {
			await auth.resetPassword(token, password);
			success = 'Your password has been reset successfully. You can now log in with your new password.';
			setTimeout(() => {
				goto('/login');
			}, 1500);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not reset password. Please try again.';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Reset password — {SITE.name} Pastoral School</title>
	<meta name="description" content="Choose a new password for your Pastoral School account." />
</svelte:head>

<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
	<Nav />

	<main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-md px-4 sm:px-6">
			<div
				class="rounded-2xl border border-pink-200/50 card-premium bg-white p-6 sm:p-8 shadow-xl dark:bg-[#1e2642]"
			>
				<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">
					Reset your password
				</h1>
				<p class="mt-2 text-sm text-slate-600 dark:text-slate-100">
					Choose a new password for your account.
				</p>

				<form class="mt-8 space-y-6" onsubmit={handleSubmit}>
					{#if error}
						<div
							class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700 dark:border-red-800 dark:bg-red-900/30 dark:text-red-200"
						>
							{error}
						</div>
					{/if}
					{#if success}
						<div
							class="rounded-xl border border-emerald-200 bg-emerald-50 p-4 text-sm text-emerald-700 dark:border-emerald-800 dark:bg-emerald-900/30 dark:text-emerald-200"
						>
							{success}
						</div>
					{/if}

					<div>
						<label
							for="password"
							class="block text-sm font-medium text-slate-700 dark:text-slate-100"
						>
							New password
						</label>
						<input
							id="password"
							type="password"
							bind:value={password}
							required
							minlength={8}
							class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
							placeholder="Enter a new password"
						/>
					</div>

					<div>
						<label
							for="confirm-password"
							class="block text-sm font-medium text-slate-700 dark:text-slate-100"
						>
							Confirm new password
						</label>
						<input
							id="confirm-password"
							type="password"
							bind:value={confirmPassword}
							required
							minlength={8}
							class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
							placeholder="Re-enter your new password"
						/>
					</div>

					<button
						type="submit"
						disabled={loading}
						class="w-full rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 disabled:opacity-60"
					>
						{loading ? 'Updating password…' : 'Reset password'}
					</button>
				</form>

				<p class="mt-6 text-center text-sm text-slate-600 dark:text-slate-200">
					<a
						href="/login"
						class="font-medium text-pink-600 hover:text-pink-500 dark:text-pink-400 dark:hover:text-pink-300"
					>
						Back to login
					</a>
				</p>
			</div>
		</div>
	</main>

	<Footer />
</div>

