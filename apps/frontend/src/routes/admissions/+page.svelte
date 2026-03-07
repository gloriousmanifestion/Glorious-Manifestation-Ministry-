<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { SITE } from '$lib/config';

	const requirements = [
		'A genuine call to pastoral, ministerial, or prophetic ministry',
		'Commitment to attend classes Monday to Friday',
		'Basic literacy and ability to study',
		'Recommendation from a church or ministry leader (optional but preferred)',
		'Heart for serving God and His people'
	];

	const steps = [
		{ num: 1, title: 'Register', desc: 'Create your account with our simple registration form.' },
		{ num: 2, title: 'Verify account', desc: 'Verify your email to activate your account.' },
		{ num: 3, title: 'Choose course & pay', desc: 'Select your program and complete payment.' },
		{ num: 4, title: 'Begin classes', desc: 'Start your journey in February and complete by July.' }
	];
</script>

<svelte:head>
	<title>Admissions — {SITE.name} Pastoral School</title>
	<meta
		name="description"
		content="Apply to the Pastoral School at Prophetic Glorious Manifestation. For pastors, ministers and prophets. February to July, Owerri, Imo State."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1">
		<section class="gradient-mesh px-4 sm:px-6 pt-24 sm:pt-28 pb-12 sm:pb-16 text-center">
			<p class="hero-enter-delay-1 mb-4 text-sm font-medium uppercase tracking-[0.2em] text-pink-600 dark:text-pink-400">Apply now</p>
			<h1 class="hero-enter-delay-2 mx-auto max-w-3xl text-3xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-4xl md:text-5xl">
				Admissions
			</h1>
			<p class="hero-enter-delay-3 mx-auto mt-6 max-w-2xl text-lg text-slate-600 dark:text-slate-100">
				{SITE.schoolName} is now open for pastors, ministers and prophets. Program runs {SITE.duration}, {SITE.schedule}.
			</p>
			<div class="hero-enter-delay-4 mt-8">
				<a
					href="/register"
					class="btn-scale inline-flex items-center justify-center rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-8 py-3.5 sm:px-10 text-sm font-semibold text-white shadow-lg transition hover:shadow-xl hover:from-pink-600 hover:to-purple-700 touch-manipulation active:scale-[0.98]"
				>
					Register now
				</a>
			</div>
		</section>

		<section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6 section-separator dark:bg-[#374266]/80">
			<div class="mx-auto max-w-3xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-2xl">Program details</h2>
				</AnimateOnView>
				<div class="mt-6 grid gap-4 grid-cols-1 sm:grid-cols-3">
					<AnimateOnView animation="fade-up" stagger={0}>
						<div class="hover-lift rounded-xl border border-pink-200/50 card-premium bg-white p-4 dark:bg-[#2f3a52]">
							<p class="text-sm font-medium text-pink-600 dark:text-pink-400">Duration</p>
							<p class="mt-1 font-semibold text-slate-900 dark:text-white">{SITE.duration}</p>
						</div>
					</AnimateOnView>
					<AnimateOnView animation="fade-up" stagger={1}>
						<div class="hover-lift rounded-xl border border-pink-200/50 card-premium bg-white p-4 dark:bg-[#2f3a52]">
							<p class="text-sm font-medium text-pink-600 dark:text-pink-400">Schedule</p>
							<p class="mt-1 font-semibold text-slate-900 dark:text-white">{SITE.schedule}</p>
						</div>
					</AnimateOnView>
					<AnimateOnView animation="fade-up" stagger={2}>
						<div class="hover-lift rounded-xl border border-pink-200/50 card-premium bg-white p-4 dark:bg-[#2f3a52]">
							<p class="text-sm font-medium text-pink-600 dark:text-pink-400">Location</p>
							<p class="mt-1 font-semibold text-slate-900 dark:text-white">{SITE.location}</p>
						</div>
					</AnimateOnView>
				</div>
			</div>
		</section>

		<section class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6 section-separator dark:from-[#374266]/60 dark:to-[#2f3a52]/60">
			<div class="mx-auto max-w-3xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">Requirements</h2>
				</AnimateOnView>
				<ul class="mt-6 space-y-3">
					{#each requirements as req, i}
						<AnimateOnView animation="slide-right" stagger={i} duration={400}>
							<li class="flex items-start gap-3">
								<span class="mt-1 flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-pink-500 text-xs font-bold text-white">
									✓
								</span>
								<span class="text-slate-600 dark:text-slate-200">{req}</span>
							</li>
						</AnimateOnView>
					{/each}
				</ul>
			</div>
		</section>

		<section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6 section-separator dark:bg-[#374266]/80">
			<div class="mx-auto max-w-3xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-2xl">How to apply</h2>
				</AnimateOnView>
				<div class="mt-6 sm:mt-8 space-y-6">
					{#each steps as step, i}
						<AnimateOnView animation="fade-up" stagger={i}>
							<div class="flex flex-col sm:flex-row gap-4">
							<span
								class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-pink-500 to-purple-600 text-lg font-bold text-white"
							>
								{step.num}
							</span>
								<div>
									<h3 class="font-semibold text-slate-900 dark:text-white">{step.title}</h3>
									<p class="mt-1 text-slate-600 dark:text-slate-200">{step.desc}</p>
								</div>
							</div>
						</AnimateOnView>
					{/each}
				</div>
			</div>
		</section>

		<section class="border-t border-pink-200/50 bg-gradient-to-r from-pink-500/10 to-purple-500/10 py-16 sm:py-20 px-4 sm:px-6 section-separator dark:from-pink-500/20 dark:to-purple-500/20">
			<AnimateOnView animation="scale">
				<div class="mx-auto max-w-2xl text-center">
					<h2 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">Ready to apply?</h2>
				<p class="mt-4 text-slate-600 dark:text-slate-200">Create your account and complete registration. You'll verify and pay from your dashboard.</p>
				<a
					href="/register"
					class="btn-scale mt-8 inline-flex items-center gap-2 rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-10 py-4 text-base font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700"
				>
					Create account
				</a>
				<p class="mt-4 text-sm text-slate-500 dark:text-slate-300">Questions? <a href="https://wa.me/2348026954759" target="_blank" rel="noopener noreferrer" class="font-medium text-pink-600 hover:underline dark:text-pink-400">Contact us on WhatsApp</a></p>
				</div>
			</AnimateOnView>
		</section>
	</main>

	<Footer />
</div>
