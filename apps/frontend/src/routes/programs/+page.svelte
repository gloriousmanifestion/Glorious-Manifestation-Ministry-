<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import HeroCarousel from '$lib/components/HeroCarousel.svelte';
	import { SITE } from '$lib/config';
	import { COURSE_CATALOG } from '$lib/courses';

	const heroSlides = [
		{
			title: 'Equip for Ministry',
			subtitle: 'Comprehensive training for pastors, ministers, and prophets. Learn to lead, shepherd, and serve with biblical wisdom.',
			gradient: 'linear-gradient(135deg, #ec4899 0%, #a855f7 50%, #6366f1 100%)',
			icon: '📖'
		},
		{
			title: 'Biblical Foundation',
			subtitle: 'All teaching rooted in Scripture and sound doctrine. Grow in your understanding of God\'s Word.',
			gradient: 'linear-gradient(135deg, #8b5cf6 0%, #6366f1 50%, #3b82f6 100%)',
			icon: '📜'
		},
		{
			title: 'Practical Ministry',
			subtitle: 'Combine theological education with hands-on ministry skills. Ready for real-world ministry challenges.',
			gradient: 'linear-gradient(135deg, #db2777 0%, #ec4899 50%, #8b5cf6 100%)',
			icon: '⛪'
		},
		{
			title: `${SITE.duration}, ${SITE.schedule}`,
			subtitle: `A structured program in ${SITE.location}. Join a community of ministers and prophets.`,
			gradient: 'linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #a855f7 100%)',
			icon: '📅'
		},
		{
			title: 'Six Core Courses',
			subtitle: 'Pastoral Leadership, Prophetic Ministry, Biblical Studies, Worship, Evangelism, and Ministerial Training.',
			gradient: 'linear-gradient(135deg, #a855f7 0%, #ec4899 50%, #f43f5e 100%)',
			icon: '✨'
		}
	];

	const values = [
		{ title: 'Biblical Foundation', description: 'All teaching rooted in Scripture and sound doctrine.' },
		{ title: 'Spiritual Growth', description: 'Nurturing personal relationship with God and spiritual maturity.' },
		{ title: 'Practical Ministry', description: 'Hands-on training for real-world ministry challenges.' },
		{ title: 'Community', description: 'Building lasting fellowship among ministers and prophets.' }
	];

	const highlights = [
		{ label: 'Duration', value: SITE.duration },
		{ label: 'Schedule', value: SITE.schedule },
		{ label: 'Location', value: SITE.location }
	];

	const faqs = [
		{
			q: 'Who is this program for?',
			a: 'Our Pastoral School is designed for pastors, ministers, and prophets who desire to grow in their calling. Whether you are already in ministry or sensing a call, this program will equip you with biblical foundations and practical skills.'
		},
		{
			q: 'What are the admission requirements?',
			a: 'We look for a genuine call to ministry, commitment to attend classes, and basic literacy. A recommendation from a church or ministry leader is optional but preferred. See our <a href="/admissions" class="text-pink-600 font-medium hover:underline">Admissions page</a> for the full process.'
		},
		{
			q: 'How do I register?',
			a: 'Create an account on our site, verify your email, then select your course and complete payment. You can also contact us via WhatsApp for assistance.'
		},
		{
			q: 'Where are classes held?',
			a: 'Classes are held at our facility in ' + SITE.location + '. Our address is ' + SITE.address + '.'
		}
	];
</script>

