<script lang="ts">
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { getEvents } from '$lib/api/events';
	import type { Event } from '$lib/api/events';
	import { SITE } from '$lib/config';

	let events = $state<Event[]>([]);
	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		try {
			events = await getEvents();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load events';
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Events — {SITE.name} Pastoral School</title>
	<meta
		name="description"
		content="Upcoming events at Prophetic Glorious Manifestation Pastoral School. School opening, revivals, graduation, and weekly chapel in Owerri."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1">
		<section class="gradient-mesh px-4 sm:px-6 pt-24 sm:pt-28 pb-12 sm:pb-16 text-center">
			<p class="hero-enter-delay-1 mb-4 text-sm font-medium uppercase tracking-[0.2em] text-pink-600">Calendar</p>
			<h1 class="hero-enter-delay-2 mx-auto max-w-3xl text-3xl font-bold tracking-tight text-slate-900 sm:text-4xl md:text-5xl">
				Events
			</h1>
			<p class="hero-enter-delay-3 mx-auto mt-6 max-w-2xl text-lg text-slate-600">
				Stay updated with school activities, revivals, and special programs.
			</p>
		</section>

		<section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6">
			<div class="mx-auto max-w-3xl">
				{#if loading}
					<div class="flex justify-center py-12">
						<div class="h-8 w-8 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
					</div>
				{:else if error}
					<div class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700">
						{error}
					</div>
				{:else}
					<div class="space-y-4 sm:space-y-6">
						{#each events as event, i}
							<AnimateOnView animation="fade-up" stagger={i}>
								<article
									class="hover-lift flex flex-col sm:flex-row gap-4 sm:gap-6 rounded-2xl border border-pink-200/50 bg-white p-4 sm:p-6 shadow-sm hover:border-pink-300 hover:shadow-lg"
								>
									<div
										class="flex h-16 w-16 shrink-0 flex-col items-center justify-center rounded-xl bg-gradient-to-br from-pink-500 to-purple-600 text-center text-white"
									>
										<span class="text-xs font-medium uppercase leading-tight">{event.date}</span>
									</div>
									<div>
										<h2 class="text-lg font-bold text-slate-900">{event.title}</h2>
										<p class="mt-2 text-slate-600">{event.description ?? ''}</p>
									</div>
								</article>
							</AnimateOnView>
						{/each}
					</div>
				{/if}
			</div>
		</section>

		<section class="border-t border-pink-200/50 py-12 sm:py-16 px-4 sm:px-6">
			<AnimateOnView animation="scale">
				<div class="mx-auto max-w-2xl text-center">
					<p class="text-slate-600">For the latest event updates, follow us and contact via WhatsApp.</p>
					<a
						href="https://wa.me/2348026954759"
						target="_blank"
						rel="noopener noreferrer"
						class="btn-scale mt-6 inline-flex items-center gap-2 rounded-full bg-[#25d366] px-6 py-3 text-sm font-semibold text-white transition hover:bg-[#20bd5a]"
					>
					<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
						<path
							d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"
						/>
					</svg>
					Get updates on WhatsApp
				</a>
				</div>
			</AnimateOnView>
		</section>
	</main>

	<Footer />
</div>
