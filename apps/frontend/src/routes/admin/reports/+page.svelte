<script lang="ts">
	import { downloadRegistrationsCsv } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let loading = $state(false);
	let error = $state('');

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
</script>

<svelte:head>
	<title>Reports — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="space-y-6">
	<h1 class="text-2xl font-semibold text-slate-900 dark:text-white">Reports</h1>
	<p class="text-slate-600 dark:text-slate-100">
		Export data for use in spreadsheets or external tools.
	</p>

	<div
		class="rounded-xl border border-slate-200 bg-white p-6 shadow-sm dark:border-slate-700 dark:bg-slate-800/50"
	>
		<h2 class="text-lg font-medium text-slate-900 dark:text-white">Registrations</h2>
		<p class="mt-1 text-sm text-slate-600 dark:text-slate-100">
			Download all student registrations (id, email, name, phone, ministry role, church, verified, payment status, course, created date) as a CSV file.
		</p>
		<div class="mt-4">
			<button
				type="button"
				disabled={loading}
				onclick={handleDownloadCsv}
				class="inline-flex items-center rounded-lg bg-pink-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-pink-700 disabled:opacity-50 dark:bg-pink-500 dark:hover:bg-pink-600"
			>
				{loading ? 'Downloading…' : 'Download registrations (CSV)'}
			</button>
		</div>
		{#if error}
			<p class="mt-2 text-sm text-red-600 dark:text-red-400">{error}</p>
		{/if}
	</div>
</div>
