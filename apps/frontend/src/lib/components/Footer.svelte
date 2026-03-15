<script lang="ts">
	import { SITE } from '$lib/config';

	import { subscribeNewsletter } from '$lib/api/newsletter';

	const quickLinks = [
		{ href: '/programs', label: 'Programs' },
		{ href: '/admissions', label: 'Admissions' },
		{ href: '/events', label: 'Events' },
		{ href: '/faq', label: 'FAQ' },
		{ href: '/testimonials', label: 'Testimonials' },
		{ href: '/blog', label: 'Blog' },
		{ href: '/newsletter', label: 'Newsletter' },
		{ href: '/contact', label: 'Contact' }
	];

	let newsletterEmail = $state('');
	let newsletterLoading = $state(false);
	let newsletterSuccess = $state(false);
	let newsletterError = $state('');

	async function handleNewsletterSubmit(e: Event) {
		e.preventDefault();
		if (newsletterLoading || !newsletterEmail.trim()) return;
		newsletterError = '';
		newsletterSuccess = false;
		newsletterLoading = true;
		try {
			await subscribeNewsletter(newsletterEmail);
			newsletterSuccess = true;
			newsletterEmail = '';
		} catch (err) {
			newsletterError = err instanceof Error ? err.message : 'Subscription failed';
		} finally {
			newsletterLoading = false;
		}
	}

	const legalLinks = [
		{ href: '/privacy', label: 'Privacy' },
		{ href: '/terms', label: 'Terms' },
		{ href: '/cookies', label: 'Cookies' }
	];
</script>

<footer class="border-t border-slate-200/80 section-separator bg-slate-50 dark:bg-[#1e2642]">
	<div class="mx-auto max-w-6xl px-4 py-12 sm:px-6 lg:px-8">
		<div class="grid gap-10 sm:grid-cols-2 lg:grid-cols-4">
			<!-- Brand -->
			<div class="sm:col-span-2 lg:col-span-1 min-w-0">
				<a href="/" class="inline-flex items-center gap-3">
					<div
						class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-pink-500 to-purple-600 text-white text-sm font-bold shadow-lg shadow-pink-500/20"
						aria-hidden="true"
					>
						P
					</div>
					<div>
						<span class="block font-semibold text-slate-900 dark:text-white">{SITE.schoolName}</span>
						<span class="block text-xs text-slate-500 dark:text-slate-200">{SITE.name}</span>
					</div>
				</a>
				<p class="mt-4 max-w-xs text-sm leading-relaxed text-slate-600 dark:text-slate-100">
					Equipping pastors, ministers and prophets for effective ministry. {SITE.duration}, {SITE.schedule}.
				</p>
			</div>

			<!-- Quick links -->
			<div class="min-w-0">
				<h3 class="text-sm font-semibold uppercase tracking-wider text-slate-900 dark:text-white">Quick links</h3>
				<ul class="mt-4 space-y-3">
					{#each quickLinks as link}
						<li>
							<a
								href={link.href}
								class="text-sm text-slate-600 transition hover:text-pink-600 dark:text-slate-100 dark:hover:text-pink-400"
							>
								{link.label}
							</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Newsletter -->
			<div class="min-w-0">
				<h3 class="text-sm font-semibold uppercase tracking-wider text-slate-900 dark:text-white">Newsletter</h3>
				<p class="mt-2 text-sm text-slate-600 dark:text-slate-100">Stay updated with news and events.</p>
				<form class="mt-3 flex flex-col gap-2 sm:flex-row sm:items-center" onsubmit={handleNewsletterSubmit}>
					<label for="footer-newsletter-email" class="sr-only">Newsletter email</label>
					<input
						id="footer-newsletter-email"
						type="email"
						placeholder="your@email.com"
						bind:value={newsletterEmail}
						disabled={newsletterLoading}
						class="min-w-0 flex-1 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 placeholder-slate-500 focus:border-pink-500 focus:outline-none focus:ring-1 focus:ring-pink-500 dark:border-slate-600 dark:bg-slate-800 dark:text-white dark:placeholder-slate-400"
						required
					/>
					<button
						type="submit"
						disabled={newsletterLoading}
						class="shrink-0 rounded-lg bg-pink-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-pink-700 disabled:opacity-50 dark:bg-pink-500 dark:hover:bg-pink-600"
					>
						{newsletterLoading ? 'Subscribing…' : 'Subscribe'}
					</button>
				</form>
				{#if newsletterSuccess}
					<p class="mt-2 text-sm text-green-600 dark:text-green-400">Thank you for subscribing.</p>
				{/if}
				{#if newsletterError}
					<p class="mt-2 text-sm text-red-600 dark:text-red-400">{newsletterError}</p>
				{/if}
			</div>

			<!-- Contact -->
			<div class="min-w-0">
				<h3 class="text-sm font-semibold uppercase tracking-wider text-slate-900 dark:text-white">Contact</h3>
				<address class="mt-4 not-italic">
					<p class="text-sm text-slate-600 break-words dark:text-slate-100">{SITE.address}</p>
					<p class="mt-1 text-sm text-slate-500 dark:text-slate-200">{SITE.location}</p>
					<div class="mt-4 space-y-2">
						<a
							href={`https://wa.me/${SITE.whatsappFull.replace('+', '')}`}
							target="_blank"
							rel="noopener noreferrer"
							class="flex items-center gap-2 text-sm text-slate-600 transition hover:text-[#25d366] dark:text-slate-100"
						>
							<svg class="h-4 w-4 shrink-0" fill="currentColor" viewBox="0 0 24 24">
								<path
									d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"
								/>
							</svg>
							{SITE.whatsapp}
						</a>
						<a
							href="mailto:{SITE.infoEmail}"
							class="block text-sm text-slate-600 transition hover:text-pink-600 break-words dark:text-slate-100 dark:hover:text-pink-400"
						>
							{SITE.infoEmail}
						</a>
					</div>
				</address>
			</div>
		</div>

		<!-- Bottom bar -->
		<div
			class="mt-12 flex flex-col items-center justify-between gap-4 border-t border-slate-200/80 section-separator pt-8 sm:flex-row"
		>
			<p class="text-sm text-slate-500 dark:text-slate-200">
				© {new Date().getFullYear()} {SITE.legalName}. All rights reserved.
			</p>
			<div class="flex flex-wrap items-center justify-center gap-6 text-sm">
				{#each legalLinks as link}
					<a href={link.href} class="text-slate-500 transition hover:text-slate-700 dark:text-slate-200 dark:hover:text-white">
						{link.label}
					</a>
				{/each}
			</div>
		</div>
		<p class="mt-4 text-center text-xs text-slate-400 pb-[max(2rem,calc(2rem+env(safe-area-inset-bottom)))] dark:text-slate-200">
			Powered by
			<a
				href="https://www.1nga.com"
				target="_blank"
				rel="noopener noreferrer"
				class="text-slate-500 transition hover:text-slate-700 dark:text-slate-200 dark:hover:text-white"
			>
				1nga Solutions
			</a>
		</p>
	</div>
</footer>
