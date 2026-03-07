<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { auth, userStore, authReady, fetchCourses } from '$lib/auth';
	import type { Course } from '$lib/auth';
	import { SITE } from '$lib/config';
	import { getCourseDetailById } from '$lib/courses';

	let mounted = $state(false);
	let courses = $state<Course[]>([]);

	$effect(() => {
		if (!mounted) return;
		const ready = $authReady;
		const user = $userStore;
		if (ready && !user) {
			goto('/register');
		}
	});

	onMount(async () => {
		mounted = true;
		try {
			courses = await fetchCourses();
		} catch {
			// Fallback empty - user can still use dashboard
		}
	});

	const user = $derived($userStore);
	const showVerifyBanner = $derived(user && !user.verified);
	const showPaymentBanner = $derived(user && !user.paymentComplete);
	const selectedCourse = $derived(user?.selectedCourseId ? courses.find((c) => c.id === user.selectedCourseId) : null);

	let verifyLoading = $state(false);
	let actionError = $state('');

	async function handleVerify() {
		if (verifyLoading) return;
		verifyLoading = true;
		actionError = '';
		try {
			await auth.verifyAccount();
		} catch (e) {
			actionError = e instanceof Error ? e.message : 'Verification failed';
		} finally {
			verifyLoading = false;
		}
	}

	async function handleSelectCourse(courseId: string) {
		if (!courseId) return;
		actionError = '';
		try {
			await auth.selectCourse(courseId);
		} catch (e) {
			actionError = e instanceof Error ? e.message : 'Failed to update course';
		}
	}

</script>

<svelte:head>
	<title>Dashboard — {SITE.name} Pastoral School</title>
	<meta name="description" content="Your Pastoral School dashboard." />
</svelte:head>

