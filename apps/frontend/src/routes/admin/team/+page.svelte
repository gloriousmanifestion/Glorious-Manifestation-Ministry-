<script lang="ts">
	import { goto } from '$app/navigation';
	import { userStore } from '$lib/auth';
	import * as adminApi from '$lib/admin-api';
	import type { InviteResult } from '$lib/admin-api';
	import { SITE } from '$lib/config';

	const user = $derived($userStore);
	let createdInvite = $state<InviteResult | null>(null);
	let loading = $state(false);
	let error = $state('');

	$effect(() => {
		if (user && user.role !== 'super_admin') {
			goto('/admin');
		}
	});

	async function createInvite(role: 'admin' | 'super_admin') {
		loading = true;
		error = '';
		createdInvite = null;
		try {
			createdInvite = await adminApi.createInvite(role);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to create invite link.';
		} finally {
			loading = false;
		}
	}

	function copyLink() {
		if (!createdInvite?.link) return;
		navigator.clipboard.writeText(createdInvite.link);
	}

	function formatExpires(iso: string): string {
		return new Date(iso).toLocaleString(undefined, {
			dateStyle: 'medium',
			timeStyle: 'short'
		});
	}
</script>

<svelte:head>
	<title>Team & invites — Admin — {SITE.schoolName}</title>
</svelte:head>

<div class="mx-auto max-w-2xl">
	<header class="mb-8">
		<h1 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">Team & invites</h1>
		<p class="mt-1 text-slate-400">
			As a Super Admin, you can invite other admins to help monitor and manage the platform.
		</p>
	</header>

	{#if error}
		<div class="mb-6 rounded-xl border border-red-800/50 bg-red-900/20 px-4 py-3 text-red-300">
			{error}
		</div>
	{/if}

	<div class="rounded-xl border border-slate-800 bg-slate-900/80 p-6">
		<h2 class="text-lg font-semibold text-white">Create invitation link</h2>
		<p class="mt-1 text-sm text-slate-400">
			Generate a link to invite someone to join as an Admin. They will register and get access to the admin dashboard.
		</p>
		<div class="mt-4 flex flex-wrap gap-3">
			<button
				type="button"
				disabled={loading}
				onclick={() => createInvite('admin')}
				class="rounded-lg bg-pink-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-pink-500 disabled:opacity-60"
			>
				{loading ? 'Creating…' : 'Invite as Admin'}
			</button>
		</div>
	</div>

	{#if createdInvite}
		<div class="mt-6 rounded-xl border border-emerald-800/50 bg-emerald-900/20 p-6">
			<h3 class="font-semibold text-emerald-300">Invitation link created</h3>
			<p class="mt-1 text-sm text-slate-400">
				Share this link with the person you want to invite. They will sign up and get the <strong>{createdInvite.role === 'super_admin' ? 'Super Admin' : 'Admin'}</strong> role.
			</p>
			<div class="mt-4 flex flex-col gap-2 sm:flex-row sm:items-center">
				<input
					type="text"
					readonly
					value={createdInvite.link}
					class="flex-1 rounded-lg border border-slate-700 bg-slate-800 px-4 py-2.5 text-sm text-white"
				/>
				<button
					type="button"
					onclick={copyLink}
					class="rounded-lg border border-slate-600 bg-slate-800 px-4 py-2.5 text-sm font-medium text-slate-200 hover:bg-slate-700"
				>
					Copy link
				</button>
			</div>
			{#if createdInvite.expires_at}
				<p class="mt-2 text-xs text-slate-500">Expires: {formatExpires(createdInvite.expires_at)}</p>
			{/if}
		</div>
	{/if}

	<div class="mt-8 rounded-xl border border-slate-800 bg-slate-900/50 p-6">
		<h3 class="font-semibold text-slate-300">How it works</h3>
		<ul class="mt-2 list-inside list-disc space-y-1 text-sm text-slate-400">
			<li>Click &quot;Invite as Admin&quot; to generate a one-time invitation link.</li>
			<li>Share the link with the person you want to add as an admin.</li>
			<li>They open the link, then complete the registration form to create their account.</li>
			<li>After signing up, they can access the admin dashboard and help manage the platform.</li>
		</ul>
	</div>
</div>
