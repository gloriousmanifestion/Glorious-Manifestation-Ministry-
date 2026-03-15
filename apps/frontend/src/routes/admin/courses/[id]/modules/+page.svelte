<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import * as adminApi from '$lib/admin-api';
	import type { AdminCourseModule } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let { data }: { data: { courseId: string; courseName: string } } = $props();

	let modules = $state<AdminCourseModule[]>([]);
	let loading = $state(true);
	let error = $state('');
	let modalOpen = $state(false);
	let editing = $state<AdminCourseModule | null>(null);
	let form = $state({
		title: '',
		sort_order: 0,
		link: '' as string,
		description: '' as string
	});
	let submitLoading = $state(false);
	let deleteLoading = $state<string | null>(null);

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			modules = await adminApi.getCourseModules(data.courseId);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load modules';
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editing = null;
		form = {
			title: '',
			sort_order: modules.length,
			link: '',
			description: ''
		};
		modalOpen = true;
	}

	function openEdit(m: AdminCourseModule) {
		editing = m;
		form = {
			title: m.title,
			sort_order: m.sort_order,
			link: m.link ?? '',
			description: m.description ?? ''
		};
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
		editing = null;
	}

	async function submit() {
		submitLoading = true;
		error = '';
		try {
			if (editing) {
				await adminApi.updateCourseModule(data.courseId, editing.id, {
					title: form.title.trim(),
					sort_order: form.sort_order,
					link: form.link.trim() || null,
					description: form.description.trim() || null
				});
			} else {
				await adminApi.createCourseModule(data.courseId, {
					title: form.title.trim(),
					sort_order: form.sort_order,
					link: form.link.trim() || null,
					description: form.description.trim() || null
				});
			}
			closeModal();
			await load();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save module';
		} finally {
			submitLoading = false;
		}
	}

	async function remove(m: AdminCourseModule) {
		if (!confirm('Delete this module?')) return;
		deleteLoading = m.id;
		error = '';
		try {
			await adminApi.deleteCourseModule(data.courseId, m.id);
			await load();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete';
		} finally {
			deleteLoading = null;
		}
	}
</script>

<svelte:head>
	<title>Syllabus: {data.courseName} — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-4xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<a
				href="/admin/courses"
				class="inline-flex items-center gap-2 text-sm font-medium text-slate-400 hover:text-white"
			>
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
				</svg>
				Courses
			</a>
			<h1 class="mt-2 text-2xl font-bold tracking-tight text-white sm:text-3xl">
				Syllabus: {data.courseName}
			</h1>
			<p class="mt-1 text-slate-400">Manage curriculum modules for this course. They appear on the public course page.</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center gap-2 rounded-lg bg-pink-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
			</svg>
			Add module
		</button>
	</header>

	{#if error}
		<div class="mb-6 rounded-xl border border-red-800/50 bg-red-900/20 px-4 py-3 text-red-300">
			{error}
		</div>
	{/if}

	{#if loading}
		<div class="flex items-center justify-center py-24">
			<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
		</div>
	{:else}
		<ul class="space-y-3">
			{#each modules as mod}
				<li class="flex items-center gap-4 rounded-xl border border-slate-800 bg-slate-900/80 p-4">
					<div class="min-w-0 flex-1">
						<p class="text-xs font-medium uppercase tracking-wider text-pink-400">
							Module {mod.sort_order + 1}
						</p>
						<h3 class="font-semibold text-white">{mod.title}</h3>
						{#if mod.description}
							<p class="mt-1 text-sm text-slate-400 line-clamp-2">{mod.description}</p>
						{/if}
						{#if mod.link}
							<a
								href={mod.link}
								target="_blank"
								rel="noopener noreferrer"
								class="mt-1 inline-block text-xs text-pink-400 hover:underline"
							>
								{mod.link}
							</a>
						{/if}
					</div>
					<div class="flex shrink-0 gap-1">
						<button
							type="button"
							onclick={() => openEdit(mod)}
							class="rounded-lg p-2 text-slate-400 hover:bg-slate-800 hover:text-white"
							aria-label="Edit module"
						>
							<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
							</svg>
						</button>
						<button
							type="button"
							onclick={() => remove(mod)}
							disabled={deleteLoading === mod.id}
							class="rounded-lg p-2 text-slate-400 hover:bg-red-900/30 hover:text-red-400 disabled:opacity-50"
							aria-label="Delete module"
						>
							{#if deleteLoading === mod.id}
								<span class="h-5 w-5 block animate-spin rounded-full border-2 border-current border-t-transparent"></span>
							{:else}
								<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
								</svg>
							{/if}
						</button>
					</div>
				</li>
			{/each}
		</ul>
		{#if modules.length === 0}
			<div class="rounded-xl border border-slate-800 bg-slate-900/50 py-16 text-center text-slate-500">
				No modules yet. Add modules to show a curriculum on the public course page.
			</div>
		{/if}
	{/if}
</div>

<!-- Add/Edit modal -->
{#if modalOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
		role="dialog"
		aria-modal="true"
		aria-labelledby="module-modal-title"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div
			role="document"
			class="w-full max-w-md rounded-2xl border border-slate-700 bg-slate-900 shadow-xl"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && closeModal()}
		>
			<div class="border-b border-slate-800 px-6 py-4">
				<h2 id="module-modal-title" class="text-lg font-semibold text-white">
					{editing ? 'Edit module' : 'Add module'}
				</h2>
			</div>
			<form onsubmit={(e) => { e.preventDefault(); submit(); }} class="space-y-4 px-6 py-4">
				<div>
					<label for="mod-title" class="block text-sm font-medium text-slate-400">Title</label>
					<input
						id="mod-title"
						type="text"
						bind:value={form.title}
						required
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="e.g. Foundations of pastoral leadership"
					/>
				</div>
				<div>
					<label for="mod-sort" class="block text-sm font-medium text-slate-400">Order</label>
					<input
						id="mod-sort"
						type="number"
						bind:value={form.sort_order}
						min="0"
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
					/>
				</div>
				<div>
					<label for="mod-link" class="block text-sm font-medium text-slate-400">Link (optional)</label>
					<input
						id="mod-link"
						type="url"
						bind:value={form.link}
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="https://..."
					/>
				</div>
				<div>
					<label for="mod-desc" class="block text-sm font-medium text-slate-400">Description (optional)</label>
					<textarea
						id="mod-desc"
						bind:value={form.description}
						rows="2"
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="Brief description"
					></textarea>
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
						disabled={submitLoading}
						class="rounded-lg bg-pink-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-pink-500 disabled:opacity-60"
					>
						{submitLoading ? 'Saving…' : editing ? 'Update' : 'Add module'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
