<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { getBlogPostBySlug, type BlogPost } from '$lib/api/blog';
	import { SITE } from '$lib/config';

	let post = $state<BlogPost | null>(null);
	let loading = $state(true);
	let notFound = $state(false);

	const slug = $derived($page.params.slug);

	onMount(async () => {
		if (!slug) {
			notFound = true;
			loading = false;
			return;
		}
		try {
			post = await getBlogPostBySlug(slug);
			notFound = !post;
		} catch {
			notFound = true;
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

<svelte:head>
	<title>{post?.title ?? 'Post'} — {SITE.name} Pastoral School</title>
	<meta name="description" content={post ? post.body.slice(0, 160) + '…' : 'Blog post'} />
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1 pt-16 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-3xl px-4 sm:px-6">
			{#if loading}
				<div class="flex justify-center py-12">
					<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
				</div>
			{:else if notFound || !post}
				<div class="rounded-2xl border border-slate-200/80 bg-white/95 p-8 text-center dark:border-white/10 dark:bg-[#1e2642]">
					<h1 class="text-xl font-semibold text-slate-900 dark:text-white">Post not found</h1>
					<a href="/blog" class="mt-4 inline-block text-pink-600 hover:text-pink-700 dark:text-pink-400">
						← Back to Blog
					</a>
				</div>
			{:else}
				<article class="rounded-2xl border border-slate-200/80 section-separator card-premium bg-white/95 p-6 sm:p-8 shadow-sm dark:bg-[#1e2642] dark:border-white/[0.08]">
					<a href="/blog" class="text-sm font-medium text-slate-500 hover:text-pink-600 dark:text-slate-400 dark:hover:text-pink-400">
						← Back to Blog
					</a>
					<h1 class="mt-4 text-3xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-4xl">
						{post.title}
					</h1>
					<p class="mt-2 text-slate-500 dark:text-slate-400">
						{post.author_name} · {formatDate(post.published_at)}
					</p>
					{#if post.image_url}
						<img
							src={post.image_url}
							alt=""
							class="mt-6 w-full rounded-xl object-cover"
						/>
					{/if}
					<div class="mt-6 whitespace-pre-wrap text-slate-700 dark:text-slate-200">
						{post.body}
					</div>
				</article>
			{/if}
		</div>
	</main>

	<Footer />
</div>
