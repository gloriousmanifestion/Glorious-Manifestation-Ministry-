<script lang="ts">
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { getBlogPosts, type BlogPost } from '$lib/api/blog';
	import { SITE } from '$lib/config';
	import EmptySection from '$lib/components/EmptySection.svelte';
	import { pageSeo } from '$lib/stores/pageSeo';

	pageSeo.set({
		title: `Blog — ${SITE.name} Pastoral School`,
		description: 'News and updates from the Pastoral School.'
	});

	let items = $state<BlogPost[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			items = await getBlogPosts();
		} catch {
			items = [];
		} finally {
			loading = false;
		}
	});

	function formatDate(iso: string | null): string {
		if (!iso) return '';
		try {
			return new Date(iso).toLocaleDateString(undefined, {
				month: 'long',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return iso;
		}
	}
</script>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1 pt-16 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-4xl px-4 sm:px-6">
			<h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-4xl">
				Blog
			</h1>
			<p class="mt-2 text-slate-600 dark:text-slate-200">
				News and updates from the Pastoral School.
			</p>

			{#if loading}
				<div class="mt-12 flex justify-center">
					<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
				</div>
			{:else if items.length === 0}
				<div class="mt-12">
					<EmptySection type="blog" />
				</div>
			{:else}
				<div class="mt-10 space-y-6">
					{#each items as post}
						<article
							class="rounded-2xl border border-slate-200/80 section-separator card-premium bg-white/95 p-6 shadow-sm dark:bg-[#1e2642] dark:border-white/[0.08]"
						>
							<a href="/blog/{post.slug}" class="block">
								<h2 class="text-xl font-semibold text-slate-900 dark:text-white hover:text-pink-600 dark:hover:text-pink-400">
									{post.title}
								</h2>
							</a>
							<p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
								{post.author_name} · {formatDate(post.published_at)}
							</p>
							<p class="mt-3 line-clamp-3 text-slate-600 dark:text-slate-200">
								{post.body.slice(0, 200)}
								{post.body.length > 200 ? '…' : ''}
							</p>
							<a
								href="/blog/{post.slug}"
								class="mt-3 inline-block text-sm font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400 dark:hover:text-pink-300"
							>
								Read more →
							</a>
						</article>
					{/each}
				</div>
			{/if}
		</div>
	</main>

	<Footer />
</div>
