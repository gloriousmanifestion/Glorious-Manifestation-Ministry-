<script lang="ts">
	import { onMount } from 'svelte';
	import {
		listEventsAdmin,
		createEvent,
		updateEvent,
		deleteEvent,
		type Event
	} from '$lib/api/events';
	import { SITE } from '$lib/config';

	let items = $state<Event[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let modalOpen = $state(false);
	let editId = $state<string | null>(null);
	let form = $state({ title: '', date: '', description: '' });

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			items = await listEventsAdmin();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load events';
			items = [];
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editId = null;
		form = { title: '', date: '', description: '' };
		modalOpen = true;
	}

	function openEdit(ev: Event) {
		editId = ev.id;
		form = {
			title: ev.title,
			date: ev.date,
			description: ev.description ?? ''
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
		if (!form.date.trim()) {
			error = 'Date is required';
			return;
		}
		saving = true;
		error = '';
		try {
			if (editId) {
				const updated = await updateEvent(editId, {
					title: form.title.trim(),
					date: form.date.trim(),
					description: form.description.trim() || null
				});
				items = items.map((e) => (e.id === editId ? updated : e));
			} else {
				const created = await createEvent({
					title: form.title.trim(),
					date: form.date.trim(),
					description: form.description.trim() || null
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

	async function remove(ev: Event) {
		if (!confirm('Delete this event?')) return;
		saving = true;
		error = '';
		try {
			await deleteEvent(ev.id);
			items = items.filter((e) => e.id !== ev.id);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>Events — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-4xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold text-white">Events</h1>
			<p class="mt-1 text-sm text-slate-400">
				Manage school calendar events shown on the public events page.
			</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center justify-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			Add event
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
			No events yet. Add one to show on the public <a href="/events" class="text-pink-400 hover:underline">events page</a>.
		</div>
	{:else}
		<div class="space-y-3">
			{#each items as item}
				<div class="flex items-start justify-between gap-4 rounded-xl border border-slate-700 bg-slate-800/50 p-4">
					<div class="min-w-0 flex-1">
						<h3 class="font-medium text-white">{item.title}</h3>
						<p class="mt-1 text-sm text-slate-400">{item.date}</p>
						{#if item.description}
							<p class="mt-1 line-clamp-2 text-sm text-slate-500">{item.description}</p>
						{/if}
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
		<div class="relative z-10 w-full max-w-lg rounded-xl border border-slate-700 bg-slate-900 p-6 shadow-xl">
			<h2 class="text-lg font-bold text-white">
				{editId ? 'Edit event' : 'Add event'}
			</h2>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					saveItem();
				}}
				class="mt-4 space-y-4"
			>
				<div>
					<label for="ev-title" class="block text-sm font-medium text-slate-300">Title</label>
					<input
						id="ev-title"
						type="text"
						bind:value={form.title}
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="e.g. School Opening"
					/>
				</div>
				<div>
					<label for="ev-date" class="block text-sm font-medium text-slate-300">Date</label>
					<input
						id="ev-date"
						type="text"
						bind:value={form.date}
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="e.g. February or 2025-02-01"
					/>
				</div>
				<div>
					<label for="ev-desc" class="block text-sm font-medium text-slate-300">Description</label>
					<textarea
						id="ev-desc"
						bind:value={form.description}
						rows="3"
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="Optional description"
					></textarea>
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
