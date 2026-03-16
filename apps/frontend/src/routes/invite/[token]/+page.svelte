<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { getInvite } from '$lib/auth-api';
	import type { InviteInfo } from '$lib/auth-api';
	import { SITE } from '$lib/config';
	import { pageSeo } from '$lib/stores/pageSeo';

	pageSeo.set({
		title: `Invitation — ${SITE.schoolName} Admin`,
		description: 'Admin invitation to Pastoral School.',
		noIndex: true
	});

	interface Props {
		data: { token: string };
	}
	let { data }: Props = $props();

	let invite = $state<InviteInfo | null>(null);
	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		try {
			invite = await getInvite(data.token);
			if (!invite) error = 'This invitation link is invalid or has expired.';
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not load invitation.';
		} finally {
			loading = false;
		}
	});

	function acceptInvite() {
		goto(`/register?invite=${encodeURIComponent(data.token)}`);
	}
</script>

<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
	<Nav />

	<main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-lg px-4 sm:px-6">
			{#if loading}
				<div class="flex flex-col items-center justify-center rounded-2xl border border-pink-200/50 bg-white py-16 dark:bg-[#1e2642]">
					<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
					<p class="mt-4 text-slate-600 dark:text-slate-300">Checking invitation…</p>
				</div>
			{:else if error}
				<div class="rounded-2xl border border-red-200 bg-red-50 p-6 text-center dark:border-red-800 dark:bg-red-900/20">
					<p class="font-medium text-red-800 dark:text-red-200">{error}</p>
					<a
						href="/"
						class="mt-4 inline-block text-sm font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400"
					>
						Return to home
					</a>
				</div>
			{:else if invite}
				<div class="rounded-2xl border border-pink-200/50 card-premium bg-white p-6 shadow-xl dark:bg-[#1e2642] sm:p-8">
					<div class="text-center">
						<div class="mx-auto flex h-14 w-14 items-center justify-center rounded-full bg-pink-100 dark:bg-pink-900/50">
							<svg class="h-7 w-7 text-pink-600 dark:text-pink-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
							</svg>
						</div>
						<h1 class="mt-4 text-2xl font-bold tracking-tight text-slate-900 dark:text-white">
							You're invited
						</h1>
						<p class="mt-2 text-slate-600 dark:text-slate-300">
							You have been invited to join the <strong>{SITE.schoolName}</strong> platform as
							<strong class="text-pink-600 dark:text-pink-400">
								{invite.role === 'super_admin' ? 'Super Admin' : 'Admin'}
							</strong>.
						</p>
						<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
							{invite.role === 'super_admin'
								? 'Create your account to set up the platform and invite other admins.'
								: 'Create your account to monitor registrations, manage courses, and use the admin dashboard.'}
						</p>
					</div>
					<div class="mt-8">
						<button
							type="button"
							onclick={acceptInvite}
							class="w-full rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3.5 text-base font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700"
						>
							Create your account
						</button>
						<p class="mt-3 text-center text-sm text-slate-500 dark:text-slate-400">
							You will be taken to the registration form to set your name, email, and password.
						</p>
					</div>
				</div>
			{/if}
		</div>
	</main>

	<Footer />
</div>
