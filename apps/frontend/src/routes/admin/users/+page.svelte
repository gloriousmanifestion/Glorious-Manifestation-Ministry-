<script lang="ts">
	import { onMount } from 'svelte';
	import * as adminApi from '$lib/admin-api';
	import type { AdminUser } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let users = $state<AdminUser[]>([]);
	let total = $state(0);
	let loading = $state(true);
	let error = $state('');
	let search = $state('');
	let page = $state(1);
	const limit = 20;

	onMount(load);

	async function load() {
		loading = true;
		error = '';
		try {
			const res = await adminApi.getAdminUsers({
				search: search || undefined,
				page,
				limit
			});
			users = res.users;
			total = res.total;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load users';
		} finally {
			loading = false;
		}
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString(undefined, {
			day: 'numeric',
			month: 'short',
			year: 'numeric'
		});
	}

	function onSearchSubmit(e: Event) {
		e.preventDefault();
		page = 1;
		load();
	}
</script>

<svelte:head>
	<title>Users — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-7xl">
	<header class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">Users</h1>
			<p class="mt-1 text-slate-400">Manage registrations and student accounts</p>
		</div>
	</header>

	{#if error}
		<div class="mb-6 rounded-xl border border-red-800/50 bg-red-900/20 px-4 py-3 text-red-300">
			{error}
		</div>
	{/if}

	<!-- Search -->
	<form onsubmit={onSearchSubmit} class="mb-6 flex gap-3">
		<input
			type="search"
			bind:value={search}
			placeholder="Search by name or email…"
			class="flex-1 rounded-lg border border-slate-700 bg-slate-800/80 px-4 py-2.5 text-white placeholder-slate-500 focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500"
		/>
		<button
			type="submit"
			class="rounded-lg bg-pink-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-pink-500"
		>
			Search
		</button>
	</form>

	{#if loading}
		<div class="flex items-center justify-center py-24">
			<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
		</div>
	{:else}
		<div class="rounded-xl border border-slate-800 bg-slate-900/80 overflow-hidden">
			<div class="overflow-x-auto">
				<table class="w-full min-w-[800px]">
					<thead>
						<tr class="border-b border-slate-800 text-left text-xs font-medium uppercase tracking-wider text-slate-500">
							<th class="px-5 py-3">Name</th>
							<th class="px-5 py-3">Email</th>
							<th class="px-5 py-3">Phone</th>
							<th class="px-5 py-3">Church / Role</th>
							<th class="px-5 py-3">Course</th>
							<th class="px-5 py-3">Status</th>
							<th class="px-5 py-3">Registered</th>
						</tr>
					</thead>
					<tbody>
						{#each users as u}
							<tr class="border-b border-slate-800/80 transition hover:bg-slate-800/50">
								<td class="px-5 py-3 font-medium text-white">{u.name}</td>
								<td class="px-5 py-3 text-slate-400">{u.email}</td>
								<td class="px-5 py-3 text-slate-400">{u.phone}</td>
								<td class="px-5 py-3 text-slate-400">
									{u.churchName}
									<span class="text-slate-600"> · </span>
									{u.ministryRole}
								</td>
								<td class="px-5 py-3 text-slate-400">
									{u.selectedCourseId ?? '—'}
								</td>
								<td class="px-5 py-3">
									<span class="inline-flex flex-wrap gap-1">
										<span
											class="rounded-full px-2 py-0.5 text-xs font-medium {u.verified
												? 'bg-emerald-500/20 text-emerald-400'
												: 'bg-amber-500/20 text-amber-400'}"
										>
											{u.verified ? 'Verified' : 'Pending'}
										</span>
										<span
											class="rounded-full px-2 py-0.5 text-xs font-medium {u.paymentComplete
												? 'bg-emerald-500/20 text-emerald-400'
												: 'bg-slate-600/50 text-slate-400'}"
										>
											{u.paymentComplete ? 'Paid' : 'Unpaid'}
										</span>
									</span>
								</td>
								<td class="px-5 py-3 text-slate-500">{formatDate(u.createdAt)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			{#if users.length === 0}
				<div class="px-5 py-12 text-center text-slate-500">No users found.</div>
			{/if}
			{#if total > limit}
				<div class="flex items-center justify-between border-t border-slate-800 px-5 py-3 text-sm text-slate-500">
					<span>Showing {users.length} of {total}</span>
					<div class="flex gap-2">
						<button
							disabled={page <= 1}
							onclick={() => {
								page--;
								load();
							}}
							class="rounded px-3 py-1 text-pink-400 hover:bg-slate-800 disabled:opacity-50 disabled:hover:bg-transparent"
						>
							Previous
						</button>
						<button
							disabled={page * limit >= total}
							onclick={() => {
								page++;
								load();
							}}
							class="rounded px-3 py-1 text-pink-400 hover:bg-slate-800 disabled:opacity-50 disabled:hover:bg-transparent"
						>
							Next
						</button>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>
