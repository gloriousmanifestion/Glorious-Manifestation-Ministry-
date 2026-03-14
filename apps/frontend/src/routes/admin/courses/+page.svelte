<script lang="ts">
	import { onMount } from 'svelte';
	import * as adminApi from '$lib/admin-api';
	import type { AdminCourse } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let courses = $state<AdminCourse[]>([]);
	let loading = $state(true);
	let error = $state('');
	let modalOpen = $state(false);
	let editing = $state<AdminCourse | null>(null);
	let form = $state({
		name: '',
		duration: '',
		description: '',
		price: '' as string | number,
		teams_class_id: '' as string
	});
	let submitLoading = $state(false);

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			courses = await adminApi.getAdminCourses();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load courses';
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editing = null;
		form = { name: '', duration: '8 weeks', description: '', price: '', teams_class_id: '' };
		modalOpen = true;
	}

	function openEdit(c: AdminCourse) {
		editing = c;
		form = {
			name: c.name,
			duration: c.duration,
			description: c.description ?? '',
			price: c.price ?? '',
			teams_class_id: c.teams_class_id ?? ''
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
			const price = form.price === '' ? undefined : Number(form.price);
			const teams_class_id = form.teams_class_id.trim() || null;
			if (editing) {
				await adminApi.updateCourse(editing.id, {
					name: form.name,
					duration: form.duration,
					description: form.description || undefined,
					price,
					teams_class_id
				});
			} else {
				await adminApi.createCourse({
					name: form.name,
					duration: form.duration,
					description: form.description || undefined,
					price,
					teams_class_id
				});
			}
			closeModal();
			await load();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save course';
		} finally {
			submitLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Courses — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-7xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">Courses</h1>
			<p class="mt-1 text-slate-400">Add and manage pastoral school courses</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center gap-2 rounded-lg bg-pink-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
			</svg>
			Add course
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
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each courses as c}
				<div
					class="rounded-xl border border-slate-800 bg-slate-900/80 p-5 transition hover:border-slate-700"
				>
					<div class="flex items-start justify-between">
						<div class="min-w-0 flex-1">
							<h3 class="font-semibold text-white">{c.name}</h3>
							<p class="mt-1 text-sm text-slate-400">{c.duration}</p>
							{#if c.price != null}
								<p class="mt-1 text-sm text-pink-400">₦{c.price.toLocaleString()}</p>
							{/if}
							<span
								class="mt-2 inline-block rounded-full px-2 py-0.5 text-xs font-medium {c.active
									? 'bg-emerald-500/20 text-emerald-400'
									: 'bg-slate-600/50 text-slate-400'}"
							>
								{c.active ? 'Active' : 'Inactive'}
							</span>
						</div>
						<button
							type="button"
							onclick={() => openEdit(c)}
							class="rounded-lg p-2 text-slate-400 hover:bg-slate-800 hover:text-white"
							aria-label="Edit course"
						>
							<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
							</svg>
						</button>
					</div>
				</div>
			{/each}
		</div>
		{#if courses.length === 0}
			<div class="rounded-xl border border-slate-800 bg-slate-900/50 py-16 text-center text-slate-500">
				No courses yet. Add your first course to get started.
			</div>
		{/if}
	{/if}
</div>

<!-- Modal -->
{#if modalOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
		role="dialog"
		aria-modal="true"
		aria-labelledby="course-modal-title"
	>
		<!-- Modal content: stopPropagation so clicking inside doesn't close. Div required (cannot nest form in button). -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="w-full max-w-md rounded-2xl border border-slate-700 bg-slate-900 shadow-xl"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="border-b border-slate-800 px-6 py-4">
				<h2 id="course-modal-title" class="text-lg font-semibold text-white">
					{editing ? 'Edit course' : 'Add course'}
				</h2>
			</div>
			<form onsubmit={(e) => { e.preventDefault(); submit(); }} class="space-y-4 px-6 py-4">
				<div>
					<label for="course-name" class="block text-sm font-medium text-slate-400">Name</label>
					<input
						id="course-name"
						type="text"
						bind:value={form.name}
						required
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="e.g. Pastoral Leadership"
					/>
				</div>
				<div>
					<label for="course-duration" class="block text-sm font-medium text-slate-400">Duration</label>
					<input
						id="course-duration"
						type="text"
						bind:value={form.duration}
						required
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="e.g. 8 weeks"
					/>
				</div>
				<div>
					<label for="course-description" class="block text-sm font-medium text-slate-400">Description (optional)</label>
					<textarea
						id="course-description"
						bind:value={form.description}
						rows="3"
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="Brief description of the course"
					></textarea>
				</div>
				<div>
					<label for="course-price" class="block text-sm font-medium text-slate-400">Price (optional, NGN)</label>
					<input
						id="course-price"
						type="number"
						bind:value={form.price}
						min="0"
						step="1"
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="30000"
					/>
				</div>
				<div>
					<label for="course-teams-class-id" class="block text-sm font-medium text-slate-400">Teams class ID (optional)</label>
					<input
						id="course-teams-class-id"
						type="text"
						bind:value={form.teams_class_id}
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="Paste Teams class/team ID for “Open in Teams” link"
					/>
					<p class="mt-1 text-xs text-slate-500">Students will see an “Open in Teams” link on their dashboard when set.</p>
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
						{submitLoading ? 'Saving…' : editing ? 'Update' : 'Add course'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
