<script lang="ts">
	import { onMount } from 'svelte';
	import {
		listTestimonialsAdmin,
		createTestimonial,
		updateTestimonial,
		deleteTestimonial,
		type Testimonial
	} from '$lib/api/testimonials';
	import { SITE } from '$lib/config';

	let items = $state<Testimonial[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let modalOpen = $state(false);
	let editId = $state<string | null>(null);
	let form = $state({ author_name: '', role_or_title: '', body: '', image_url: '', sort_order: 0 });

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			items = await listTestimonialsAdmin();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load testimonials';
			items = [];
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editId = null;
		form = { author_name: '', role_or_title: '', body: '', image_url: '', sort_order: 0 };
		modalOpen = true;
	}

	function openEdit(t: Testimonial) {
		editId = t.id;
		form = {
			author_name: t.author_name,
			role_or_title: t.role_or_title ?? '',
			body: t.body,
			image_url: t.image_url ?? '',
			sort_order: t.sort_order
		};
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
		editId = null;
	}

	async function saveItem() {
		if (!form.author_name.trim()) {
			error = 'Author name is required';
			return;
		}
		if (!form.body.trim()) {
			error = 'Body is required';
			return;
		}
		saving = true;
		error = '';
		try {
			if (editId) {
				const updated = await updateTestimonial(editId, {
					author_name: form.author_name.trim(),
					role_or_title: form.role_or_title.trim() || null,
					body: form.body.trim(),
					image_url: form.image_url.trim() || null,
					sort_order: form.sort_order
				});
				items = items.map((t) => (t.id === editId ? updated : t));
			} else {
				const created = await createTestimonial({
					author_name: form.author_name.trim(),
					role_or_title: form.role_or_title.trim() || null,
					body: form.body.trim(),
					image_url: form.image_url.trim() || null,
					sort_order: form.sort_order
				});
				items = [...items, created];
			}
			closeModal();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save';
		} finally {
			saving = false;
		}
	}

	async function remove(t: Testimonial) {
		if (!confirm('Delete this testimonial?')) return;
		saving = true;
		error = '';
		try {
			await deleteTestimonial(t.id);
			items = items.filter((x) => x.id !== t.id);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>Testimonials — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-4xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold text-white">Testimonials</h1>
			<p class="mt-1 text-sm text-slate-400">
				Manage student and alumni testimonials shown on the public testimonials page.
			</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center justify-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			Add testimonial
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
			No testimonials yet. Add one to show on the public
			<a href="/testimonials" class="text-pink-400 hover:underline">testimonials page</a>.
		</div>
	{:else}
		<div class="space-y-3">
			{#each items as item}
				<div class="flex items-start justify-between gap-4 rounded-xl border border-slate-700 bg-slate-800/50 p-4">
					<div class="min-w-0 flex-1">
						<h3 class="font-medium text-white">{item.author_name}</h3>
						{#if item.role_or_title}
							<p class="text-sm text-slate-400">{item.role_or_title}</p>
						{/if}
						<p class="mt-1 line-clamp-2 whitespace-pre-wrap text-sm text-slate-500">{item.body}</p>
					</div>
					<div class="flex shrink-0 gap-2">
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
				{editId ? 'Edit testimonial' : 'Add testimonial'}
			</h2>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					saveItem();
				}}
				class="mt-4 space-y-4"
			>
				<div>
					<label for="t-author" class="block text-sm font-medium text-slate-300">Author name</label>
					<input
						id="t-author"
						type="text"
						bind:value={form.author_name}
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="e.g. John Doe"
					/>
				</div>
				<div>
					<label for="t-role" class="block text-sm font-medium text-slate-300">Role or title</label>
					<input
						id="t-role"
						type="text"
						bind:value={form.role_or_title}
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="e.g. Graduate 2024"
					/>
				</div>
				<div>
					<label for="t-body" class="block text-sm font-medium text-slate-300">Quote / body</label>
					<textarea
						id="t-body"
						bind:value={form.body}
						rows="4"
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="Testimonial text"
					></textarea>
				</div>
				<div>
					<label for="t-image" class="block text-sm font-medium text-slate-300">Image URL (optional)</label>
					<input
						id="t-image"
						type="text"
						bind:value={form.image_url}
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="https://..."
					/>
				</div>
				<div>
					<label for="t-sort" class="block text-sm font-medium text-slate-300">Sort order</label>
					<input
						id="t-sort"
						type="number"
						bind:value={form.sort_order}
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
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
