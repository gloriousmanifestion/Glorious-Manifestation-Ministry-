<script lang="ts">
	import { onMount } from 'svelte';
	import {
		createNotifications,
		downloadRegistrationsCsv,
		emailStudents,
		getAuditLog,
		type AuditLogEntry
	} from '$lib/admin-api';
	import { SITE } from '$lib/config';

	let loading = $state(false);
	let error = $state('');
	let auditLog = $state<AuditLogEntry[]>([]);
	let auditLoading = $state(true);
	let auditError = $state('');
	let notifTitle = $state('');
	let notifMessage = $state('');
	let notifUserIds = $state('');
	let notifSending = $state(false);
	let notifResult = $state('');
	let emailSubject = $state('');
	let emailBody = $state('');
	let emailUserIds = $state('');
	let emailSending = $state(false);
	let emailResult = $state('');

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

	function parseUserIds(raw: string): string[] | null {
		const s = raw.trim();
		if (!s) return null;
		return s.split(/[\s,]+/).filter((x) => x.length > 0);
	}

	async function handleCreateNotifications() {
		if (notifSending || !notifTitle.trim() || !notifMessage.trim()) return;
		notifSending = true;
		notifResult = '';
		try {
			const res = await createNotifications({
				title: notifTitle.trim(),
				message: notifMessage.trim(),
				user_ids: parseUserIds(notifUserIds)
			});
			notifResult = `Created ${res.created} notification(s).`;
			notifTitle = '';
			notifMessage = '';
			notifUserIds = '';
		} catch (e) {
			notifResult = e instanceof Error ? e.message : 'Failed';
		} finally {
			notifSending = false;
		}
	}

	async function handleEmailStudents() {
		if (emailSending || !emailSubject.trim() || !emailBody.trim()) return;
		emailSending = true;
		emailResult = '';
		try {
			const res = await emailStudents({
				subject: emailSubject.trim(),
				body: emailBody.trim(),
				user_ids: parseUserIds(emailUserIds)
			});
			emailResult = `Sent: ${res.sent}, failed: ${res.failed}.`;
			emailSubject = '';
			emailBody = '';
			emailUserIds = '';
		} catch (e) {
			emailResult = e instanceof Error ? e.message : 'Failed';
		} finally {
			emailSending = false;
		}
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
		<h2 class="text-lg font-medium text-white">Create notification</h2>
		<p class="mt-1 text-sm text-slate-400">
			Send an in-app notification to all students (or paste user IDs, comma-separated, to target specific users).
		</p>
		<form class="mt-4 space-y-3" onsubmit={(e) => { e.preventDefault(); handleCreateNotifications(); }}>
			<div>
				<label for="notif-title" class="block text-sm font-medium text-slate-300">Title</label>
				<input
					id="notif-title"
					type="text"
					bind:value={notifTitle}
					class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
					placeholder="Notification title"
				/>
			</div>
			<div>
				<label for="notif-message" class="block text-sm font-medium text-slate-300">Message</label>
				<textarea
					id="notif-message"
					bind:value={notifMessage}
					rows="3"
					class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
					placeholder="Message body"
				></textarea>
			</div>
			<div>
				<label for="notif-ids" class="block text-sm font-medium text-slate-300">User IDs (optional)</label>
				<input
					id="notif-ids"
					type="text"
					bind:value={notifUserIds}
					class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
					placeholder="Leave blank for all students"
				/>
			</div>
			<button
				type="submit"
				disabled={notifSending || !notifTitle.trim() || !notifMessage.trim()}
				class="rounded-lg bg-pink-600 px-4 py-2 text-sm font-medium text-white hover:bg-pink-500 disabled:opacity-50"
			>
				{notifSending ? 'Sending…' : 'Create notification'}
			</button>
			{#if notifResult}
				<p class="text-sm text-slate-400">{notifResult}</p>
			{/if}
		</form>
	</div>

	<div class="rounded-xl border border-slate-700 bg-slate-800/50 p-6">
		<h2 class="text-lg font-medium text-white">Email students</h2>
		<p class="mt-1 text-sm text-slate-400">
			Send an email to all students (or paste user IDs to target specific users). Requires Azure/Microsoft Graph email configuration.
		</p>
		<form class="mt-4 space-y-3" onsubmit={(e) => { e.preventDefault(); handleEmailStudents(); }}>
			<div>
				<label for="email-subject" class="block text-sm font-medium text-slate-300">Subject</label>
				<input
					id="email-subject"
					type="text"
					bind:value={emailSubject}
					class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
					placeholder="Email subject"
				/>
			</div>
			<div>
				<label for="email-body" class="block text-sm font-medium text-slate-300">Body</label>
				<textarea
					id="email-body"
					bind:value={emailBody}
					rows="4"
					class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
					placeholder="Plain text email body"
				></textarea>
			</div>
			<div>
				<label for="email-ids" class="block text-sm font-medium text-slate-300">User IDs (optional)</label>
				<input
					id="email-ids"
					type="text"
					bind:value={emailUserIds}
					class="mt-1 w-full rounded-lg border border-slate-600 bg-slate-800 px-3 py-2 text-white placeholder-slate-500"
					placeholder="Leave blank for all students"
				/>
			</div>
			<button
				type="submit"
				disabled={emailSending || !emailSubject.trim() || !emailBody.trim()}
				class="rounded-lg bg-pink-600 px-4 py-2 text-sm font-medium text-white hover:bg-pink-500 disabled:opacity-50"
			>
				{emailSending ? 'Sending…' : 'Send email'}
			</button>
			{#if emailResult}
				<p class="text-sm text-slate-400">{emailResult}</p>
			{/if}
		</form>
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
