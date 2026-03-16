<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { auth, userStore, authReady } from '$lib/auth';
	import { SITE } from '$lib/config';

	let mounted = $state(false);
	let form = $state({ name: '', phone: '', ministryRole: '', churchName: '', calling: '' });
	let saving = $state(false);
	let error = $state('');
	let success = $state(false);

	$effect(() => {
		if (!mounted) return;
		const ready = $authReady;
		const user = $userStore;
		if (ready && !user) {
			goto('/register');
		}
	});

	onMount(() => {
		mounted = true;
		const user = $userStore;
		if (user) {
			form = {
				name: user.name,
				phone: user.phone,
				ministryRole: user.ministryRole,
				churchName: user.churchName,
				calling: user.calling
			};
		}
	});

	$effect(() => {
		const user = $userStore;
		if (user && mounted) {
			form = {
				name: user.name,
				phone: user.phone,
				ministryRole: user.ministryRole,
				churchName: user.churchName,
				calling: user.calling
			};
		}
	});

	const user = $derived($userStore);

	async function save() {
		saving = true;
		error = '';
		success = false;
		try {
			await auth.updateProfile({
				name: form.name.trim(),
				phone: form.phone.trim(),
				ministryRole: form.ministryRole.trim(),
				churchName: form.churchName.trim(),
				calling: form.calling.trim()
			});
			success = true;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update profile';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>Profile — {SITE.name} Pastoral School</title>
	<meta name="description" content="Edit your profile." />
</svelte:head>

{#if !mounted || !$authReady}
	<div class="flex min-h-screen items-center justify-center bg-slate-50 dark:bg-[#252d47]">
		<div class="h-8 w-8 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
	</div>
{:else if user}
	<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
		<Nav />

		<main class="flex-1 pt-16 sm:pt-24 pb-12 sm:pb-16">
			<div class="mx-auto max-w-2xl px-4 sm:px-6">
				<div class="mb-6">
					<a
						href="/dashboard"
						class="text-sm font-medium text-slate-500 transition hover:text-pink-600 dark:text-slate-400 dark:hover:text-pink-400"
					>
						← Back to dashboard
					</a>
				</div>
				<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-3xl">
					Your profile
				</h1>
				<p class="mt-1 text-slate-600 dark:text-slate-200">
					Update your name, contact, and ministry details.
				</p>

				{#if error}
					<div
						class="mt-4 rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700 dark:border-red-800 dark:bg-red-900/30 dark:text-red-200"
					>
						{error}
					</div>
				{/if}
				{#if success}
					<div
						class="mt-4 rounded-xl border border-green-200 bg-green-50 p-4 text-sm text-green-700 dark:border-green-800 dark:bg-green-900/30 dark:text-green-200"
					>
						Profile saved.
					</div>
				{/if}

				<form
					onsubmit={(e) => {
						e.preventDefault();
						save();
					}}
					class="mt-6 space-y-4 rounded-2xl border border-slate-200/80 section-separator card-premium bg-white/95 p-4 sm:p-6 shadow-sm dark:bg-[#1e2642]"
				>
					<div>
						<label for="profile-email" class="block text-sm font-medium text-slate-700 dark:text-slate-200">
							Email
						</label>
						<input
							id="profile-email"
							type="email"
							value={user.email}
							disabled
							class="mt-1 block w-full rounded-xl border border-slate-200 bg-slate-100 px-3 py-2 text-slate-500 dark:border-white/10 dark:bg-slate-800/50 dark:text-slate-400"
						/>
						<p class="mt-1 text-xs text-slate-500 dark:text-slate-400">Email cannot be changed here.</p>
					</div>
					<div>
						<label for="profile-name" class="block text-sm font-medium text-slate-700 dark:text-slate-200">
							Name
						</label>
						<input
							id="profile-name"
							type="text"
							bind:value={form.name}
							class="mt-1 block w-full rounded-xl border border-slate-200 px-3 py-2 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white"
						/>
					</div>
					<div>
						<label for="profile-phone" class="block text-sm font-medium text-slate-700 dark:text-slate-200">
							Phone
						</label>
						<input
							id="profile-phone"
							type="text"
							bind:value={form.phone}
							class="mt-1 block w-full rounded-xl border border-slate-200 px-3 py-2 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white"
						/>
					</div>
					<div>
						<label
							for="profile-ministry"
							class="block text-sm font-medium text-slate-700 dark:text-slate-200"
						>
							Ministry role
						</label>
						<input
							id="profile-ministry"
							type="text"
							bind:value={form.ministryRole}
							placeholder="e.g. Pastor, Minister, Prophet"
							class="mt-1 block w-full rounded-xl border border-slate-200 px-3 py-2 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
						/>
					</div>
					<div>
						<label for="profile-church" class="block text-sm font-medium text-slate-700 dark:text-slate-200">
							Church
						</label>
						<input
							id="profile-church"
							type="text"
							bind:value={form.churchName}
							placeholder="Church or ministry name"
							class="mt-1 block w-full rounded-xl border border-slate-200 px-3 py-2 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
						/>
					</div>
					<div>
						<label for="profile-calling" class="block text-sm font-medium text-slate-700 dark:text-slate-200">
							Calling
						</label>
						<textarea
							id="profile-calling"
							bind:value={form.calling}
							rows="3"
							placeholder="Brief description of your calling or ministry focus"
							class="mt-1 block w-full rounded-xl border border-slate-200 px-3 py-2 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
						></textarea>
					</div>
					<div class="flex gap-3 pt-2">
						<button
							type="submit"
							disabled={saving}
							class="btn-scale rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-2.5 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 disabled:opacity-60"
						>
							{saving ? 'Saving…' : 'Save profile'}
						</button>
						<a
							href="/dashboard"
							class="rounded-xl border border-slate-200 px-6 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50 dark:border-white/10 dark:text-slate-200 dark:hover:bg-white/5"
						>
							Cancel
						</a>
					</div>
				</form>
			</div>
		</main>

		<Footer />
	</div>
{/if}
