<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import LeaderSection from '$lib/components/LeaderSection.svelte';
	import { SITE } from '$lib/config';
	import { COURSE_CATALOG } from '$lib/courses';

	const values = [
		{ title: 'Biblical Foundation', description: 'All teaching rooted in Scripture and sound doctrine.' },
		{ title: 'Spiritual Growth', description: 'Nurturing personal relationship with God and spiritual maturity.' },
		{ title: 'Practical Ministry', description: 'Hands-on training for real-world ministry challenges.' },
		{ title: 'Community', description: 'Building lasting fellowship among ministers and prophets.' }
	];
</script>

<svelte:head>
	<title>Pastoral School — {SITE.name}</title>
	<meta
		name="description"
		content="Pastoral School under Prophetic Glorious Manifestation. Training pastors, ministers and prophets in Owerri, Imo State. Our mission, values, programs and admissions. February to July program."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col dark:bg-[#252d47]">
	<Nav />

	<main class="flex-1">
		<!-- Hero -->
		<section class="gradient-mesh relative min-h-[85vh] sm:min-h-[90vh] flex flex-col items-center justify-center px-4 pt-24 pb-16 sm:px-6 sm:pt-28 sm:pb-24 text-center overflow-hidden">
			<div
				class="animate-gradient-bg absolute inset-0"
				aria-hidden="true"
				style="background: radial-gradient(circle at 30% 20%, rgba(236,72,153,0.3) 0%, transparent 50%), radial-gradient(circle at 70% 80%, rgba(168,85,247,0.3) 0%, transparent 50%);"
			></div>
			<p class="hero-enter-delay-1 relative mb-4 text-sm font-medium uppercase tracking-[0.2em] text-pink-600/90 dark:text-pink-400">
				{SITE.name}
			</p>
			<h1 class="hero-enter-delay-2 relative max-w-4xl text-3xl font-bold tracking-tight text-slate-900 sm:text-4xl md:text-5xl lg:text-6xl xl:text-7xl dark:text-white">
				<span class="bg-gradient-to-r from-pink-600 via-purple-600 to-pink-600 bg-clip-text text-transparent">
					{SITE.schoolName}
				</span>
			</h1>
			<div class="hero-enter-delay-3 relative mt-6 flex items-center gap-3">
				<span
					class="animate-glow-pulse inline-block rounded-full bg-pink-500 px-5 py-2 text-sm font-bold uppercase tracking-wider text-white shadow-lg glow-pink"
				>
					Now Open
				</span>
			</div>
			<div class="hero-enter-delay-3 relative mt-6 h-px w-32 rounded-full shine-line" role="presentation"></div>
			<p class="hero-enter-delay-4 relative mt-6 sm:mt-8 max-w-2xl text-base text-slate-600 sm:text-lg md:text-xl px-1 dark:text-slate-100">
				For pastors, ministers and prophets. Equipping the next generation of spiritual leaders with sound doctrine and practical ministry skills.
			</p>
			<div class="hero-enter-delay-5 relative mt-8 sm:mt-12 flex flex-col sm:flex-row flex-wrap items-stretch sm:items-center justify-center gap-3 sm:gap-4">
				<a
					href="/register"
					class="btn-scale rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3.5 sm:px-8 text-sm font-semibold text-white shadow-lg glow-pink text-center touch-manipulation active:scale-[0.98] dark:from-pink-500 dark:to-purple-600"
				>
					Register now
				</a>
				<a
					href="#programs"
					class="btn-scale inline-flex items-center justify-center gap-2 rounded-full border-2 border-pink-300 bg-white px-6 py-3.5 sm:px-8 text-sm font-semibold text-pink-600 transition hover:bg-pink-50 hover:border-pink-400 touch-manipulation active:scale-[0.98] dark:border-pink-500/60 dark:bg-white/10 dark:text-pink-300 dark:hover:bg-white/20"
				>
					Learn more
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
					</svg>
				</a>
			</div>
		</section>

		<!-- About & mission -->
		<section id="about" class="border-t border-pink-200/50 section-separator bg-white/60 py-12 sm:py-16 px-4 sm:px-6 dark:bg-[#252d47]/80">
			<div class="mx-auto max-w-4xl">
				<AnimateOnView animation="fade-up">
					<div class="mb-12 overflow-hidden rounded-2xl border border-pink-200/50 card-premium shadow-lg transition-shadow duration-500 hover:shadow-xl ">
						<img
							src="/pastoral-school-flyer.png"
							alt="Pastoral School promotional flyer - Now Open for pastors, ministers and prophets"
							class="w-full object-cover transition-transform duration-700 hover:scale-[1.02]"
						/>
					</div>
				</AnimateOnView>
				<AnimateOnView animation="fade-up" delay={100}>
					<h2 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">Our mission</h2>
					<p class="mt-6 text-slate-600 leading-relaxed dark:text-slate-100">
						Under the umbrella of Prophetic Glorious Manifestation, our Pastoral School exists to equip and empower pastors, ministers, and prophets for effective ministry. We are committed to raising a generation of spiritual leaders who are grounded in the Word, led by the Spirit, and equipped to serve their communities.
					</p>
					<p class="mt-4 text-slate-600 leading-relaxed dark:text-slate-100">
						Located in {SITE.location}, we offer a structured program running {SITE.duration}, {SITE.schedule}. Our curriculum combines theological education with practical ministry skills for those called to pastoral ministry, prophetic ministry, or general ministerial service.
					</p>
				</AnimateOnView>
			</div>
		</section>

		<!-- Leader -->
		<LeaderSection />

		<!-- Our values -->
		<section class="border-t border-pink-200/50 section-separator bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6 dark:from-[#252d47] dark:to-[#1e2642]">
			<div class="mx-auto max-w-6xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl md:text-3xl dark:text-white">Our values</h2>
				</AnimateOnView>
				<div class="mt-8 sm:mt-12 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-4">
					{#each values as value, i}
						<AnimateOnView animation="fade-up" stagger={i}>
							<div class="hover-lift rounded-2xl border border-pink-200/50 card-premium bg-white p-6 shadow-sm  dark:bg-[#2d3659]/80 dark:shadow-none">
								<h3 class="text-lg font-semibold text-slate-900 dark:text-white">{value.title}</h3>
								<p class="mt-2 text-sm text-slate-600 dark:text-slate-100">{value.description}</p>
							</div>
						</AnimateOnView>
					{/each}
				</div>
			</div>
		</section>

		<!-- Programs -->
		<section id="programs" class="border-t border-pink-200/50 section-separator bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-16 sm:py-24 px-4 sm:px-6 dark:from-[#252d47] dark:to-[#1e2642]">
			<div class="mx-auto max-w-6xl">
				<AnimateOnView animation="fade-up">
					<p class="text-center text-sm font-medium uppercase tracking-[0.2em] text-pink-600 dark:text-pink-400">
						Our curriculum
					</p>
					<h2 class="mt-4 text-center text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl md:text-4xl dark:text-white">
						Programs &amp; courses
					</h2>
					<p class="mx-auto mt-4 max-w-2xl text-center text-slate-600 dark:text-slate-100">
						Comprehensive training designed to equip pastors, ministers, and prophets for effective ministry.
					</p>
				</AnimateOnView>
				<ul class="mt-10 sm:mt-16 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
					{#each COURSE_CATALOG as course, i}
						<AnimateOnView animation="fade-up" stagger={i} duration={500}>
							<li>
								<a
									href="/courses/{course.id}"
									class="hover-lift group block rounded-2xl border border-pink-200/50 card-premium bg-white p-6 shadow-sm transition hover:border-pink-300 hover:shadow-xl hover:shadow-pink-100/50  dark:bg-[#2d3659]/80 dark:hover:border-pink-500/30 dark:hover:shadow-pink-500/10"
								>
									<div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-pink-100 to-purple-100 text-2xl transition-transform duration-300 group-hover:scale-110 dark:from-pink-500/20 dark:to-purple-500/20">
										{course.icon}
									</div>
									<h3 class="text-lg font-semibold text-slate-900 transition-colors group-hover:text-pink-600 dark:text-white dark:group-hover:text-pink-400">
										{course.name}
									</h3>
									<p class="mt-3 text-sm leading-relaxed text-slate-600 dark:text-slate-100">
										{course.shortDescription}
									</p>
									<span class="mt-3 inline-flex items-center gap-1 text-sm font-medium text-pink-600 group-hover:gap-2 transition-all dark:text-pink-400">
										View details and schedule
										<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
										</svg>
									</span>
								</a>
							</li>
						</AnimateOnView>
					{/each}
				</ul>
			</div>
		</section>

		<!-- CTA strip -->
		<section class="border-t border-pink-200/50 section-separator bg-gradient-to-r from-pink-500/10 via-purple-500/10 to-pink-500/10 py-16 sm:py-20 px-4 sm:px-6 dark:from-pink-500/20 dark:via-purple-500/20 dark:to-pink-500/20">
			<AnimateOnView animation="scale" duration={600}>
				<div class="mx-auto max-w-3xl text-center">
					<h2 class="text-xl font-bold tracking-tight text-slate-900 sm:text-2xl md:text-3xl dark:text-white">
						Ready to answer the call?
					</h2>
					<p class="mt-4 text-slate-600 dark:text-slate-100">
						Join our Pastoral School and be equipped for effective ministry.
					</p>
					<div class="mt-6 sm:mt-8 flex justify-center">
						<a
							href="/register"
							class="btn-scale inline-flex items-center justify-center rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-8 py-3.5 sm:px-10 text-sm font-semibold text-white shadow-lg transition hover:shadow-xl touch-manipulation active:scale-[0.98]"
						>
							Register now
						</a>
					</div>
				</div>
			</AnimateOnView>
		</section>
	</main>

	<Footer />
</div>
