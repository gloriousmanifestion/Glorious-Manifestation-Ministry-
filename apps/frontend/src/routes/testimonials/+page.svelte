<script lang="ts">
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { getTestimonials, type Testimonial } from '$lib/api/testimonials';
	import { SITE } from '$lib/config';
	import EmptySection from '$lib/components/EmptySection.svelte';

	let items = $state<Testimonial[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			items = await getTestimonials();
		} catch {
			items = [];
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Testimonials — {SITE.name} Pastoral School</title>
	<meta
		name="description"
		content="What students and alumni say about the Pastoral School."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1 pt-16 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-4xl px-4 sm:px-6">
			<h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-4xl">
				Testimonials
			</h1>
			<p class="mt-2 text-slate-600 dark:text-slate-200">
				What our students and alumni say about the Pastoral School.
			</p>

			{#if loading}
				<div class="mt-12 flex justify-center">
					<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
				</div>
			{:else if items.length === 0}
				<div class="mt-12">
					<EmptySection type="testimonials" />
				</div>
			{:else}
				<div class="mt-10 space-y-6">
					{#each items as item}
						<blockquote
							class="rounded-2xl border border-slate-200/80 section-separator card-premium bg-white/95 p-6 shadow-sm dark:bg-[#1e2642] dark:border-white/[0.08]"
						>
							<p class="whitespace-pre-wrap text-slate-700 dark:text-slate-200">"{item.body}"</p>
							<footer class="mt-4 flex items-center gap-3">
								{#if item.image_url}
									<img
										src={item.image_url}
										alt=""
										class="h-12 w-12 rounded-full object-cover"
									/>
								{/if}
								<div>
									<cite class="font-semibold not-italic text-slate-900 dark:text-white">
										{item.author_name}
									</cite>
									{#if item.role_or_title}
										<p class="text-sm text-slate-500 dark:text-slate-400">{item.role_or_title}</p>
									{/if}
								</div>
							</footer>
						</blockquote>
					{/each}
				</div>
			{/if}
		</div>
	</main>

	<Footer />
</div>
