<script lang="ts">
	import { onMount } from 'svelte';
	import * as adminApi from '$lib/admin-api';
	import type { LearningPlan, AdminCourse } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let plans = $state<LearningPlan[]>([]);
	let courses = $state<AdminCourse[]>([]);
	let loading = $state(true);
	let error = $state('');
	let modalOpen = $state(false);
	let editing = $state<LearningPlan | null>(null);
	let form = $state({
		name: '',
		description: '',
		price: '' as string | number,
		currency: 'NGN',
		duration: '',
		courseIds: [] as string[]
	});
	let submitLoading = $state(false);

	onMount(async () => {
		loading = true;
		error = '';
		try {
			[plans, courses] = await Promise.all([
				adminApi.getLearningPlans(),
				adminApi.getAdminCourses()
			]);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load data';
		} finally {
			loading = false;
		}
	});

	function openAdd() {
		editing = null;
		form = {
			name: '',
			description: '',
			price: '',
			currency: 'NGN',
			duration: SITE.duration,
			courseIds: courses.map((c) => c.id)
		};
		modalOpen = true;
	}

	function openEdit(p: LearningPlan) {
		editing = p;
		form = {
			name: p.name,
			description: p.description,
			price: p.price,
			currency: p.currency,
			duration: p.duration,
			courseIds: [...p.courseIds]
		};
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
		editing = null;
	}

	function toggleCourse(id: string) {
		if (form.courseIds.includes(id)) {
			form.courseIds = form.courseIds.filter((x) => x !== id);
		} else {
			form.courseIds = [...form.courseIds, id];
		}
	}

	async function submit() {
		submitLoading = true;
		error = '';
		const price = typeof form.price === 'string' ? Number(form.price) : form.price;
		if (isNaN(price) || price < 0) {
			error = 'Please enter a valid price';
			submitLoading = false;
			return;
		}
		try {
			if (editing) {
				await adminApi.updateLearningPlan(editing.id, {
					name: form.name,
					description: form.description,
					price,
					duration: form.duration,
					courseIds: form.courseIds
				});
			} else {
				await adminApi.createLearningPlan({
					name: form.name,
					description: form.description,
					price,
					currency: form.currency,
					duration: form.duration,
					courseIds: form.courseIds
				});
			}
			closeModal();
			plans = await adminApi.getLearningPlans();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save learning plan';
		} finally {
			submitLoading = false;
		}
	}

	function formatCurrency(amount: number, currency: string): string {
		if (currency === 'NGN') return '₦' + amount.toLocaleString();
		return `${currency} ${amount.toLocaleString()}`;
	}
</script>

<svelte:head>
	<title>Learning plans — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-7xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">Learning plans</h1>
			<p class="mt-1 text-slate-400">Create plans, set prices, and bundle courses</p>
		</div>
		<button
			type="button"
			onclick={openAdd}
			class="inline-flex items-center gap-2 rounded-lg bg-pink-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
			</svg>
			New learning plan
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
		<div class="space-y-4">
			{#each plans as p}
				<div
					class="rounded-xl border border-slate-800 bg-slate-900/80 p-5 transition hover:border-slate-700"
				>
					<div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2">
								<h3 class="text-lg font-semibold text-white">{p.name}</h3>
								<span
									class="rounded-full px-2 py-0.5 text-xs font-medium {p.active
										? 'bg-emerald-500/20 text-emerald-400'
										: 'bg-slate-600/50 text-slate-400'}"
								>
									{p.active ? 'Active' : 'Inactive'}
								</span>
							</div>
							{#if p.description}
								<p class="mt-1 text-sm text-slate-400">{p.description}</p>
							{/if}
							<div class="mt-3 flex flex-wrap gap-3 text-sm">
								<span class="font-semibold text-pink-400">
									{formatCurrency(p.price, p.currency)}
								</span>
								<span class="text-slate-500">{p.duration}</span>
								<span class="text-slate-500">{p.courseIds.length} course(s)</span>
							</div>
						</div>
						<button
							type="button"
							onclick={() => openEdit(p)}
							class="shrink-0 rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium text-slate-300 hover:bg-slate-800 hover:text-white"
						>
							Edit
						</button>
					</div>
				</div>
			{/each}
		</div>
		{#if plans.length === 0}
			<div class="rounded-xl border border-slate-800 bg-slate-900/50 py-16 text-center text-slate-500">
				No learning plans yet. Create one to set prices and bundle courses for students.
			</div>
		{/if}
	{/if}
</div>

<!-- Modal -->
{#if modalOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 overflow-y-auto"
		role="dialog"
		aria-modal="true"
		aria-labelledby="plan-modal-title"
	>
		<div
			class="my-8 w-full max-w-lg rounded-2xl border border-slate-700 bg-slate-900 shadow-xl"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="border-b border-slate-800 px-6 py-4">
				<h2 id="plan-modal-title" class="text-lg font-semibold text-white">
					{editing ? 'Edit learning plan' : 'New learning plan'}
				</h2>
			</div>
			<form onsubmit={(e) => { e.preventDefault(); submit(); }} class="space-y-4 px-6 py-4">
				<div>
					<label for="plan-name" class="block text-sm font-medium text-slate-400">Name</label>
					<input
						id="plan-name"
						type="text"
						bind:value={form.name}
						required
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="e.g. Full Program 2025"
					/>
				</div>
				<div>
					<label for="plan-description" class="block text-sm font-medium text-slate-400">Description</label>
					<textarea
						id="plan-description"
						bind:value={form.description}
						rows="2"
						class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
						placeholder="Brief description"
					></textarea>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="plan-price" class="block text-sm font-medium text-slate-400">Price ({form.currency})</label>
						<input
							id="plan-price"
							type="number"
							bind:value={form.price}
							min="0"
							step="1"
							required
							class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
							placeholder="30000"
						/>
					</div>
					<div>
						<label for="plan-duration" class="block text-sm font-medium text-slate-400">Duration</label>
						<input
							id="plan-duration"
							type="text"
							bind:value={form.duration}
							required
							class="mt-1 w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-white focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
							placeholder="February to July"
						/>
					</div>
				</div>
				<div>
					<label class="block text-sm font-medium text-slate-400">Courses included</label>
					<div class="mt-2 space-y-2 rounded-lg border border-slate-700 bg-slate-800/50 p-3 max-h-48 overflow-y-auto">
						{#each courses as c}
							<label class="flex cursor-pointer items-center gap-2 text-sm text-slate-300">
								<input
									type="checkbox"
									checked={form.courseIds.includes(c.id)}
									onchange={() => toggleCourse(c.id)}
									class="rounded border-slate-600 bg-slate-800 text-pink-500 focus:ring-pink-500"
								/>
								{c.name}
							</label>
						{/each}
					</div>
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
						{submitLoading ? 'Saving…' : editing ? 'Update' : 'Create plan'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
