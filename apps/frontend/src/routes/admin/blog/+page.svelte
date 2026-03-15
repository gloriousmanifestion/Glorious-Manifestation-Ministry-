<script lang="ts">
	import { onMount } from 'svelte';
	import {
		listBlogPostsAdmin,
		createBlogPost,
		updateBlogPost,
		deleteBlogPost,
		type BlogPost
	} from '$lib/api/blog';
	import { SITE } from '$lib/config';

	let items = $state<BlogPost[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let modalOpen = $state(false);
	let editId = $state<string | null>(null);
	let form = $state({
		title: '',
		slug: '',
		body: '',
		author_name: '',
		image_url: '',
		published_at: ''
	});

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			items = await listBlogPostsAdmin();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load posts';
			items = [];
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editId = null;
		form = {
			title: '',
			slug: '',
			body: '',
			author_name: '',
			image_url: '',
			published_at: ''
		};
		modalOpen = true;
	}

	function openEdit(p: BlogPost) {
		editId = p.id;
		form = {
			title: p.title,
			slug: p.slug,
			body: p.body,
			author_name: p.author_name,
			image_url: p.image_url ?? '',
			published_at: p.published_at ?? ''
		};
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
		editId = null;
	}

	async function saveItem() {
		if (!form.title.trim()) {
			error = 'Title is required';
			return;
		}
		if (!form.body.trim()) {
			error = 'Body is required';
			return;
		}
		if (!form.author_name.trim()) {
			error = 'Author name is required';
			return;
		}
		saving = true;
		error = '';
		try {
			if (editId) {
				const updated = await updateBlogPost(editId, {
					title: form.title.trim(),
					slug: form.slug.trim() || undefined,
					body: form.body.trim(),
					author_name: form.author_name.trim(),
					image_url: form.image_url.trim() || null,
					published_at: form.published_at.trim() || null
				});
				items = items.map((p) => (p.id === editId ? updated : p));
			} else {
				const created = await createBlogPost({
					title: form.title.trim(),
					slug: form.slug.trim() || undefined,
					body: form.body.trim(),
					author_name: form.author_name.trim(),
					image_url: form.image_url.trim() || null,
					published_at: form.published_at.trim() || null
				});
				items = [created, ...items];
			}
			closeModal();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save';
		} finally {
			saving = false;
		}
	}

	async function remove(p: BlogPost) {
		if (!confirm('Delete this post?')) return;
		saving = true;
		error = '';
		try {
			await deleteBlogPost(p.id);
			items = items.filter((x) => x.id !== p.id);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete';
		} finally {
			saving = false;
		}
	}

	function formatDate(iso: string | null): string {
		if (!iso) return 'Draft';
		try {
			return new Date(iso).toLocaleDateString(undefined, {
				month: 'short',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return iso;
		}
	}
</script>

<svelte:head>
	<title>Blog — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-4xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold text-white">Blog</h1>
			<p class="mt-1 text-sm text-slate-400">
				Manage news and blog posts. Set published_at to publish; leave empty for draft.
			</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center justify-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			Add post
		</button>
	</header>

	{#if error}
		<div class="mb-4 rounded-lg border border-red-500/50 bg-red-500/10 px-4 py-3 text-sm text-red-300">
			{error}
		</div>
	{/if}

	{#if loading}
		<div class="flex items-center justify-center py-16">
			<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
		</div>
	{:else if items.length === 0}
		<div class="rounded-xl border border-slate-700 bg-slate-800/50 p-8 text-center text-slate-400">
			No posts yet. Add one to show on the public <a href="/blog" class="text-pink-400 hover:underline">blog</a>.
		</div>
	{:else}
		<div class="space-y-3">
			{#each items as item}
				<div class="flex items-start justify-between gap-4 rounded-xl border border-slate-700 bg-slate-800/50 p-4">
					<div class="min-w-0 flex-1">
						<h3 class="font-medium text-white">{item.title}</h3>
						<p class="mt-1 text-sm text-slate-400">
							{item.author_name} · {formatDate(item.published_at)} · /blog/{item.slug}
						</p>
						<p class="mt-1 line-clamp-2 text-sm text-slate-500">{item.body.slice(0, 120)}…</p>
					</div>
					<div class="flex shrink-0 gap-2">
						<a
							href="/blog/{item.slug}"
							target="_blank"
							rel="noopener noreferrer"
							class="rounded-lg border border-slate-600 px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-700"
						>
							View
						</a>
						<button
							type="button"
							onclick={() => openEdit(item)}
							class="rounded-lg border border-slate-600 px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-700"
						>
							Edit
						</button>
						<button
							type="button"
							onclick={() => remove(item)}
							class="rounded-lg border border-red-500/50 px-3 py-1.5 text-sm text-red-400 hover:bg-red-500/10"
						>
							Remove
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

{#if modalOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onkeydown={(ev) => ev.key === 'Escape' && closeModal()}
	>
		<button
			type="button"
			class="absolute inset-0 -z-10 cursor-default"
			aria-label="Close modal"
			onclick={closeModal}
			onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && closeModal()}
		></button>
		<div class="relative z-10 max-h-[90vh] w-full max-w-lg overflow-y-auto rounded-xl border border-slate-700 bg-slate-900 p-6 shadow-xl">
			<h2 class="text-lg font-bold text-white">
				{editId ? 'Edit post' : 'Add post'}
			</h2>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					saveItem();
				}}
				class="mt-4 space-y-4"
			>
				<div>
					<label for="b-title" class="block text-sm font-medium text-slate-300">Title</label>
					<input
						id="b-title"
						type="text"
						bind:value={form.title}
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="Post title"
					/>
				</div>
				<div>
					<label for="b-slug" class="block text-sm font-medium text-slate-300">Slug (URL)</label>
					<input
						id="b-slug"
						type="text"
						bind:value={form.slug}
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="my-post-slug (leave empty to auto-generate from title)"
					/>
				</div>
				<div>
					<label for="b-body" class="block text-sm font-medium text-slate-300">Body</label>
					<textarea
						id="b-body"
						bind:value={form.body}
						rows="6"
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="Post content (plain text)"
					></textarea>
				</div>
				<div>
					<label for="b-author" class="block text-sm font-medium text-slate-300">Author name</label>
					<input
						id="b-author"
						type="text"
						bind:value={form.author_name}
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="Author"
					/>
				</div>
				<div>
					<label for="b-image" class="block text-sm font-medium text-slate-300">Image URL (optional)</label>
					<input
						id="b-image"
						type="text"
						bind:value={form.image_url}
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="https://..."
					/>
				</div>
				<div>
					<label for="b-published" class="block text-sm font-medium text-slate-300">Published at (ISO date or empty = draft)</label>
					<input
						id="b-published"
						type="text"
						bind:value={form.published_at}
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="2025-03-15T12:00:00Z or leave empty"
					/>
				</div>
				<div class="flex justify-end gap-2 pt-4">
					<button
						type="button"
						onclick={closeModal}
						class="rounded-lg border border-slate-600 px-4 py-2 text-sm text-slate-300 hover:bg-slate-800"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={saving}
						class="rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white hover:bg-pink-500 disabled:opacity-50"
					>
						{saving ? 'Saving…' : 'Save'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
