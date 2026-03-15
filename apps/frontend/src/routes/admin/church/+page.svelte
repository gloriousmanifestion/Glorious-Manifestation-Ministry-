<script lang="ts">
	import { onMount } from 'svelte';
	import * as churchApi from '$lib/api/church';
	import type { ChurchService } from '$lib/api/church';
	import { SITE } from '$lib/config';

	let services = $state<ChurchService[]>([]);
	let loading = $state(true);
	let error = $state('');
	let modalOpen = $state(false);
	let editing = $state<ChurchService | null>(null);
	let form = $state({
		name: '',
		description: '',
		day_of_week: 'Sunday',
		time: '',
		sort_order: 0,
		image_url: '' as string | null,
		featuresText: '',
		is_active: true
	});
	let uploadProgress = $state(false);
	let submitLoading = $state(false);

	const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			services = await churchApi.listChurchServicesAdmin();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load church services';
		} finally {
			loading = false;
		}
	}

	function openAdd() {
		editing = null;
		form = {
			name: '',
			description: '',
			day_of_week: 'Sunday',
			time: '',
			sort_order: services.length,
			image_url: null,
			featuresText: '',
			is_active: true
		};
		modalOpen = true;
	}

	function openEdit(s: ChurchService) {
		editing = s;
		form = {
			name: s.name,
			description: s.description ?? '',
			day_of_week: s.day_of_week,
			time: s.time,
			sort_order: s.sort_order,
			image_url: s.image_url ?? null,
			featuresText: Array.isArray(s.features) ? s.features.join(', ') : '',
			is_active: s.is_active
		};
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
		editing = null;
	}

	async function onFileSelect(e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;
		uploadProgress = true;
		error = '';
		try {
			const { image_url } = await churchApi.uploadChurchArtwork(file);
			form.image_url = image_url;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Upload failed';
		} finally {
			uploadProgress = false;
			input.value = '';
		}
	}

	function clearImage() {
		form.image_url = null;
	}

	async function submit() {
		submitLoading = true;
		error = '';
		const features = form.featuresText
			.split(',')
			.map((s) => s.trim())
			.filter(Boolean);
		try {
			if (editing) {
				await churchApi.updateChurchService(editing.id, {
					name: form.name.trim(),
					description: form.description.trim() || undefined,
					day_of_week: form.day_of_week,
					time: form.time.trim(),
					sort_order: form.sort_order,
					image_url: form.image_url || null,
					features: features.length ? features : undefined,
					is_active: form.is_active
				});
			} else {
				const created = await churchApi.createChurchService({
					name: form.name.trim(),
					description: form.description.trim() || undefined,
					day_of_week: form.day_of_week,
					time: form.time.trim(),
					sort_order: form.sort_order,
					features: features.length ? features : undefined,
					is_active: form.is_active
				});
				if (form.image_url) {
					await churchApi.updateChurchService(created.id, { image_url: form.image_url });
				}
			}
			closeModal();
			await load();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save service';
		} finally {
			submitLoading = false;
		}
	}

	async function remove(s: ChurchService) {
		if (!confirm(`Delete "${s.name}"?`)) return;
		error = '';
		try {
			await churchApi.deleteChurchService(s.id);
			await load();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete';
		}
	}
</script>

