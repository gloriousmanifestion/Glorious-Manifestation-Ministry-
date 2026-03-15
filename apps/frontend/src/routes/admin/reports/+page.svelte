<script lang="ts">
	import { onMount } from 'svelte';
	import { downloadRegistrationsCsv, getAuditLog, type AuditLogEntry } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let loading = $state(false);
	let error = $state('');
	let auditLog = $state<AuditLogEntry[]>([]);
	let auditLoading = $state(true);
	let auditError = $state('');

	onMount(loadAuditLog);

	async function loadAuditLog() {
		auditLoading = true;
		auditError = '';
		try {
			auditLog = await getAuditLog({ limit: 50 });
		} catch (e) {
			auditError = e instanceof Error ? e.message : 'Failed to load audit log';
			auditLog = [];
		} finally {
			auditLoading = false;
		}
	}

	async function handleDownloadCsv() {
		if (loading) return;
		error = '';
		loading = true;
		try {
			await downloadRegistrationsCsv();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Download failed';
		} finally {
			loading = false;
		}
	}

	function formatAuditTime(iso: string): string {
		return new Date(iso).toLocaleString(undefined, {
			day: 'numeric',
			month: 'short',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function actionLabel(action: string): string {
		if (action === 'bulk_verify') return 'Bulk verify';
		if (action === 'bulk_mark_paid') return 'Bulk mark paid';
		return action;
	}
</script>

<svelte:head>
	<title>Reports — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-6xl space-y-8">
	<h1 class="text-2xl font-bold text-white">Reports</h1>
	<p class="text-slate-400">
		Export data and view audit log of admin actions.
	</p>

	<div class="rounded-xl border border-slate-700 bg-slate-800/50 p-6">
		<h2 class="text-lg font-medium text-white">Registrations</h2>
		<p class="mt-1 text-sm text-slate-400">
			Download all student registrations (id, email, name, phone, ministry role, church, verified, payment status, course, created date) as a CSV file.
		</p>
		<div class="mt-4">
			<button
				type="button"
				disabled={loading}
				onclick={handleDownloadCsv}
				class="inline-flex items-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-pink-500 disabled:opacity-50"
			>
				{loading ? 'Downloading…' : 'Download registrations (CSV)'}
			</button>
		</div>
		{#if error}
			<p class="mt-2 text-sm text-red-400">{error}</p>
		{/if}
	</div>

	<div class="rounded-xl border border-slate-700 bg-slate-800/50 p-6">
		<h2 class="text-lg font-medium text-white">Audit log</h2>
		<p class="mt-1 text-sm text-slate-400">
			Recent admin actions (bulk verify, bulk mark paid, etc.). Most recent first.
		</p>
		{#if auditLoading}
			<div class="mt-4 flex justify-center py-8">
				<div class="h-8 w-8 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
			</div>
		{:else if auditError}
			<p class="mt-4 text-sm text-red-400">{auditError}</p>
		{:else if auditLog.length === 0}
			<p class="mt-4 text-slate-500">No audit entries yet.</p>
		{:else}
			<div class="mt-4 overflow-x-auto">
				<table class="w-full text-left text-sm">
					<thead>
						<tr class="border-b border-slate-700 text-xs font-medium uppercase tracking-wider text-slate-500">
							<th class="px-4 py-3">Time</th>
							<th class="px-4 py-3">Action</th>
							<th class="px-4 py-3">Target</th>
							<th class="px-4 py-3">Details</th>
						</tr>
					</thead>
					<tbody>
						{#each auditLog as entry}
							<tr class="border-b border-slate-700/80 text-slate-300">
								<td class="px-4 py-3 text-slate-400">{formatAuditTime(entry.created_at)}</td>
								<td class="px-4 py-3 font-medium text-white">{actionLabel(entry.action)}</td>
								<td class="px-4 py-3">
									{entry.target_type ?? '—'}
									{#if entry.target_id}
										<span class="text-slate-500"> · {entry.target_id}</span>
									{/if}
								</td>
								<td class="max-w-xs truncate px-4 py-3 text-slate-500" title={entry.details ?? ''}>
									{entry.details ?? '—'}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>
</div>
