<script lang="ts">
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { getChurchContent } from '$lib/api/church';
	import { SITE, getDefaultFaqs } from '$lib/config';
	import EmptySection from '$lib/components/EmptySection.svelte';
	import { pageSeo } from '$lib/stores/pageSeo';

	pageSeo.set({
		title: `FAQ — ${SITE.name} Pastoral School`,
		description: 'Frequently asked questions about the Pastoral School: admissions, courses, payments, and more.'
	});

	interface FaqItem {
		q: string;
		a: string;
	}

	let items = $state<FaqItem[]>([]);
	let loading = $state(true);
	let openIndex = $state<number | null>(null);

	onMount(async () => {
		try {
			const raw = await getChurchContent('faq');
			if (raw) {
				const parsed = JSON.parse(raw) as unknown;
				const fromApi = Array.isArray(parsed)
					? (parsed as FaqItem[]).filter((x) => x && typeof x.q === 'string' && typeof x.a === 'string')
					: [];
				items = fromApi.length > 0 ? fromApi : getDefaultFaqs();
			} else {
				items = getDefaultFaqs();
			}
		} catch {
			items = getDefaultFaqs();
		} finally {
			loading = false;
		}
	});

	function toggle(i: number) {
		openIndex = openIndex === i ? null : i;
	}
</script>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1">
		<section class="gradient-mesh px-4 sm:px-6 pt-24 sm:pt-28 pb-12 sm:pb-16 text-center">
			<p class="hero-enter-delay-1 mb-4 text-sm font-medium uppercase tracking-[0.2em] text-pink-600 dark:text-pink-400">
				Support
			</p>
			<h1 class="hero-enter-delay-2 mx-auto max-w-3xl text-3xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-4xl md:text-5xl">
				Frequently asked questions
			</h1>
			<p class="hero-enter-delay-3 mx-auto mt-6 max-w-2xl text-lg text-slate-600 dark:text-slate-100">
				Quick answers about admissions, courses, and the school.
			</p>
		</section>

		<section class="border-t border-pink-200/50 section-separator bg-white/60 py-12 sm:py-16 px-4 sm:px-6 dark:bg-[#252d47]/80">
			<div class="mx-auto max-w-3xl">
				{#if loading}
					<div class="flex justify-center py-12">
						<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
					</div>
				{:else if items.length === 0}
					<AnimateOnView animation="fade-up">
						<EmptySection type="faq" />
					</AnimateOnView>
				{:else}
					<div class="space-y-2">
						{#each items as item, i}
							<AnimateOnView animation="fade-up" stagger={i}>
								<div
									class="rounded-xl border border-pink-200/50 bg-white dark:bg-[#1e2642] dark:border-pink-500/30 overflow-hidden"
								>
									<button
										type="button"
										class="flex w-full items-center justify-between gap-4 px-4 py-4 text-left font-medium text-slate-900 dark:text-white hover:bg-slate-50 dark:hover:bg-white/5 transition"
										onclick={() => toggle(i)}
										aria-expanded={openIndex === i}
										aria-controls="faq-answer-{i}"
										id="faq-question-{i}"
									>
										<span>{item.q}</span>
										<span
											class="shrink-0 text-pink-500 transition {openIndex === i ? 'rotate-180' : ''}"
											aria-hidden="true"
										>
											<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
											</svg>
										</span>
									</button>
									<div
										id="faq-answer-{i}"
										role="region"
										aria-labelledby="faq-question-{i}"
										class="border-t border-pink-200/30 dark:border-pink-500/20"
									>
										{#if openIndex === i}
											<div class="px-4 py-3 text-slate-600 dark:text-slate-300 whitespace-pre-wrap">
												{item.a}
											</div>
										{/if}
									</div>
								</div>
							</AnimateOnView>
						{/each}
					</div>
				{/if}
			</div>
		</section>

		<section class="border-t border-pink-200/50 section-separator bg-slate-50/80 py-12 px-4 sm:px-6 dark:bg-[#1e2642]/60">
			<div class="mx-auto max-w-2xl text-center">
				<p class="text-slate-600 dark:text-slate-300">
					Still have questions? <a href="/contact" class="font-medium text-pink-600 hover:underline dark:text-pink-400">Get in touch</a>.
				</p>
			</div>
		</section>
	</main>

	<Footer />
</div>
