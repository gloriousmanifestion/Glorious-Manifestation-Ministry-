<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getAdminResources,
		createResource,
		updateResource,
		deleteResource,
		getAdminCourses,
		type AdminResource
	} from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let items = $state<AdminResource[]>([]);
	let courses = $state<{ id: string; name: string }[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let modalOpen = $state(false);
	let editId = $state<string | null>(null);
	let form = $state({
		title: '',
		url: '',
		resource_type: 'link',
		course_id: '' as string | null,
		sort_order: 0
	});

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			[items, courses] = await Promise.all([getAdminResources(), getAdminCourses()]);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load resources';
			items = [];
			courses = [];
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editId = null;
		form = { title: '', url: '', resource_type: 'link', course_id: null, sort_order: 0 };
		modalOpen = true;
	}

	function openEdit(r: AdminResource) {
		editId = r.id;
		form = {
			title: r.title,
			url: r.url,
			resource_type: r.resource_type,
			course_id: r.course_id ?? null,
			sort_order: r.sort_order
		};
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
		editId = null;
	}

	async function saveItem() {
		if (!form.title.trim() || !form.url.trim()) {
			error = 'Title and URL are required';
			return;
		}
		saving = true;
		error = '';
		try {
			if (editId) {
				const updated = await updateResource(editId, {
					title: form.title.trim(),
					url: form.url.trim(),
					resource_type: form.resource_type,
					course_id: form.course_id || null,
					sort_order: form.sort_order
				});
				items = items.map((r) => (r.id === editId ? updated : r));
			} else {
				const created = await createResource({
					title: form.title.trim(),
					url: form.url.trim(),
					resource_type: form.resource_type,
					course_id: form.course_id || null,
					sort_order: form.sort_order
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

	async function remove(r: AdminResource) {
		if (!confirm('Delete this resource?')) return;
		saving = true;
		error = '';
		try {
			await deleteResource(r.id);
			items = items.filter((x) => x.id !== r.id);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete';
		} finally {
			saving = false;
		}
	}

	function typeLabel(t: string): string {
		if (t === 'pdf') return 'PDF';
		if (t === 'video') return 'Video';
		return 'Link';
	}
</script>

<svelte:head>
	<title>Resources — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-4xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold text-white">Resource library</h1>
			<p class="mt-1 text-sm text-slate-400">
				Links, PDFs, and videos shown on the public /resources page. Optional per-course.
			</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center justify-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			Add resource
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
			No resources yet. Add links, PDFs, or videos for students.
		</div>
	{:else}
		<div class="space-y-3">
			{#each items as item}
				<div class="flex items-start justify-between gap-4 rounded-xl border border-slate-700 bg-slate-800/50 p-4">
					<div class="min-w-0 flex-1">
						<h3 class="font-medium text-white">{item.title}</h3>
						<p class="mt-0.5 text-sm text-slate-400">
							{typeLabel(item.resource_type)} · {item.course_id ?? 'All courses'}
						</p>
						<a
							href={item.url}
							target="_blank"
							rel="noopener noreferrer"
							class="mt-1 block truncate text-xs text-pink-400 hover:underline"
						>
							{item.url}
						</a>
					</div>
					<div class="flex shrink-0 gap-2">
						<button
							type="button"
							onclick={() => openEdit(item)}
							class="rounded-lg px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-700 hover:text-white"
						>
							Edit
						</button>
						<button
							type="button"
							onclick={() => remove(item)}
							disabled={saving}
							class="rounded-lg px-3 py-1.5 text-sm text-red-400 hover:bg-red-500/20 disabled:opacity-50"
						>
							Delete
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
		aria-labelledby="resource-modal-title"
		tabindex="-1"
		onclick={closeModal}
		onkeydown={(e) => e.key === 'Escape' && closeModal()}
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			role="document"
			class="w-full max-w-md rounded-2xl border border-slate-700 bg-slate-900 shadow-xl"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && closeModal()}
		>
			<div class="border-b border-slate-800 px-6 py-4">
				<h2 id="resource-modal-title" class="text-lg font-semibold text-white">
					{editId ? 'Edit resource' : 'Add resource'}
				</h2>
			</div>
			<form onsubmit={(e) => { e.preventDefault(); saveItem(); }} class="space-y-4 px-6 py-4">
				<div>
					<label for="res-title" class="block text-sm font-medium text-slate-400">Title</label>
					<input
						id="res-title"
						type="text"
						bind:value={form.title}
						required
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="e.g. Week 1 handout"
					/>
				</div>
				<div>
					<label for="res-url" class="block text-sm font-medium text-slate-400">URL</label>
					<input
						id="res-url"
						type="url"
						bind:value={form.url}
						required
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="https://..."
					/>
				</div>
				<div>
					<label for="res-type" class="block text-sm font-medium text-slate-400">Type</label>
					<select
						id="res-type"
						bind:value={form.resource_type}
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
					>
						<option value="link">Link</option>
						<option value="pdf">PDF</option>
						<option value="video">Video</option>
					</select>
				</div>
				<div>
					<label for="res-course" class="block text-sm font-medium text-slate-400">Course (optional)</label>
					<select
						id="res-course"
						bind:value={form.course_id}
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
					>
						<option value="">All courses</option>
						{#each courses as c}
							<option value={c.id}>{c.name}</option>
						{/each}
					</select>
				</div>
				<div>
					<label for="res-sort" class="block text-sm font-medium text-slate-400">Order</label>
					<input
						id="res-sort"
						type="number"
						bind:value={form.sort_order}
						min="0"
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
					/>
				</div>
				<div class="flex justify-end gap-3 pt-2">
					<button
						type="button"
						onclick={closeModal}
						class="rounded-lg px-4 py-2.5 text-sm font-medium text-slate-400 hover:bg-slate-800 hover:text-white"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={saving}
						class="rounded-lg bg-pink-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-pink-500 disabled:opacity-60"
					>
						{saving ? 'Saving…' : editId ? 'Update' : 'Add resource'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
