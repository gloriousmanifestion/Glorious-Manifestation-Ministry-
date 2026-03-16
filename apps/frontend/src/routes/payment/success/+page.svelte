<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { auth } from '$lib/auth';

	let status = $state<'checking' | 'confirmed' | 'error'>('checking');
	let message = $state('Confirming your payment…');

	async function confirm() {
		try {
			const url = $page.url;
			const ref = url.searchParams.get('ref') ?? url.searchParams.get('reference');
			await auth.markPaymentSuccess(ref);
			status = 'confirmed';
			message = 'Your payment has been recorded. Welcome to the Pastoral School!';
			setTimeout(() => goto('/dashboard'), 1500);
		} catch (e) {
			status = 'error';
			message =
				e instanceof Error
					? e.message
					: 'Could not confirm payment. Please try again or contact support.';
		}
	}

	onMount(() => {
		// Optionally read Selar query params (reference, email) for logging/debugging
		confirm();
	});
</script>

<Nav />

<main class="min-h-[60vh] bg-slate-50 px-4 pt-24 pb-12 dark:bg-[#252d47]">
	<div class="mx-auto max-w-xl rounded-2xl border border-slate-200/80 bg-white/95 p-6 text-center shadow-sm dark:border-white/[0.08] dark:bg-[#1e2642]">
		<h1 class="text-xl font-semibold text-slate-900 dark:text-white">
			{#if status === 'confirmed'}
				Payment confirmed
			{:else if status === 'error'}
				Unable to confirm payment
			{:else}
				Confirming payment…
			{/if}
		</h1>
		<p class="mt-2 text-sm text-slate-600 dark:text-slate-200">
			{message}
		</p>

		{#if status === 'checking'}
			<div class="mt-4 flex justify-center">
				<div class="h-6 w-6 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
			</div>
		{/if}

		{#if status === 'error'}
			<div class="mt-4 space-y-2 text-xs text-slate-500 dark:text-slate-400">
				<p>You can still continue to your dashboard and check your enrollment status there.</p>
			</div>
		{/if}

		<div class="mt-6 flex items-center justify-center gap-3">
			<a
				href="/dashboard"
				class="inline-flex items-center justify-center rounded-xl bg-pink-600 px-4 py-2 text-sm font-semibold text-white hover:bg-pink-700"
			>
				Go to dashboard
			</a>
			<button
				type="button"
				onclick={confirm}
				class="inline-flex items-center justify-center rounded-xl border border-slate-200 px-4 py-2 text-sm font-semibold text-slate-700 hover:bg-slate-50 dark:border-slate-600 dark:text-slate-100 dark:hover:bg-slate-800"
			>
				Refresh status
			</button>
		</div>
	</div>
</main>

<Footer />

