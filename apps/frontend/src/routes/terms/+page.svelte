<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { SITE } from '$lib/config';
	import { pageSeo } from '$lib/stores/pageSeo';

	pageSeo.set({
		title: `Terms of Service — ${SITE.schoolName}`,
		description: 'Terms of Service for the Pastoral School under Prophetic Glorious Manifestation.'
	});
</script>

<div class="min-h-screen flex flex-col bg-white dark:bg-[#252d47]">
	<Nav />
	<main class="flex-1 w-full max-w-3xl mx-auto px-4 sm:px-6 pt-24 sm:pt-28 pb-12 sm:pb-16">
		<a
			href="/"
			class="inline-flex items-center gap-2 text-sm font-medium text-slate-600 hover:text-slate-900 mb-8 transition dark:text-slate-200 dark:hover:text-white"
		>
			<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
			</svg>
			Back to home
		</a>
		<div class="text-center">
			<h1 class="text-2xl font-bold text-slate-900 dark:text-white mb-2">Terms of Service</h1>
			<p class="text-sm text-slate-500 dark:text-slate-200 mb-8">Last updated: February 2025</p>
		</div>
		<div class="prose prose-slate max-w-none text-slate-600 dark:text-slate-100 space-y-6 text-left dark:prose-invert">
					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">1. Agreement to terms</h2>
						<p>
							Welcome to the {SITE.schoolName} under {SITE.name}. By accessing or using our website,
							registration system, student portal, or any services we provide, you agree to be bound by
							these Terms of Service. If you do not agree, do not use our services.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">2. Description of services</h2>
						<p>
							We operate the {SITE.schoolName}, a ministry training program for pastors, ministers, and
							prophets. Our website provides information about our programs, admissions, events, and
							allows you to register, create an account, and access student resources. Your use of
							specific services (e.g. payment, events) may be subject to additional terms.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">3. Eligibility and account</h2>
						<p>
							You must be at least 18 years old (or the age of majority in your jurisdiction) and
							able to form a binding contract to create an account. You are responsible for keeping your
							sign-in credentials secure and up to date. You must notify us promptly of any unauthorized
							use of your account.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">4. Acceptable use</h2>
						<p>
							You agree not to use our website or services to: (a) violate any law or regulation;
							(b) infringe others' rights; (c) distribute malware or attempt to gain unauthorized
							access to our or others' systems; (d) abuse, harass, or harm others; or (e) circumvent
							security or access controls. We may suspend or terminate access for breach of these terms.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">5. Privacy and data</h2>
						<p>
							Your use of our services is also governed by our
							<a href="/privacy" class="text-pink-600 underline hover:no-underline dark:text-pink-400 dark:hover:text-pink-300">Privacy Policy</a>
							and, where applicable, our
							<a href="/cookies" class="text-pink-600 underline hover:no-underline dark:text-pink-400 dark:hover:text-pink-300">Cookie Policy</a>.
							By using our services, you consent to the collection, use, and sharing of information
							as described there.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">6. Intellectual property</h2>
						<p>
							{SITE.name}, the {SITE.schoolName}, and related logos, names, and materials are owned by
							us or our licensors. You do not receive any right to use them except as necessary to use
							our services in accordance with these terms.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">7. Disclaimers</h2>
						<p>
							Our services are provided "as is". We do not warrant that our website or services will be
							uninterrupted, error-free, or secure. To the fullest extent permitted by law, we
							disclaim all warranties, express or implied, including merchantability and fitness for
							a particular purpose.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">8. Limitation of liability</h2>
						<p>
							To the maximum extent permitted by law, {SITE.name} and the {SITE.schoolName} (and their
							affiliates, officers, and employees) shall not be liable for any indirect, incidental,
							special, consequential, or punitive damages, or any loss of data, revenue, or profits,
							arising from your use of our services. Our total liability shall not exceed the amount
							you paid to us (if any) in the twelve months before the claim.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">9. Changes</h2>
						<p>
							We may update these Terms from time to time. We will post the revised Terms on this
							page and update the "Last updated" date. Continued use of our services after changes
							constitutes acceptance. If you do not agree, you must stop using our services.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 dark:text-white mt-8 mb-3">10. Contact</h2>
						<p>
							For questions about these Terms, contact {SITE.name} at
							<a href="/contact" class="text-pink-600 underline hover:no-underline dark:text-pink-400 dark:hover:text-pink-300">our contact page</a>
							or email {SITE.contactEmail}. 
						</p>
					</section>
		</div>
	</main>
	<Footer />
</div>
