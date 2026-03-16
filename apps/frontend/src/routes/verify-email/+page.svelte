<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { auth } from '$lib/auth';
	import { SITE } from '$lib/config';

	let status: 'loading' | 'success' | 'error' = 'loading';
	let message = 'Verifying your email…';

	onMount(async () => {
		const url = $page.url;
		const token = url.searchParams.get('token') ?? '';
		if (!token) {
			status = 'error';
			message = 'Invalid verification link.';
			return;
		}
		try {
			await auth.confirmEmailVerification(token);
			status = 'success';
			message = 'Your email has been verified and you are now signed in.';
			// Redirect to dashboard after a short delay.
			setTimeout(() => {
				goto('/dashboard');
			}, 1500);
		} catch (e) {
			status = 'error';
			message =
				e instanceof Error ? e.message : 'Verification failed. Your link may be expired or invalid.';
		}
	});
</script>

<Nav />

<main class="min-h-[60vh] bg-slate-50 px-4 pt-24 pb-12 dark:bg-[#252d47]">
	<div class="mx-auto max-w-xl rounded-2xl border border-slate-200/80 bg-white/95 p-6 text-center shadow-sm dark:border-white/[0.08] dark:bg-[#1e2642]">
		<h1 class="text-xl font-semibold text-slate-900 dark:text-white">
			{#if status === 'loading'}
				Verifying your email…
			{:else if status === 'success'}
				Email verified
			{:else}
				Verification failed
			{/if}
		</h1>
		<p class="mt-2 text-sm text-slate-600 dark:text-slate-200">
			{message}
		</p>
		{#if status === 'success'}
			<p class="mt-3 text-xs text-slate-500 dark:text-slate-400">
				Redirecting you to your dashboard at {SITE.name} Pastoral School…
			</p>
		{:else if status === 'error'}
			<a
				href="/dashboard"
				class="mt-4 inline-flex items-center justify-center rounded-xl bg-pink-600 px-4 py-2 text-sm font-semibold text-white hover:bg-pink-700"
			>
				Go to dashboard
			</a>
		{/if}
	</div>
</main>

<Footer />

