<script lang="ts">
	import { onMount } from 'svelte';
	import * as adminApi from '$lib/admin-api';
	import type { AdminMetrics, AdminUser, AdminPaymentProof } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let metrics = $state<AdminMetrics | null>(null);
	let recentUsers = $state<AdminUser[]>([]);
	let loading = $state(true);
	let error = $state('');
	let pendingProofs = $state<AdminPaymentProof[]>([]);
	let proofsError = $state('');
	let proofsLoading = $state(false);

	onMount(async () => {
		try {
			metrics = await adminApi.getAdminMetrics();
			const { users } = await adminApi.getAdminUsers({ limit: 8 });
			recentUsers = users;
			try {
				pendingProofs = await adminApi.getPaymentProofs({ status: 'pending', limit: 10 });
			} catch (e) {
				proofsError =
					e instanceof Error ? e.message : 'Failed to load payment proofs. Backend not ready?';
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load dashboard';
		} finally {
			loading = false;
		}
	});

	function formatCurrency(amount: number, currency: string): string {
		if (currency === 'NGN') return '₦' + amount.toLocaleString();
		return `${currency} ${amount.toLocaleString()}`;
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString(undefined, {
			day: 'numeric',
			month: 'short',
			year: 'numeric'
		});
	}

	async function handleReviewProof(id: string, status: 'approved' | 'rejected') {
		if (proofsLoading) return;
		proofsLoading = true;
		proofsError = '';
		try {
			await adminApi.reviewPaymentProof(id, status);
			pendingProofs = pendingProofs.filter((p) => p.id !== id);
		} catch (e) {
			proofsError =
				e instanceof Error ? e.message : 'Failed to update proof status. Please try again.';
		} finally {
			proofsLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Dashboard — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-7xl">
	<header class="mb-8">
		<h1 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">
			Admin dashboard
		</h1>
		<p class="mt-1 text-slate-400">
			Monitor registrations, payments, and platform metrics
		</p>
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
	{:else if metrics}
		<!-- Metric cards -->
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
			<div
				class="rounded-xl border border-slate-800 bg-slate-900/80 p-5 shadow-lg backdrop-blur"
			>
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-slate-400">Total registrations</p>
						<p class="mt-1 text-2xl font-bold text-white">{metrics.totalRegistrations}</p>
						<p class="mt-0.5 text-xs text-slate-500">
							+{metrics.registrationsThisMonth} this month
						</p>
					</div>
					<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-pink-500/20">
						<svg class="h-6 w-6 text-pink-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
						</svg>
					</div>
				</div>
			</div>

			<div
				class="rounded-xl border border-slate-800 bg-slate-900/80 p-5 shadow-lg backdrop-blur"
			>
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-slate-400">Verified</p>
						<p class="mt-1 text-2xl font-bold text-white">{metrics.verifiedCount}</p>
						<p class="mt-0.5 text-xs text-slate-500">
							{metrics.totalRegistrations > 0
								? Math.round((metrics.verifiedCount / metrics.totalRegistrations) * 100)
								: 0}% of total
						</p>
					</div>
					<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-emerald-500/20">
						<svg class="h-6 w-6 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
					</div>
				</div>
			</div>

			<div
				class="rounded-xl border border-slate-800 bg-slate-900/80 p-5 shadow-lg backdrop-blur"
			>
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-slate-400">Paid</p>
						<p class="mt-1 text-2xl font-bold text-white">{metrics.paidCount}</p>
						<p class="mt-0.5 text-xs text-slate-500">
							{metrics.pendingPaymentCount} pending
						</p>
					</div>
					<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-amber-500/20">
						<svg class="h-6 w-6 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
					</div>
				</div>
			</div>

			<div
				class="rounded-xl border border-slate-800 bg-slate-900/80 p-5 shadow-lg backdrop-blur"
			>
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-slate-400">Revenue</p>
						<p class="mt-1 text-2xl font-bold text-white">
							{formatCurrency(metrics.revenueTotal, metrics.currency)}
						</p>
						<p class="mt-0.5 text-xs text-slate-500">
							Completion rate {metrics.completionRate}%
						</p>
					</div>
					<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-violet-500/20">
						<svg class="h-6 w-6 text-violet-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
						</svg>
					</div>
				</div>
			</div>
		</div>

		<!-- Success metrics bar -->
		<div class="mt-6 rounded-xl border border-slate-800 bg-slate-900/80 p-5">
			<h2 class="text-sm font-semibold text-slate-300">Success metrics</h2>
			<div class="mt-4 flex flex-wrap gap-6">
				<div>
					<p class="text-xs text-slate-500">Verification rate</p>
					<div class="mt-1 h-2 w-48 overflow-hidden rounded-full bg-slate-800">
						<div
							class="h-full rounded-full bg-pink-500"
							style="width: {metrics.totalRegistrations > 0 ? (metrics.verifiedCount / metrics.totalRegistrations) * 100 : 0}%"
						></div>
					</div>
					<p class="mt-0.5 text-sm text-slate-400">
						{metrics.totalRegistrations > 0
							? Math.round((metrics.verifiedCount / metrics.totalRegistrations) * 100)
							: 0}%
					</p>
				</div>
				<div>
					<p class="text-xs text-slate-500">Payment conversion</p>
					<div class="mt-1 h-2 w-48 overflow-hidden rounded-full bg-slate-800">
						<div
							class="h-full rounded-full bg-emerald-500"
							style="width: {metrics.totalRegistrations > 0 ? (metrics.paidCount / metrics.totalRegistrations) * 100 : 0}%"
						></div>
					</div>
					<p class="mt-0.5 text-sm text-slate-400">
						{metrics.totalRegistrations > 0
							? Math.round((metrics.paidCount / metrics.totalRegistrations) * 100)
							: 0}%
					</p>
				</div>
				<div>
					<p class="text-xs text-slate-500">Completion rate</p>
					<div class="mt-1 h-2 w-48 overflow-hidden rounded-full bg-slate-800">
						<div
							class="h-full rounded-full bg-violet-500"
							style="width: {metrics.completionRate}%"
						></div>
					</div>
					<p class="mt-0.5 text-sm text-slate-400">{metrics.completionRate}%</p>
				</div>
			</div>
		</div>

		<!-- Recent registrations -->
		<div class="mt-8 rounded-xl border border-slate-800 bg-slate-900/80">
			<div class="flex items-center justify-between border-b border-slate-800 px-5 py-4">
				<h2 class="text-lg font-semibold text-white">Recent registrations</h2>
				<a
					href="/admin/users"
					class="text-sm font-medium text-pink-400 hover:text-pink-300"
				>
					View all →
				</a>
			</div>
			<div class="overflow-x-auto">
				<table class="w-full min-w-[640px]">
					<thead>
						<tr class="border-b border-slate-800 text-left text-xs font-medium uppercase tracking-wider text-slate-500">
							<th class="px-5 py-3">Name</th>
							<th class="px-5 py-3">Email</th>
							<th class="px-5 py-3">Church</th>
							<th class="px-5 py-3">Status</th>
							<th class="px-5 py-3">Registered</th>
						</tr>
					</thead>
					<tbody>
						{#each recentUsers as u}
							<tr class="border-b border-slate-800/80 transition hover:bg-slate-800/50">
								<td class="px-5 py-3 font-medium text-white">{u.name}</td>
								<td class="px-5 py-3 text-slate-400">{u.email}</td>
								<td class="px-5 py-3 text-slate-400">{u.churchName}</td>
								<td class="px-5 py-3">
									<span class="inline-flex items-center gap-1.5">
										<span
											class="inline-block h-2 w-2 rounded-full {u.verified ? 'bg-emerald-500' : 'bg-amber-500'}"
										></span>
										{u.verified ? 'Verified' : 'Pending'}
										<span class="text-slate-600">·</span>
										{u.paymentComplete ? 'Paid' : 'Unpaid'}
									</span>
								</td>
								<td class="px-5 py-3 text-slate-500">{formatDate(u.createdAt)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			{#if recentUsers.length === 0}
				<div class="px-5 py-12 text-center text-slate-500">No registrations yet.</div>
			{/if}
		</div>

		<!-- Payment proofs (POP) -->
		<div class="mt-8 rounded-xl border border-slate-800 bg-slate-900/80">
			<div class="flex items-center justify-between border-b border-slate-800 px-5 py-4">
				<h2 class="text-lg font-semibold text-white">Payment proofs</h2>
				<p class="text-xs text-slate-500">
					Review student uploads and mark payments as approved or rejected.
				</p>
			</div>
			{#if proofsError}
				<div class="border-b border-slate-800 bg-red-900/20 px-5 py-3 text-xs text-red-300">
					{proofsError}
				</div>
			{/if}
			<div class="overflow-x-auto">
				<table class="w-full min-w-[720px]">
					<thead>
						<tr class="border-b border-slate-800 text-left text-xs font-medium uppercase tracking-wider text-slate-500">
							<th class="px-5 py-3">Student</th>
							<th class="px-5 py-3">Contact</th>
							<th class="px-5 py-3">Note</th>
							<th class="px-5 py-3">Uploaded</th>
							<th class="px-5 py-3">Actions</th>
						</tr>
					</thead>
					<tbody>
						{#if pendingProofs.length === 0}
							<tr>
								<td
									class="px-5 py-8 text-center text-slate-500"
									colspan="5"
								>
									No pending payment proofs.
								</td>
							</tr>
						{:else}
							{#each pendingProofs as proof}
								<tr class="border-b border-slate-800/80 transition hover:bg-slate-800/50">
									<td class="px-5 py-3 text-sm text-slate-100">
										<div class="font-medium">{proof.userName}</div>
										<div class="text-xs text-slate-500">{proof.userEmail}</div>
									</td>
									<td class="px-5 py-3 text-xs text-slate-400">
										<div class="truncate">{proof.userEmail}</div>
										{#if proof.userPhone}
											<div class="mt-1 truncate">{proof.userPhone}</div>
										{/if}
									</td>
									<td class="px-5 py-3 text-xs text-slate-300 max-w-xs">
										{proof.note || '—'}
									</td>
									<td class="px-5 py-3 text-xs text-slate-400">
										{new Date(proof.uploadedAt).toLocaleString(undefined, {
											day: 'numeric',
											month: 'short',
											hour: '2-digit',
											minute: '2-digit'
										})}
									</td>
									<td class="px-5 py-3 text-xs">
										<div class="flex flex-wrap gap-2">
											<a
												href={proof.fileUrl}
												target="_blank"
												rel="noopener noreferrer"
												class="inline-flex items-center gap-1 rounded-full border border-slate-700 px-3 py-1 text-[11px] font-medium text-slate-200 hover:border-slate-500"
											>
												View file
											</a>
											<button
												type="button"
												class="inline-flex items-center gap-1 rounded-full bg-emerald-500/10 px-3 py-1 text-[11px] font-semibold text-emerald-300 hover:bg-emerald-500/20 disabled:opacity-50"
												disabled={proofsLoading}
												onclick={() => handleReviewProof(proof.id, 'approved')}
											>
												Approve
											</button>
											<button
												type="button"
												class="inline-flex items-center gap-1 rounded-full bg-red-500/10 px-3 py-1 text-[11px] font-semibold text-red-300 hover:bg-red-500/20 disabled:opacity-50"
												disabled={proofsLoading}
												onclick={() => handleReviewProof(proof.id, 'rejected')}
											>
												Reject
											</button>
										</div>
									</td>
								</tr>
							{/each}
						{/if}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
</div>
