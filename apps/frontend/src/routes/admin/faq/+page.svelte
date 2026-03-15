<script lang="ts">
	import { onMount } from 'svelte';
	import { getChurchContent, setChurchContent } from '$lib/api/church';
	import { SITE } from '$lib/config';

	interface FaqItem {
		q: string;
		a: string;
	}

	let items = $state<FaqItem[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let modalOpen = $state(false);
	let editIndex = $state<number | null>(null);
	let form = $state({ q: '', a: '' });

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			const raw = await getChurchContent('faq');
			if (raw) {
				const parsed = JSON.parse(raw) as unknown;
				items = Array.isArray(parsed)
					? (parsed as FaqItem[]).filter((x) => x && typeof x.q === 'string' && typeof x.a === 'string')
					: [];
			} else {
				items = [];
			}
		} catch {
			items = [];
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editIndex = null;
		form = { q: '', a: '' };
		modalOpen = true;
	}

	function openEdit(i: number) {
		editIndex = i;
		form = { q: items[i].q, a: items[i].a };
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
		editIndex = null;
	}

	async function saveItem() {
		if (!form.q.trim()) {
			error = 'Question is required';
			return;
		}
		saving = true;
		error = '';
		try {
			let next: FaqItem[];
			if (editIndex !== null) {
				next = items.slice();
				next[editIndex] = { q: form.q.trim(), a: form.a.trim() };
			} else {
				next = [...items, { q: form.q.trim(), a: form.a.trim() }];
			}
			await setChurchContent('faq', JSON.stringify(next));
			items = next;
			closeModal();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save';
		} finally {
			saving = false;
		}
	}

	async function remove(i: number) {
		if (!confirm('Remove this FAQ entry?')) return;
		const next = items.filter((_, idx) => idx !== i);
		saving = true;
		error = '';
		try {
			await setChurchContent('faq', JSON.stringify(next));
			items = next;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to remove';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>FAQ — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-4xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold text-white">FAQ</h1>
			<p class="mt-1 text-sm text-slate-400">Manage frequently asked questions shown on the public FAQ page.</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center justify-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			Add entry
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
			<p>No custom FAQ entries yet. The public <a href="/faq" class="text-pink-400 hover:underline">FAQ page</a> currently shows default questions.</p>
			<p class="mt-2 text-sm">Add entries here to replace the defaults with your own.</p>
		</div>
	{:else}
		<div class="space-y-3">
			{#each items as item, i}
				<div class="flex items-start justify-between gap-4 rounded-xl border border-slate-700 bg-slate-800/50 p-4">
					<div class="min-w-0 flex-1">
						<h3 class="font-medium text-white">{item.q}</h3>
						<p class="mt-1 line-clamp-2 text-sm text-slate-400">{item.a}</p>
					</div>
					<div class="flex shrink-0 gap-2">
						<button
							type="button"
							onclick={() => openEdit(i)}
							class="rounded-lg border border-slate-600 px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-700"
						>
							Edit
						</button>
						<button
							type="button"
							onclick={() => remove(i)}
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
			<h2 class="text-lg font-bold text-white">{editIndex !== null ? 'Edit entry' : 'Add entry'}</h2>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					saveItem();
				}}
				class="mt-4 space-y-4"
			>
				<div>
					<label for="faq-q" class="block text-sm font-medium text-slate-300">Question</label>
					<input
						id="faq-q"
						type="text"
						bind:value={form.q}
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="e.g. How do I apply?"
					/>
				</div>
				<div>
					<label for="faq-a" class="block text-sm font-medium text-slate-300">Answer</label>
					<textarea
						id="faq-a"
						bind:value={form.a}
						rows="4"
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						placeholder="Answer (plain text or simple HTML)"
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
