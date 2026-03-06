<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { SITE } from '$lib/config';
</script>

<svelte:head>
	<title>Cookie Policy — {SITE.schoolName}</title>
	<meta
		name="description"
		content="Cookie Policy for the Pastoral School under Prophetic Glorious Manifestation."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col bg-white">
	<Nav />
	<main class="flex-1 w-full max-w-3xl mx-auto px-4 sm:px-6 pt-24 sm:pt-28 pb-12 sm:pb-16">
		<a
			href="/"
			class="inline-flex items-center gap-2 text-sm font-medium text-slate-600 hover:text-slate-900 mb-8 transition"
		>
			<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
			</svg>
			Back to home
		</a>
		<div class="text-center">
			<h1 class="text-2xl font-bold text-slate-900 mb-2">Cookie Policy</h1>
			<p class="text-sm text-slate-500 mb-8">Last updated: February 2025</p>
		</div>
		<div class="prose prose-slate max-w-none text-slate-600 space-y-6 text-left">
					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">1. What are cookies?</h2>
						<p>
							Cookies are small text files that websites place on your device (computer, tablet, or
							phone) when you visit them. They are widely used to make sites work properly, remember
							your preferences, and understand how the site is used. This policy describes how the {SITE.schoolName} ("we", "us", "our") under {SITE.name} uses cookies and similar technologies on our website.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">2. Why we use cookies</h2>
						<p>We use cookies and similar technologies to:</p>
						<ul class="list-disc pl-6 space-y-1">
							<li>
								<strong>Keep you signed in</strong> — so you don’t have to enter your details every time
								you use the student portal or registration system.
							</li>
							<li>
								<strong>Secure your session</strong> — to help prevent fraud and protect your account.
							</li>
							<li>
								<strong>Remember your preferences</strong> — such as language or display settings where
								we offer them.
							</li>
							<li>
								<strong>Understand how our site is used</strong> — to improve the experience for visitors
								and students (e.g. which pages are visited, how we can improve our content).
							</li>
						</ul>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">3. Types of cookies we use</h2>
						<h3 class="text-base font-semibold text-slate-800 mt-6 mb-2">Strictly necessary cookies</h3>
						<p>
							These are required for our website to work. They enable sign-in, session management, and
							security. You cannot turn these off if you want to use the registration or student portal.
						</p>
						<h3 class="text-base font-semibold text-slate-800 mt-6 mb-2">Functional cookies</h3>
						<p>
							These remember choices you make (e.g. preferences) so we can provide a more
							personalised experience. They are not strictly necessary but improve how the site
							works for you.
						</p>
						<h3 class="text-base font-semibold text-slate-800 mt-6 mb-2">Analytics and performance cookies</h3>
						<p>
							These help us see how the site is used (e.g. number of visitors, pages viewed,
							errors). The data is typically aggregated and used to improve the site. Where we
							use third-party analytics, we ensure appropriate safeguards and, where required by
							law, ask for your consent.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">4. How long cookies last</h2>
						<p>
							<strong>Session cookies</strong> are deleted when you close your browser.
							<strong>Persistent cookies</strong> stay on your device for a set period (e.g. until they
							expire or you delete them). Sign-in and security cookies may last for the duration of your
							session or longer where needed to keep you signed in securely.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">5. Third-party cookies</h2>
						<p>
							When you use our site, some third-party services (e.g. payment processors, embedded content)
							may set their own cookies. Their use is governed by their respective privacy and cookie
							policies. We do not control third-party cookies. We only set cookies on our domain for the
							purposes described in this policy.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">6. Your choices</h2>
						<p>You can control cookies in several ways:</p>
						<ul class="list-disc pl-6 space-y-1">
							<li>
								<strong>Browser settings</strong> — Most browsers let you block or delete cookies. Note
								that blocking strictly necessary cookies will prevent you from signing in or using the
								student portal properly.
							</li>
							<li>
								<strong>Consent (where required)</strong> — In regions where we must obtain consent for
								non-essential cookies (e.g. certain analytics), we will ask for your choice and honour
								it.
							</li>
						</ul>
						<p>
							If you disable or delete cookies, some features may not work, and you may need to sign
							in again more often.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">7. Similar technologies</h2>
						<p>
							We may use technologies similar to cookies, such as local storage or session storage,
							for the same purposes (e.g. storing session data or preferences). This policy applies
							to those technologies in the same way as to cookies where they are used for similar
							functions.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">8. More information</h2>
						<p>
							For how we use your personal data, including data collected via cookies, see our
							<a href="/privacy" class="text-pink-600 underline hover:no-underline">Privacy Policy</a>.
							For the rules that govern your use of our service, see our
							<a href="/terms" class="text-pink-600 underline hover:no-underline">Terms of Service</a>.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">9. Changes</h2>
						<p>
							We may update this Cookie Policy from time to time to reflect changes in our practices
							or the law. We will post the revised policy here and update the “Last updated” date.
							We encourage you to review this page periodically.
						</p>
					</section>

					<section>
						<h2 class="text-lg font-semibold text-slate-900 mt-8 mb-3">10. Contact</h2>
						<p>
							For questions about our use of cookies, contact {SITE.name} at
							<a href="/contact" class="text-pink-600 underline hover:no-underline">our contact page</a>
							or email {SITE.contactEmail}.
						</p>
					</section>
		</div>
	</main>
	<Footer />
</div>
