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
	<div class="flex min-h-screen items-center justify-center bg-slate-50">
		<div class="h-8 w-8 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
	</div>
{:else if user}
	<div class="min-h-screen flex flex-col bg-slate-50">
		<Nav />

		<main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16">
			<div class="mx-auto max-w-6xl px-4 sm:px-6">
				<!-- Welcome header -->
				<div class="mb-6 sm:mb-8">
					<h1 class="text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl">
						Welcome back, {user.name.split(' ')[0]}
					</h1>
					<p class="mt-1 text-slate-600">
						Manage your Pastoral School enrollment
					</p>
				</div>

				{#if actionError}
					<div class="mb-4 rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700">
						{actionError}
					</div>
				{/if}

				<!-- Pending actions banners -->
				<div class="mb-8 space-y-4">
					{#if showVerifyBanner}
						<div
							class="flex flex-col gap-4 rounded-2xl border-2 border-amber-200 bg-amber-50/80 p-4 sm:p-6 shadow-sm sm:flex-row sm:items-center sm:justify-between"
						>
							<div class="flex items-start gap-4">
								<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-amber-100">
									<svg class="h-6 w-6 text-amber-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
									</svg>
								</div>
								<div>
									<h3 class="font-semibold text-slate-900">Verify your account</h3>
									<p class="mt-1 text-sm text-slate-600">
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
							class="flex flex-col gap-4 rounded-2xl border-2 border-pink-200 bg-gradient-to-r from-pink-50/80 to-purple-50/50 p-4 sm:p-6 shadow-sm sm:flex-row sm:items-center sm:justify-between"
						>
							<div class="flex items-start gap-4">
								<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-pink-100">
									<svg class="h-6 w-6 text-pink-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
									</svg>
								</div>
								<div>
									<h3 class="font-semibold text-slate-900">Complete your enrollment</h3>
									<p class="mt-1 text-sm text-slate-600">
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
										class="rounded-xl border border-pink-200 bg-white px-4 py-3 text-base sm:text-sm font-medium text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 touch-manipulation"
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
					<div class="rounded-2xl border border-pink-200/50 bg-white p-4 sm:p-6 shadow-sm">
						<h2 class="text-lg font-semibold text-slate-900">Your profile</h2>
						<dl class="mt-4 space-y-3">
							<div>
								<dt class="text-sm font-medium text-slate-500">Name</dt>
								<dd class="mt-0.5 font-medium text-slate-900">{user.name}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500">Email</dt>
								<dd class="mt-0.5 font-medium text-slate-900">{user.email}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500">Phone</dt>
								<dd class="mt-0.5 font-medium text-slate-900">{user.phone}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500">Church</dt>
								<dd class="mt-0.5 font-medium text-slate-900">{user.churchName}</dd>
							</div>
							<div>
								<dt class="text-sm font-medium text-slate-500">Status</dt>
								<dd class="mt-0.5 flex items-center gap-2">
									<span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium {user.verified ? 'bg-green-100 text-green-800' : 'bg-amber-100 text-amber-800'}">
										{user.verified ? 'Verified' : 'Pending verification'}
									</span>
									<span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium {user.paymentComplete ? 'bg-green-100 text-green-800' : 'bg-amber-100 text-amber-800'}">
										{user.paymentComplete ? 'Paid' : 'Payment pending'}
									</span>
								</dd>
							</div>
						</dl>
					</div>

					<!-- Enrollment card -->
					<div class="rounded-2xl border border-pink-200/50 bg-white p-4 sm:p-6 shadow-sm">
						<h2 class="text-lg font-semibold text-slate-900">Enrollment</h2>
						{#if selectedCourse}
							<div class="mt-4 rounded-xl border border-pink-200/50 bg-pink-50/30 p-4">
								<div class="flex items-center gap-4">
									<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-lg bg-pink-100 text-pink-600 text-xl">
										{getCourseDetailById(selectedCourse.id)?.icon ?? '📖'}
									</div>
									<div class="flex-1">
										<p class="font-medium text-slate-900">{selectedCourse.name}</p>
										<p class="text-sm text-slate-500">{selectedCourse.duration} • {SITE.schedule}</p>
									</div>
								</div>
								<a
									href="/courses/{selectedCourse.id}"
									class="mt-3 inline-flex items-center gap-1 text-sm font-medium text-pink-600 hover:text-pink-700"
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
							<p class="mt-4 text-slate-600">No course selected yet.</p>
							<select
								class="mt-4 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500"
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
				<div class="mt-6 sm:mt-8 rounded-2xl border border-pink-200/50 bg-white p-4 sm:p-6 shadow-sm">
					<h2 class="text-lg font-semibold text-slate-900">Program details</h2>
					<div class="mt-4 grid gap-4 grid-cols-1 sm:grid-cols-3">
						<div>
							<p class="text-sm font-medium text-slate-500">Duration</p>
							<p class="font-medium text-slate-900">{SITE.duration}</p>
						</div>
						<div>
							<p class="text-sm font-medium text-slate-500">Schedule</p>
							<p class="font-medium text-slate-900">{SITE.schedule}</p>
						</div>
						<div>
							<p class="text-sm font-medium text-slate-500">Location</p>
							<p class="font-medium text-slate-900">{SITE.location}</p>
						</div>
					</div>
				</div>

				<!-- Logout -->
				<div class="mt-8">
					<button
						type="button"
						onclick={() => auth.logout()}
						class="text-sm font-medium text-slate-500 transition hover:text-red-600"
					>
						Sign out
					</button>
				</div>
			</div>
		</main>

		<Footer />
	</div>
{/if}
