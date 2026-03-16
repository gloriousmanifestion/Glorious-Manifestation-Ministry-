<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { subscribeNewsletter } from '$lib/api/newsletter';
	import { SITE } from '$lib/config';
	import { pageSeo } from '$lib/stores/pageSeo';

	pageSeo.set({
		title: `Newsletter — ${SITE.name} Pastoral School`,
		description: `Subscribe to the ${SITE.name} Pastoral School newsletter for news, events, and updates.`
	});

	let email = $state('');
	let loading = $state(false);
	let success = $state(false);
	let error = $state('');

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (loading || !email.trim()) return;
		error = '';
		success = false;
		loading = true;
		try {
			await subscribeNewsletter(email);
			success = true;
			email = '';
		} catch (err) {
			error = err instanceof Error ? err.message : 'Subscription failed';
		} finally {
			loading = false;
		}
	}
</script>

<div class="min-h-screen flex flex-col bg-white dark:bg-[#252d47]">
	<Nav />

	<main class="flex-1">
		<section class="px-4 sm:px-6 pt-24 sm:pt-32 pb-16 sm:pb-24">
			<div class="mx-auto max-w-xl text-center">
				<AnimateOnView>
					<h1 class="text-3xl font-semibold tracking-tight text-slate-900 sm:text-4xl dark:text-white">
						Newsletter
					</h1>
					<p class="mt-4 text-slate-600 dark:text-slate-100">
						Subscribe to receive news, event reminders, and updates from {SITE.schoolName}.
					</p>
				</AnimateOnView>

				<AnimateOnView delay={100}>
					{#if success}
						<div
							class="mt-8 rounded-xl border border-green-200 bg-green-50 p-6 text-left dark:border-green-800 dark:bg-green-900/20"
							role="alert"
						>
							<p class="text-green-800 dark:text-green-200">Thank you for subscribing. We'll keep you updated.</p>
						</div>
					{:else}
						<form class="mt-8 space-y-4" onsubmit={handleSubmit}>
							<div>
								<label for="newsletter-email" class="sr-only">Email address</label>
								<input
									id="newsletter-email"
									type="email"
									placeholder="your@email.com"
									bind:value={email}
									disabled={loading}
									required
									class="w-full rounded-lg border border-slate-300 bg-white px-4 py-3 text-slate-900 placeholder-slate-500 focus:border-pink-500 focus:outline-none focus:ring-2 focus:ring-pink-500 dark:border-slate-600 dark:bg-slate-800 dark:text-white dark:placeholder-slate-400"
								/>
							</div>
							{#if error}
								<p class="text-sm text-red-600 dark:text-red-400">{error}</p>
							{/if}
							<button
								type="submit"
								disabled={loading}
								class="w-full rounded-lg bg-pink-600 px-4 py-3 text-sm font-medium text-white transition hover:bg-pink-700 disabled:opacity-50 dark:bg-pink-500 dark:hover:bg-pink-600"
							>
								{loading ? 'Subscribing…' : 'Subscribe'}
							</button>
						</form>
					{/if}
				</AnimateOnView>
			</div>
		</section>
	</main>

	<Footer />
</div>
