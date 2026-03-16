<script lang="ts">
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { SITE } from '$lib/config';
	import { getProgramSchedule } from '$lib/courses';
	import type { CourseDetail } from '$lib/courses';
	import type { CourseModule } from '$lib/api/course-modules';
	import { auth, userStore } from '$lib/auth';

	interface Data {
		course: CourseDetail | null;
		modules: CourseModule[];
	}

	let { data }: { data: Data } = $props();

	let completedSet = $state<Set<string>>(new Set());
	let toggleLoading = $state<string | null>(null);

	const scheduleText = getProgramSchedule();

	function registerUrl(): string {
		if (!data.course) return '/register';
		return `/register?course=${encodeURIComponent(data.course.id)}`;
	}

	onMount(() => {
		if (!data.course || !get(userStore)) return;
		auth
			.getMyProgress(data.course.id)
			.then((p) => {
				completedSet = new Set(p.completed_module_ids);
			})
			.catch(() => {});
	});

	async function toggleComplete(mod: CourseModule) {
		if (!data.course || toggleLoading) return;
		const id = mod.id;
		toggleLoading = id;
		try {
			if (completedSet.has(id)) {
				await auth.unmarkModuleComplete(data.course.id, id);
				completedSet = new Set([...completedSet].filter((x) => x !== id));
			} else {
				await auth.markModuleComplete(data.course.id, id);
				completedSet = new Set([...completedSet, id]);
			}
		} catch {
			// leave UI unchanged
		} finally {
			toggleLoading = null;
		}
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
					<h1 class="text-2xl font-bold text-slate-900 dark:text-white">Course not found</h1>
					<p class="mt-4 text-slate-600 dark:text-slate-200">The course you're looking for doesn't exist or has been removed.</p>
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
			<section class="border-b border-pink-200/50 section-separator bg-gradient-to-b from-pink-50/50 to-white py-12 sm:py-16 px-4 sm:px-6 dark:from-[#252d47] dark:to-[#1e2642]">
				<div class="mx-auto max-w-4xl">
					<a
						href="/#programs"
						class="inline-flex items-center gap-2 text-sm font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400 dark:hover:text-pink-300 mb-6"
					>
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
						</svg>
						All courses
					</a>
					<AnimateOnView animation="fade-up">
						<div class="flex items-start gap-4">
							<span
								class="flex h-14 w-14 shrink-0 items-center justify-center rounded-2xl bg-gradient-to-br from-pink-100 to-purple-100 text-3xl dark:from-pink-900/50 dark:to-purple-900/50"
							>
								{data.course.icon}
							</span>
							<div>
								<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-3xl md:text-4xl">
									{data.course.name}
								</h1>
								<p class="mt-2 text-slate-600 dark:text-slate-200">{data.course.shortDescription}</p>
								<p class="mt-2 text-sm font-medium text-pink-600 dark:text-pink-400">{scheduleText} · {SITE.location}</p>
							</div>
						</div>
					</AnimateOnView>
				</div>
			</section>

			<!-- Full description -->
			<section class="py-12 sm:py-16 px-4 sm:px-6">
				<div class="mx-auto max-w-4xl">
					<AnimateOnView animation="fade-up">
						<h2 class="text-xl font-bold text-slate-900 dark:text-white">About this course</h2>
						<p class="mt-4 text-slate-600 dark:text-slate-200 leading-relaxed">{data.course.description}</p>
					</AnimateOnView>
					<AnimateOnView animation="fade-up" delay={100}>
						<h3 class="mt-10 text-lg font-bold text-slate-900 dark:text-white">Topics covered</h3>
						<ul class="mt-4 flex flex-wrap gap-2">
							{#each data.course.topics as topic}
								<li class="rounded-full bg-pink-50 px-4 py-2 text-sm font-medium text-pink-700 dark:bg-pink-900/40 dark:text-pink-300">
									{topic}
								</li>
							{/each}
						</ul>
					</AnimateOnView>
				</div>
			</section>

			<!-- Curriculum / Schedule -->
			<section class="border-t border-pink-200/50 section-separator bg-gradient-to-b from-white to-pink-50/30 py-12 sm:py-16 px-4 sm:px-6 dark:from-[#252d47] dark:to-[#1e2642]/80">
				<div class="mx-auto max-w-4xl">
					<AnimateOnView animation="fade-up">
						<h2 class="text-xl font-bold text-slate-900 dark:text-white">
							{data.modules.length > 0 ? 'Curriculum' : 'What to expect'}
						</h2>
						<p class="mt-2 text-slate-600 dark:text-slate-200">
							{data.modules.length > 0
								? 'Modules and topics for this course.'
								: 'A typical schedule for this course over the program period.'}
						</p>
					</AnimateOnView>
					{#if data.modules.length > 0}
						<ul class="mt-8 space-y-6">
							{#each data.modules as mod, i}
								<AnimateOnView animation="fade-up" delay={i * 50}>
									<li class="rounded-2xl border border-pink-200/50 card-premium bg-white p-4 sm:p-6 shadow-sm dark:bg-[#1e2642]">
										<div class="flex items-start justify-between gap-4">
											<div class="min-w-0 flex-1">
												<p class="text-sm font-semibold uppercase tracking-wider text-pink-600 dark:text-pink-400">
													Module {mod.sort_order + 1}
												</p>
												<h3 class="mt-2 font-semibold text-slate-900 dark:text-white">{mod.title}</h3>
												{#if mod.description}
													<p class="mt-1 text-slate-600 dark:text-slate-200 text-sm">{mod.description}</p>
												{/if}
												{#if mod.link}
													<a
														href={mod.link}
														target="_blank"
														rel="noopener noreferrer"
														class="mt-2 inline-flex items-center gap-1 text-sm font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400"
													>
														View resource
														<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
															<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
														</svg>
													</a>
												{/if}
											</div>
											{#if $userStore}
												<button
													type="button"
													disabled={toggleLoading === mod.id}
													onclick={() => toggleComplete(mod)}
													class="shrink-0 rounded-lg px-3 py-2 text-sm font-medium transition {completedSet.has(mod.id)
														? 'bg-green-100 text-green-700 dark:bg-green-900/50 dark:text-green-300'
														: 'bg-slate-100 text-slate-600 hover:bg-slate-200 dark:bg-slate-700 dark:text-slate-300 dark:hover:bg-slate-600'}"
												>
													{completedSet.has(mod.id) ? 'Completed' : 'Mark complete'}
												</button>
											{/if}
										</div>
									</li>
								</AnimateOnView>
							{/each}
						</ul>
					{:else}
						<ul class="mt-8 space-y-6">
							{#each data.course.schedule as item, i}
								<AnimateOnView animation="fade-up" stagger={i}>
									<li class="rounded-2xl border border-pink-200/50 card-premium bg-white p-4 sm:p-6 shadow-sm dark:bg-[#1e2642]">
										{#if item.week}
											<p class="text-sm font-semibold uppercase tracking-wider text-pink-600 dark:text-pink-400">{item.week}</p>
										{/if}
										<h3 class="mt-2 font-semibold text-slate-900 dark:text-white">{item.title}</h3>
										<p class="mt-1 text-slate-600 dark:text-slate-200 text-sm">{item.description}</p>
									</li>
								</AnimateOnView>
							{/each}
						</ul>
					{/if}
				</div>
			</section>

			<!-- Register CTA -->
			<section class="border-t border-pink-200/50 section-separator py-12 sm:py-16 px-4 sm:px-6">
				<div class="mx-auto max-w-2xl text-center">
					<AnimateOnView animation="scale">
						<h2 class="text-xl font-bold text-slate-900 dark:text-white">Register for {data.course.name}</h2>
						<p class="mt-4 text-slate-600 dark:text-slate-200">
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
								class="inline-flex items-center justify-center rounded-full border-2 border-pink-200 bg-white px-6 py-3 text-sm font-semibold text-pink-600 hover:bg-pink-50 transition dark:border-white/[0.12] dark:bg-[#1e2642] dark:text-pink-400 dark:hover:bg-white/10"
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
