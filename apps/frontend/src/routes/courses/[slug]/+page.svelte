<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { SITE } from '$lib/config';
	import { getProgramSchedule } from '$lib/courses';
	import type { CourseDetail } from '$lib/courses';

	interface Data {
		course: CourseDetail | null;
	}

	let { data }: { data: Data } = $props();

	const scheduleText = getProgramSchedule();

	function registerUrl(): string {
		if (!data.course) return '/register';
		return `/register?course=${encodeURIComponent(data.course.id)}`;
	}
</script>

<svelte:head>
	<title>{data.course ? `${data.course.name} — ${SITE.schoolName}` : 'Course not found'} — {SITE.name}</title>
	<meta
		name="description"
		content={data.course
			? `${data.course.name}: ${data.course.shortDescription} ${SITE.schoolName}, ${SITE.location}.`
			: 'Course not found.'}
	/>
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1">
		{#if !data.course}
			<section class="px-4 pt-28 pb-16 sm:px-6">
				<div class="mx-auto max-w-2xl text-center">
					<h1 class="text-2xl font-bold text-slate-900">Course not found</h1>
					<p class="mt-4 text-slate-600">The course you're looking for doesn't exist or has been removed.</p>
					<a
						href="/#programs"
						class="mt-6 inline-flex items-center gap-2 rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3 text-sm font-semibold text-white"
					>
						View all courses
					</a>
				</div>
			</section>
		{:else}
			<!-- Hero -->
			<section class="border-b border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-white py-12 sm:py-16 px-4 sm:px-6">
				<div class="mx-auto max-w-4xl">
					<a
						href="/#programs"
						class="inline-flex items-center gap-2 text-sm font-medium text-pink-600 hover:text-pink-700 mb-6"
					>
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
						</svg>
						All courses
					</a>
					<AnimateOnView animation="fade-up">
						<div class="flex items-start gap-4">
							<span
								class="flex h-14 w-14 shrink-0 items-center justify-center rounded-2xl bg-gradient-to-br from-pink-100 to-purple-100 text-3xl"
							>
								{data.course.icon}
							</span>
							<div>
								<h1 class="text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl md:text-4xl">
									{data.course.name}
								</h1>
								<p class="mt-2 text-slate-600">{data.course.shortDescription}</p>
								<p class="mt-2 text-sm font-medium text-pink-600">{scheduleText} · {SITE.location}</p>
							</div>
						</div>
					</AnimateOnView>
				</div>
			</section>

			<!-- Full description -->
			<section class="py-12 sm:py-16 px-4 sm:px-6">
				<div class="mx-auto max-w-4xl">
					<AnimateOnView animation="fade-up">
						<h2 class="text-xl font-bold text-slate-900">About this course</h2>
						<p class="mt-4 text-slate-600 leading-relaxed">{data.course.description}</p>
					</AnimateOnView>
					<AnimateOnView animation="fade-up" delay={100}>
						<h3 class="mt-10 text-lg font-bold text-slate-900">Topics covered</h3>
						<ul class="mt-4 flex flex-wrap gap-2">
							{#each data.course.topics as topic}
								<li class="rounded-full bg-pink-50 px-4 py-2 text-sm font-medium text-pink-700">
									{topic}
								</li>
							{/each}
						</ul>
					</AnimateOnView>
				</div>
			</section>

			<!-- Schedule: what to expect -->
			<section class="border-t border-pink-200/50 bg-gradient-to-b from-white to-pink-50/30 py-12 sm:py-16 px-4 sm:px-6">
				<div class="mx-auto max-w-4xl">
					<AnimateOnView animation="fade-up">
						<h2 class="text-xl font-bold text-slate-900">What to expect</h2>
						<p class="mt-2 text-slate-600">A typical schedule for this course over the program period.</p>
					</AnimateOnView>
					<ul class="mt-8 space-y-6">
						{#each data.course.schedule as item, i}
							<AnimateOnView animation="fade-up" stagger={i}>
								<li class="rounded-2xl border border-pink-200/50 bg-white p-4 sm:p-6 shadow-sm">
									{#if item.week}
										<p class="text-sm font-semibold uppercase tracking-wider text-pink-600">{item.week}</p>
									{/if}
									<h3 class="mt-2 font-semibold text-slate-900">{item.title}</h3>
									<p class="mt-1 text-slate-600 text-sm">{item.description}</p>
								</li>
							</AnimateOnView>
						{/each}
					</ul>
				</div>
			</section>

			<!-- Register CTA -->
			<section class="border-t border-pink-200/50 py-12 sm:py-16 px-4 sm:px-6">
				<div class="mx-auto max-w-2xl text-center">
					<AnimateOnView animation="scale">
						<h2 class="text-xl font-bold text-slate-900">Register for {data.course.name}</h2>
						<p class="mt-4 text-slate-600">
							Create your account to enroll in this course. You'll complete registration, then proceed to payment—no need to select a course again on your dashboard.
						</p>
						<div class="mt-8 flex flex-col sm:flex-row items-center justify-center gap-4">
							<a
								href={registerUrl()}
								class="btn-scale inline-flex items-center justify-center rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-8 py-3.5 sm:px-10 text-sm font-semibold text-white shadow-lg transition hover:shadow-xl touch-manipulation"
							>
								Register for this course
							</a>
							<a
								href="/#programs"
								class="inline-flex items-center justify-center rounded-full border-2 border-pink-200 bg-white px-6 py-3 text-sm font-semibold text-pink-600 hover:bg-pink-50 transition"
							>
								View all courses
							</a>
						</div>
					</AnimateOnView>
				</div>
			</section>
		{/if}
	</main>

	<Footer />
</div>