<svelte:head>
	<title>Programs &amp; Courses — {SITE.name} Pastoral School</title>
	<meta
		name="description"
		content="Learn about our Pastoral School: all courses, mission, values, and helpful information. Pastoral Leadership, Prophetic Ministry, Ministerial Training, Biblical Studies, Worship, and Evangelism. {SITE.location}."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1">
		<!-- Hero carousel -->
		<section class="px-4 pt-24 sm:pt-28 pb-8 sm:px-6">
			<div class="mx-auto max-w-5xl">
				<div class="mb-6 text-center">
					<p class="text-sm font-medium uppercase tracking-[0.2em] text-pink-600">{SITE.name}</p>
					<h1 class="mt-2 text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl">
						{SITE.schoolName}
					</h1>
				</div>
				<HeroCarousel slides={heroSlides} interval={5000} />
			</div>
		</section>

		<!-- About & mission -->
		<section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6">
			<div class="mx-auto max-w-4xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-2xl font-bold tracking-tight text-slate-900">Our mission</h2>
					<p class="mt-6 text-slate-600 leading-relaxed">
						Under the umbrella of Prophetic Glorious Manifestation, our Pastoral School exists to equip and empower pastors, ministers, and prophets for effective ministry. We are committed to raising a generation of spiritual leaders who are grounded in the Word, led by the Spirit, and equipped to serve their communities.
					</p>
					<p class="mt-4 text-slate-600 leading-relaxed">
						Located in {SITE.location}, we offer a structured program running {SITE.duration}, {SITE.schedule}. Our curriculum combines theological education with practical ministry skills for those called to pastoral ministry, prophetic ministry, or general ministerial service.
					</p>
				</AnimateOnView>
			</div>
		</section>

		<!-- Program highlights -->
		<section class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6">
			<div class="mx-auto max-w-4xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl">Program details</h2>
				</AnimateOnView>
				<div class="mt-8 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-3">
					{#each highlights as item, i}
						<AnimateOnView animation="fade-up" stagger={i} duration={500}>
							<div class="hover-lift rounded-2xl border border-pink-200/60 bg-white p-6 text-center shadow-sm hover:border-pink-300 hover:shadow-lg">
								<p class="text-sm font-medium uppercase tracking-wider text-pink-600">{item.label}</p>
								<p class="mt-2 text-lg font-semibold text-slate-900">{item.value}</p>
							</div>
						</AnimateOnView>
					{/each}
				</div>
			</div>
		</section>

		<!-- Our values -->
		<section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6">
			<div class="mx-auto max-w-6xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl md:text-3xl">Our values</h2>
				</AnimateOnView>
				<div class="mt-8 sm:mt-12 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-4">
					{#each values as value, i}
						<AnimateOnView animation="fade-up" stagger={i}>
							<div class="hover-lift rounded-2xl border border-pink-200/50 bg-white p-6 shadow-sm">
								<h3 class="text-lg font-semibold text-slate-900">{value.title}</h3>
								<p class="mt-2 text-sm text-slate-600">{value.description}</p>
							</div>
						</AnimateOnView>
					{/each}
				</div>
			</div>
		</section>

		<!-- Courses -->
		<section id="courses" class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6">
			<div class="mx-auto max-w-4xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl mb-8 sm:mb-12">All courses</h2>
				</AnimateOnView>
				<div class="space-y-8 sm:space-y-12">
				{#each COURSE_CATALOG as course, i}
					<AnimateOnView animation="slide-left" stagger={i} duration={500}>
						<article
							class="hover-lift rounded-2xl border border-pink-200/50 bg-white p-4 sm:p-8 shadow-sm hover:border-pink-300 hover:shadow-xl"
						>
							<a href="/courses/{course.id}" class="flex flex-col sm:flex-row items-start gap-4 group">
								<span
									class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-pink-100 to-purple-100 text-2xl"
								>
									{course.icon}
								</span>
								<div class="flex-1">
									<h2 class="text-xl font-bold text-slate-900 group-hover:text-pink-600 transition-colors">{course.name}</h2>
									<p class="mt-3 text-slate-600">{course.description}</p>
									<ul class="mt-4 flex flex-wrap gap-2">
										{#each course.topics as topic}
											<li class="rounded-full bg-pink-50 px-3 py-1 text-sm text-pink-700">
												{topic}
											</li>
										{/each}
									</ul>
									<span class="mt-4 inline-flex items-center gap-1 text-sm font-medium text-pink-600">
										View full details and schedule
										<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
										</svg>
									</span>
								</div>
							</a>
						</article>
					</AnimateOnView>
				{/each}
				</div>
			</div>
		</section>

		<!-- FAQ / Helpful info -->
		<section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6">
			<div class="mx-auto max-w-3xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-2xl font-bold tracking-tight text-slate-900">Frequently asked questions</h2>
				</AnimateOnView>
				<div class="mt-8 space-y-6">
					{#each faqs as faq, i}
						<AnimateOnView animation="fade-up" stagger={i}>
							<div class="rounded-2xl border border-pink-200/50 bg-white p-6 shadow-sm">
								<h3 class="font-semibold text-slate-900">{faq.q}</h3>
								<p class="mt-2 text-sm text-slate-600 leading-relaxed">{@html faq.a}</p>
							</div>
						</AnimateOnView>
					{/each}
				</div>
			</div>
		</section>

		<!-- CTA -->
		<section class="border-t border-pink-200/50 bg-gradient-to-r from-pink-500/10 to-purple-500/10 py-12 sm:py-16 px-4 sm:px-6">
			<AnimateOnView animation="scale">
				<div class="mx-auto max-w-2xl text-center">
					<h2 class="text-2xl font-bold tracking-tight text-slate-900">Ready to answer the call?</h2>
					<p class="mt-4 text-slate-600">Create your account to register, or contact us for more details.</p>
					<div class="mt-6 flex justify-center">
						<a
							href="/register"
							class="btn-scale inline-flex items-center justify-center rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-8 py-3.5 text-sm font-semibold text-white shadow-lg transition hover:shadow-xl"
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
