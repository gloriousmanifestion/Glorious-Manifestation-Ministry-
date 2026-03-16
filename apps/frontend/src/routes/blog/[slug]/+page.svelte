<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import {
		addBlogComment,
		deleteBlogComment,
		getBlogComments,
		getBlogPostBySlug,
		type BlogComment,
		type BlogPost
	} from '$lib/api/blog';
	import { userStore, authReady } from '$lib/auth';
	import { SITE } from '$lib/config';
	import { pageSeo } from '$lib/stores/pageSeo';

	let post = $state<BlogPost | null>(null);
	let comments = $state<BlogComment[]>([]);
	let loading = $state(true);
	let notFound = $state(false);
	let commentBody = $state('');
	let commentSubmitting = $state(false);
	let commentError = $state('');
	let deletingId = $state<string | null>(null);

	const slug = $derived($page.params.slug);
	const isLoggedIn = $derived($authReady && $userStore != null);
	const canDeleteComment = (c: BlogComment) =>
		$userStore &&
		($userStore.id === c.user_id || $userStore.role === 'admin' || $userStore.role === 'super_admin');

	onMount(load);

	async function load() {
		if (!slug) {
			notFound = true;
			loading = false;
			return;
		}
		try {
			post = await getBlogPostBySlug(slug);
			notFound = !post;
			if (post) {
				try {
					comments = await getBlogComments(slug);
				} catch {
					comments = [];
				}
			}
		} catch {
			notFound = true;
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		if (post) {
			pageSeo.set({
				title: `${post.title} — ${SITE.name} Pastoral School`,
				description: post.body.slice(0, 160).trim() + (post.body.length > 160 ? '…' : '')
			});
		} else if (!loading) {
			pageSeo.set({ title: 'Post not found', description: 'Blog post' });
		}
	});

	async function submitComment() {
		if (!slug || !commentBody.trim() || commentSubmitting || !isLoggedIn) return;
		commentSubmitting = true;
		commentError = '';
		try {
			const newComment = await addBlogComment(slug, commentBody.trim());
			comments = [...comments, newComment];
			commentBody = '';
		} catch (e) {
			commentError = e instanceof Error ? e.message : 'Failed to add comment';
		} finally {
			commentSubmitting = false;
		}
	}

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

		function formatCommentDate(iso: string): string {
		try {
			return new Date(iso).toLocaleString(undefined, {
				month: 'short',
				day: 'numeric',
				year: 'numeric',
				hour: '2-digit',
				minute: '2-digit'
			});
		} catch {
			return iso;
		}
	}

	async function removeComment(commentId: string) {
		if (!slug || deletingId) return;
		deletingId = commentId;
		try {
			await deleteBlogComment(slug, commentId);
			comments = comments.filter((c) => c.id !== commentId);
		} catch {
			// ignore; could show toast
		} finally {
			deletingId = null;
		}
	}
</script>

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

				<!-- Comments -->
				<div class="mt-10 rounded-2xl border border-slate-200/80 bg-white/95 p-6 shadow-sm dark:border-white/[0.08] dark:bg-[#1e2642]">
					<h2 class="text-lg font-semibold text-slate-900 dark:text-white">
						Comments {comments.length ? `(${comments.length})` : ''}
					</h2>
					{#if isLoggedIn}
						<form
							class="mt-4"
							onsubmit={(e) => {
								e.preventDefault();
								submitComment();
							}}
						>
							<textarea
								bind:value={commentBody}
								rows="3"
								placeholder="Write a comment…"
								class="w-full rounded-xl border border-slate-300 bg-white px-3 py-2 text-slate-900 placeholder-slate-500 dark:border-slate-600 dark:bg-slate-800 dark:text-white dark:placeholder-slate-400"
							></textarea>
							<div class="mt-2 flex items-center gap-3">
								<button
									type="submit"
									disabled={commentSubmitting || !commentBody.trim()}
									class="rounded-lg bg-pink-600 px-4 py-2 text-sm font-medium text-white hover:bg-pink-500 disabled:opacity-50"
								>
									{commentSubmitting ? 'Posting…' : 'Post comment'}
								</button>
								{#if commentError}
									<span class="text-sm text-red-600 dark:text-red-400">{commentError}</span>
								{/if}
							</div>
						</form>
					{:else}
						<p class="mt-3 text-sm text-slate-500 dark:text-slate-400">
							<a href="/login" class="text-pink-600 hover:underline dark:text-pink-400">Log in</a> to leave a comment.
						</p>
					{/if}
					<ul class="mt-6 space-y-4">
						{#each comments as c}
							<li class="border-t border-slate-200/80 pt-4 first:border-t-0 first:pt-0 dark:border-slate-700">
								<div class="flex items-start justify-between gap-2">
									<div class="min-w-0 flex-1">
										<p class="font-medium text-slate-900 dark:text-white">{c.author_name}</p>
										<p class="mt-1 text-sm text-slate-600 dark:text-slate-200">{c.body}</p>
										<p class="mt-1 text-xs text-slate-500 dark:text-slate-400">{formatCommentDate(c.created_at)}</p>
									</div>
									{#if isLoggedIn && canDeleteComment(c)}
										<button
											type="button"
											disabled={deletingId === c.id}
											class="shrink-0 rounded px-2 py-1 text-xs text-slate-500 hover:bg-slate-100 hover:text-red-600 dark:hover:bg-slate-700 dark:hover:text-red-400 disabled:opacity-50"
											onclick={() => removeComment(c.id)}
										>
											{deletingId === c.id ? '…' : 'Delete'}
										</button>
									{/if}
								</div>
							</li>
						{/each}
					</ul>
					{#if comments.length === 0 && !isLoggedIn}
						<p class="mt-4 text-sm text-slate-500 dark:text-slate-400">No comments yet.</p>
					{/if}
				</div>
			{/if}
		</div>
	</main>

	<Footer />
</div>
