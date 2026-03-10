<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { auth } from '$lib/auth';
	import { SITE } from '$lib/config';

	let email = $state('');
	let password = $state('');
	let error = $state('');
	let loading = $state(false);

	const redirectTo = $derived((() => {
		const r = $page.url.searchParams.get('redirect');
		if (!r || !r.startsWith('/')) return '/dashboard';
		return r;
	})());

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (loading) return;
		error = '';
		loading = true;
		try {
			await auth.login(email, password);
			goto(redirectTo);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Login failed. Please try again.';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Log in — {SITE.name} Pastoral School</title>
	<meta name="description" content="Log in to your Pastoral School dashboard." />
</svelte:head>

<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
	<Nav />

	<main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-md px-4 sm:px-6">
			<div class="rounded-2xl border border-pink-200/50 card-premium bg-white p-6 sm:p-8 shadow-xl dark:bg-[#1e2642]">
				<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">Log in</h1>
				<p class="mt-2 text-slate-600 dark:text-slate-100">
					Sign in to access your Pastoral School dashboard.
				</p>

				<form onsubmit={handleSubmit} class="mt-8 space-y-6">
					{#if error}
						<div class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700 dark:border-red-800 dark:bg-red-900/30 dark:text-red-200">
							{error}
						</div>
					{/if}
					<div>
						<label for="email" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Email</label>
						<input
							id="email"
							type="email"
							bind:value={email}
							required
							class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
							placeholder="you@example.com"
						/>
					</div>
					<div>
						<label for="password" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Password</label>
						<input
							id="password"
							type="password"
							bind:value={password}
							required
							class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
							placeholder="Your password"
						/>
					</div>
					<button
						type="submit"
						disabled={loading}
						class="w-full rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 disabled:opacity-60"
					>
						{loading ? 'Signing in…' : 'Sign in'}
					</button>
				</form>

				<p class="mt-6 text-center text-sm text-slate-600 dark:text-slate-200">
					Don't have an account?
					<a href="/register" class="font-medium text-pink-600 hover:text-pink-500 dark:text-pink-400 dark:hover:text-pink-300"> Register</a>
				</p>
			</div>
		</div>
	</main>

	<Footer />
</div>
