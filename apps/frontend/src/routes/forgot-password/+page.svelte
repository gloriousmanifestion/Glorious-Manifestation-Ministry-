<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { SITE } from '$lib/config';
	import { auth } from '$lib/auth';
	import { pageSeo } from '$lib/stores/pageSeo';

	pageSeo.set({
		title: `Forgot password — ${SITE.name} Pastoral School`,
		description: 'Request a password reset link for your Pastoral School account.'
	});

	let email = $state('');
	let error = $state('');
	let success = $state('');
	let loading = $state(false);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (loading) return;
		error = '';
		success = '';
		const trimmed = email.trim();
		if (!trimmed) {
			error = 'Please enter your email address.';
			return;
		}
		loading = true;
		try {
			await auth.requestPasswordReset(trimmed);
			success =
				'If an account exists for that email, a password reset link has been sent to your inbox.';
		} catch (e) {
			error =
				e instanceof Error ? e.message : 'Could not request password reset. Please try again.';
		} finally {
			loading = false;
		}
	}
</script>

<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
	<Nav />

	<main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-md px-4 sm:px-6">
			<div
				class="rounded-2xl border border-pink-200/50 card-premium bg-white p-6 sm:p-8 shadow-xl dark:bg-[#1e2642]"
			>
				<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">
					Forgot your password?
				</h1>
				<p class="mt-2 text-sm text-slate-600 dark:text-slate-100">
					Enter the email associated with your account and we'll send you a link to reset your
					password.
				</p>

				<form class="mt-8 space-y-6" onsubmit={handleSubmit}>
					{#if error}
						<div
							class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700 dark:border-red-800 dark:bg-red-900/30 dark:text-red-200"
						>
							{error}
						</div>
					{/if}
					{#if success}
						<div
							class="rounded-xl border border-emerald-200 bg-emerald-50 p-4 text-sm text-emerald-700 dark:border-emerald-800 dark:bg-emerald-900/30 dark:text-emerald-200"
						>
							{success}
						</div>
					{/if}

					<div>
						<label
							for="email"
							class="block text-sm font-medium text-slate-700 dark:text-slate-100"
						>
							Email
						</label>
						<input
							id="email"
							type="email"
							bind:value={email}
							required
							class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400"
							placeholder="you@example.com"
						/>
					</div>

					<button
						type="submit"
						disabled={loading}
						class="w-full rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 disabled:opacity-60"
					>
						{loading ? 'Sending link…' : 'Send reset link'}
					</button>
				</form>

				<p class="mt-6 text-center text-sm text-slate-600 dark:text-slate-200">
					Remembered your password?
					<a
						href="/login"
						class="font-medium text-pink-600 hover:text-pink-500 dark:text-pink-400 dark:hover:text-pink-300"
					>
						Back to login
					</a>
				</p>
			</div>
		</div>
	</main>

	<Footer />
</div>