{#if !mounted || !$authReady}
	<div class="flex min-h-screen items-center justify-center bg-slate-50 dark:bg-[#374266]">
		<div class="h-8 w-8 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
	</div>
{:else if user}
	<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#374266]">
		<Nav />

		<main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16">
			<div class="mx-auto max-w-6xl px-4 sm:px-6">
				<!-- Welcome header -->
				<div class="mb-6 sm:mb-8">
					<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-3xl">
						Welcome back, {user.name.split(' ')[0]}
					</h1>
					<p class="mt-1 text-slate-600 dark:text-slate-200">
						Manage your Pastoral School enrollment
					</p>
				</div>

				{#if actionError}
					<div class="mb-4 rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700 dark:border-red-800 dark:bg-red-900/30 dark:text-red-200">
						{actionError}
					</div>
				{/if}

				<!-- Pending actions banners -->
				<div class="mb-8 space-y-4">
					{#if showVerifyBanner}
						<div
							class="flex flex-col gap-4 rounded-2xl border-2 border-amber-200 card-premium bg-amber-50/80 p-4 sm:p-6 shadow-sm dark:border-amber-800/40 dark:bg-amber-900/20 sm:flex-row sm:items-center sm:justify-between"
						>
							<div class="flex items-start gap-4">
								<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-amber-100 dark:bg-amber-900/50">
									<svg class="h-6 w-6 text-amber-600 dark:text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
									</svg>
								</div>
								<div>
									<h3 class="font-semibold text-slate-900 dark:text-white">Verify your account</h3>
									<p class="mt-1 text-sm text-slate-600 dark:text-slate-200">
										Complete account verification to access all features. Check your email for the verification link.
									</p>
								</div>
							</div>
							<button
								type="button"
								onclick={handleVerify}
								disabled={verifyLoading}
								class="btn-scale shrink-0 rounded-xl bg-amber-500 px-6 py-2.5 text-sm font-semibold text-white transition hover:bg-amber-600 disabled:opacity-60"
							>
								{verifyLoading ? 'Verifying…' : 'Verify now'}
							</button>
						</div>
					{/if}

					{#if showPaymentBanner}
						<div
							class="flex flex-col gap-4 rounded-2xl border-2 border-pink-200 card-premium bg-gradient-to-r from-pink-50/80 to-purple-50/50 p-4 sm:p-6 shadow-sm dark:border-white/[0.1] dark:from-pink-900/20 dark:to-purple-900/20 sm:flex-row sm:items-center sm:justify-between"
						>
							<div class="flex items-start gap-4">
								<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-pink-100 dark:bg-pink-900/50">
									<svg class="h-6 w-6 text-pink-600 dark:text-pink-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
									</svg>
								</div>
								<div>
									<h3 class="font-semibold text-slate-900 dark:text-white">Complete your enrollment</h3>
									<p class="mt-1 text-sm text-slate-600 dark:text-slate-200">
										{#if selectedCourse}
											You've selected {selectedCourse.name}. Complete payment to secure your spot.
										{:else}
											Choose a course and complete payment to secure your spot in the program.
										{/if}
									</p>
								</div>
							</div>
							<div class="flex shrink-0 flex-col gap-2 sm:flex-row w-full sm:w-auto">
								{#if !selectedCourse}
									<select
										class="rounded-xl border border-pink-200 bg-white px-4 py-3 text-base sm:text-sm font-medium text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#374266] dark:text-white touch-manipulation"
										onchange={(e) => handleSelectCourse((e.target as HTMLSelectElement).value)}
									>
										<option value="">Choose course</option>
										{#each courses as course}
											<option value={course.id}>{course.name}</option>
										{/each}
									</select>
								{/if}
								<a
									href={SITE.selarPaymentUrl}
									target="_blank"
									rel="noopener noreferrer"
									class="btn-scale inline-flex items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 touch-manipulation"
								>
									{selectedCourse ? 'Pay via Selar' : 'Select & pay via Selar'}
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
									</svg>
								</a>
							</div>
						</div>
					{/if}
				</div>

				<!-- Dashboard cards -->
				<div class="grid gap-4 sm:gap-6 grid-cols-1 lg:grid-cols-2">
					<!-- Profile card -->
					<div class="rounded-2xl border border-pink-200/50 card-premium bg-white p-4 sm:p-6 shadow-sm dark:bg-[#2f3a52]">
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white">Your profile</h2>
						<dl class="mt-4 space-y-3">
							<div>
								<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Name</dt>
								<dd class="mt-0.5 font-medium text-slate-900 dark:text-white">{user.name}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Email</dt>
								<dd class="mt-0.5 font-medium text-slate-900 dark:text-white">{user.email}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Phone</dt>
								<dd class="mt-0.5 font-medium text-slate-900 dark:text-white">{user.phone}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Church</dt>
								<dd class="mt-0.5 font-medium text-slate-900 dark:text-white">{user.churchName}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Status</dt>
								<dd class="mt-0.5 flex items-center gap-2">
									<span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium {user.verified ? 'bg-green-100 text-green-800 dark:bg-green-900/50 dark:text-green-300' : 'bg-amber-100 text-amber-800 dark:bg-amber-900/50 dark:text-amber-300'}">
										{user.verified ? 'Verified' : 'Pending verification'}
									</span>
									<span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium {user.paymentComplete ? 'bg-green-100 text-green-800 dark:bg-green-900/50 dark:text-green-300' : 'bg-amber-100 text-amber-800 dark:bg-amber-900/50 dark:text-amber-300'}">
										{user.paymentComplete ? 'Paid' : 'Payment pending'}
									</span>
								</dd>
							</div>
						</dl>
					</div>

					<!-- Enrollment card -->
					<div class="rounded-2xl border border-pink-200/50 card-premium bg-white p-4 sm:p-6 shadow-sm dark:bg-[#2f3a52]">
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white">Enrollment</h2>
						{#if selectedCourse}
							<div class="mt-4 rounded-xl border border-pink-200/50 card-premium bg-pink-50/30 p-4 dark:bg-pink-900/20">
								<div class="flex items-center gap-4">
									<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-lg bg-pink-100 text-pink-600 text-xl dark:bg-pink-900/50 dark:text-pink-300">
										{getCourseDetailById(selectedCourse.id)?.icon ?? '📖'}
									</div>
									<div class="flex-1">
										<p class="font-medium text-slate-900 dark:text-white">{selectedCourse.name}</p>
										<p class="text-sm text-slate-500 dark:text-slate-300">{selectedCourse.duration} • {SITE.schedule}</p>
									</div>
								</div>
								<a
									href="/courses/{selectedCourse.id}"
									class="mt-3 inline-flex items-center gap-1 text-sm font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400 dark:hover:text-pink-300"
								>
									Review full course details & schedule
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
									</svg>
								</a>
							</div>
							{#if !user.paymentComplete}
								<a
									href={SITE.selarPaymentUrl}
									target="_blank"
									rel="noopener noreferrer"
									class="btn-scale mt-4 inline-flex w-full items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 py-3 text-sm font-semibold text-white"
								>
									Complete payment via Selar
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
									</svg>
								</a>
							{/if}
						{:else}
							<p class="mt-4 text-slate-600 dark:text-slate-200">No course selected yet.</p>
							<select
								class="mt-4 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#374266] dark:text-white"
								onchange={(e) => handleSelectCourse((e.target as HTMLSelectElement).value)}
							>
								<option value="">Select a course</option>
								{#each courses as course}
									<option value={course.id}>{course.name}</option>
								{/each}
							</select>
						{/if}
					</div>
				</div>

				<!-- Program info -->
				<div class="mt-6 sm:mt-8 rounded-2xl border border-pink-200/50 card-premium bg-white p-4 sm:p-6 shadow-sm dark:bg-[#2f3a52]">
					<h2 class="text-lg font-semibold text-slate-900 dark:text-white">Program details</h2>
					<div class="mt-4 grid gap-4 grid-cols-1 sm:grid-cols-3">
						<div>
							<p class="text-sm font-medium text-slate-500 dark:text-slate-300">Duration</p>
							<p class="font-medium text-slate-900 dark:text-white">{SITE.duration}</p>
						</div>
						<div>
							<p class="text-sm font-medium text-slate-500 dark:text-slate-300">Schedule</p>
							<p class="font-medium text-slate-900 dark:text-white">{SITE.schedule}</p>
						</div>
						<div>
							<p class="text-sm font-medium text-slate-500 dark:text-slate-300">Location</p>
							<p class="font-medium text-slate-900 dark:text-white">{SITE.location}</p>
						</div>
					</div>
				</div>

				<!-- Logout -->
				<div class="mt-8">
					<button
						type="button"
						onclick={() => auth.logout()}
						class="text-sm font-medium text-slate-500 transition hover:text-red-600 dark:text-slate-300 dark:hover:text-red-400"
					>
						Sign out
					</button>
				</div>
			</div>
		</main>

		<Footer />
	</div>
{/if}
