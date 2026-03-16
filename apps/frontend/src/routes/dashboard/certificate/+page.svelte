<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { userStore, authReady, fetchCourses } from '$lib/auth';
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
			// ignore
		}
	});

	const user = $derived($userStore);
	const canViewCertificate = $derived(
		user?.verified === true && user?.paymentComplete === true
	);
	const selectedCourse = $derived(
		user?.selectedCourseId
			? courses.find((c) => c.id === user.selectedCourseId)
			: null
	);
	const courseDetail = $derived(
		selectedCourse ? getCourseDetailById(selectedCourse.id) : null
	);
	const certificateDate = $derived(
		typeof document !== 'undefined'
			? new Date().toLocaleDateString(undefined, {
					month: 'long',
					day: 'numeric',
					year: 'numeric'
				})
			: ''
	);

	function printCertificate() {
		window.print();
	}
</script>

<svelte:head>
	<title>Certificate — {SITE.name} Pastoral School</title>
	<meta name="description" content="Your certificate of completion." />
</svelte:head>

{#if !mounted || !$authReady}
	<div class="flex min-h-screen items-center justify-center bg-slate-50 dark:bg-[#252d47]">
		<div class="h-8 w-8 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
	</div>
{:else if user && !canViewCertificate}
	<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
		<Nav />
		<main class="flex-1 pt-16 sm:pt-24 pb-12 px-4 sm:px-6">
			<div class="mx-auto max-w-2xl rounded-2xl border border-amber-200 bg-amber-50/80 p-8 dark:border-amber-800/40 dark:bg-amber-900/20">
				<h1 class="text-xl font-bold text-slate-900 dark:text-white">Certificate not yet available</h1>
				<p class="mt-2 text-slate-700 dark:text-slate-200">
					Your certificate will be available after you verify your account and complete payment.
				</p>
				<a
					href="/dashboard"
					class="mt-4 inline-block text-sm font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400"
				>
					← Back to dashboard
				</a>
			</div>
		</main>
		<Footer />
	</div>
{:else if user && canViewCertificate}
	<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
		<!-- Screen: show nav and print button -->
		<div class="print:hidden">
			<Nav />
		</div>
		<main class="flex-1 pt-16 sm:pt-24 pb-12 px-4 sm:px-6 print:pt-0 print:pb-0">
			<div class="mx-auto max-w-2xl print:max-w-none">
				<div class="print:hidden mb-6 flex items-center justify-between">
					<a
						href="/dashboard"
						class="text-sm font-medium text-slate-500 hover:text-pink-600 dark:text-slate-400 dark:hover:text-pink-400"
					>
						← Back to dashboard
					</a>
					<button
						type="button"
						onclick={printCertificate}
						class="rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-5 py-2.5 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700"
					>
						Print / Save as PDF
					</button>
				</div>

				<!-- Certificate (visible on screen and in print) -->
				<div
					class="certificate-border rounded-2xl border-2 border-pink-300 bg-white p-8 shadow-lg dark:border-pink-500/50 dark:bg-slate-900 print:shadow-none print:border-pink-400"
				>
					<div class="text-center">
						<p class="text-sm font-medium uppercase tracking-widest text-pink-600 dark:text-pink-400">
							{SITE.name}
						</p>
						<h2 class="mt-4 text-2xl font-bold text-slate-900 dark:text-white sm:text-3xl">
							Certificate of Completion
						</h2>
						<p class="mt-6 text-lg text-slate-700 dark:text-slate-200">
							This is to certify that
						</p>
						<p class="mt-2 text-2xl font-bold text-slate-900 dark:text-white sm:text-3xl">
							{user.name}
						</p>
						<p class="mt-4 text-slate-700 dark:text-slate-200">
							has successfully completed the
						</p>
						<p class="mt-2 text-xl font-semibold text-slate-900 dark:text-white">
							{courseDetail?.name ?? selectedCourse?.name ?? 'Pastoral School'} Program
						</p>
						<p class="mt-8 text-sm text-slate-500 dark:text-slate-400">
							{certificateDate}
						</p>
						<div class="mt-10 flex justify-center gap-8 print:mt-12">
							<div class="border-t border-slate-300 pt-2 dark:border-slate-600">
								<p class="text-xs font-medium uppercase text-slate-500 dark:text-slate-400">
									Director
								</p>
							</div>
							<div class="border-t border-slate-300 pt-2 dark:border-slate-600">
								<p class="text-xs font-medium uppercase text-slate-500 dark:text-slate-400">
									Date
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</main>
		<div class="print:hidden">
			<Footer />
		</div>
	</div>
{/if}

<style>
	@media print {
		.certificate-border {
			border-color: #ec4899 !important;
		}
	}
</style>