<svelte:head>
	<title>Church — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-7xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold text-white">Church services</h1>
			<p class="mt-1 text-sm text-slate-400">
				Manage weekly services shown on the public Church page.
			</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center justify-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			Add service
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
	{:else if services.length === 0}
		<div class="rounded-xl border border-slate-700 bg-slate-800/50 p-8 text-center text-slate-400">
			No church services yet. Add one to show on the public Church page.
		</div>
	{:else}
		<div class="space-y-3">
			{#each services as s}
				<article
					class="flex flex-wrap items-center justify-between gap-4 rounded-xl border border-slate-700 bg-slate-800/50 p-4"
				>
					<div class="flex min-w-0 flex-1 items-center gap-4">
						{#if s.image_url}
							<img
								src={churchApi.churchImageUrl(s.image_url)}
								alt=""
								class="h-14 w-14 shrink-0 rounded-lg object-cover"
							/>
						{:else}
							<div
								class="flex h-14 w-14 shrink-0 items-center justify-center rounded-lg bg-slate-700 text-slate-500"
							>
								—
							</div>
						{/if}
						<div class="min-w-0">
							<h3 class="font-semibold text-white">{s.name}</h3>
							<p class="text-sm text-slate-400">
								{s.day_of_week} · {s.time}
								{#if !s.is_active}
									<span class="ml-2 text-amber-400">(hidden)</span>
								{/if}
							</p>
							{#if s.description}
								<p class="mt-1 line-clamp-2 text-sm text-slate-500">{s.description}</p>
							{/if}
						</div>
					</div>
					<div class="flex shrink-0 gap-2">
						<button
							type="button"
							onclick={() => openEdit(s)}
							class="rounded-lg border border-slate-600 px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-700"
						>
							Edit
						</button>
						<button
							type="button"
							onclick={() => remove(s)}
							class="rounded-lg border border-red-500/50 px-3 py-1.5 text-sm text-red-400 hover:bg-red-500/10"
						>
							Delete
						</button>
					</div>
				</article>
			{/each}
		</div>
	{/if}
</div>

<!-- Modal -->
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
		<div
			class="relative z-10 max-h-[90vh] w-full max-w-lg overflow-y-auto rounded-xl border border-slate-700 bg-slate-900 p-6 shadow-xl"
		>
			<h2 class="text-lg font-bold text-white">
				{editing ? 'Edit service' : 'Add service'}
			</h2>

			<form
				onsubmit={(e) => {
					e.preventDefault();
					submit();
				}}
				class="mt-4 space-y-4"
			>
				<div>
					<label for="church-name" class="block text-sm font-medium text-slate-300">Name</label>
					<input
						id="church-name"
						type="text"
						bind:value={form.name}
						required
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
						placeholder="e.g. Sunday Worship"
					/>
				</div>
				<div>
					<label for="church-desc" class="block text-sm font-medium text-slate-300">Description</label>
					<textarea
						id="church-desc"
						bind:value={form.description}
						rows="2"
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
						placeholder="Brief description"
					></textarea>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="church-day" class="block text-sm font-medium text-slate-300">Day</label>
						<select
							id="church-day"
							bind:value={form.day_of_week}
							class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
						>
							{#each DAYS as d}
								<option value={d}>{d}</option>
							{/each}
						</select>
					</div>
					<div>
						<label for="church-time" class="block text-sm font-medium text-slate-300">Time</label>
						<input
							id="church-time"
							type="text"
							bind:value={form.time}
							required
							class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
							placeholder="e.g. 10:00 AM"
						/>
					</div>
				</div>
				<div>
					<label for="church-sort" class="block text-sm font-medium text-slate-300">Sort order</label>
					<input
						id="church-sort"
						type="number"
						bind:value={form.sort_order}
						min="0"
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white"
					/>
				</div>
				<div>
					<label for="church-artwork" class="block text-sm font-medium text-slate-300"
						>Artwork / image</label
					>
					<div class="mt-1">
						<input
							id="church-artwork"
							type="file"
							accept="image/*"
							onchange={onFileSelect}
							disabled={uploadProgress}
							class="text-sm text-slate-400 file:mr-2 file:rounded file:border-0 file:bg-pink-600 file:px-3 file:py-1.5 file:text-white file:hover:bg-pink-500 {form.image_url
								? 'sr-only'
								: ''}"
							aria-hidden={!!form.image_url}
						/>
						{#if form.image_url}
							<div class="flex items-center gap-3">
								<img
									src={churchApi.churchImageUrl(form.image_url)}
									alt=""
									class="h-16 w-16 rounded-lg object-cover"
								/>
								<button
									type="button"
									onclick={clearImage}
									class="text-sm text-slate-400 hover:text-white"
								>
									Remove
								</button>
							</div>
						{:else if uploadProgress}
							<span class="ml-2 text-sm text-slate-500">Uploading…</span>
						{/if}
					</div>
				</div>
				<div>
					<label for="church-features" class="block text-sm font-medium text-slate-300">Features (comma-separated)</label>
					<input
						id="church-features"
						type="text"
						bind:value={form.featuresText}
						class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
						placeholder="e.g. Worship, Teaching, Kids"
					/>
				</div>
				<div class="flex items-center gap-2">
					<input
						id="church-active"
						type="checkbox"
						bind:checked={form.is_active}
						class="h-4 w-4 rounded border-slate-600 bg-slate-800 text-pink-600"
					/>
					<label for="church-active" class="text-sm text-slate-300">Visible on public Church page</label>
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
						disabled={submitLoading}
						class="rounded-lg bg-pink-600 px-4 py-2 text-sm font-semibold text-white hover:bg-pink-500 disabled:opacity-50"
					>
						{submitLoading ? 'Saving…' : editing ? 'Update' : 'Add service'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
