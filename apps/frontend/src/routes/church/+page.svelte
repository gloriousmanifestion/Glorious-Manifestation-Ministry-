<script lang="ts">
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { SITE } from '$lib/config';
	import { getChurchServices, getChurchContent, churchImageUrl } from '$lib/api/church';
	import type { ChurchService } from '$lib/api/church';

	const church = SITE.church;

	let services = $state<ChurchService[]>([]);
	let aboutContent = $state<string | null>(null);
	let loaded = $state(false);

	onMount(async () => {
		try {
			const [list, about] = await Promise.all([
				getChurchServices(),
				getChurchContent('church_about')
			]);
			services = list;
			aboutContent = about;
		} catch {
			// Keep config fallback
		} finally {
			loaded = true;
		}
	});

	const displayServices = $derived(
		loaded && services.length > 0
			? services
			: church.services.map((s) => ({
					id: '',
					name: s.name,
					description: s.description,
					day_of_week: s.day,
					time: s.time,
					sort_order: 0,
					image_url: null,
					features: null,
					is_active: true,
					created_at: '',
					updated_at: ''
				}))
	);
	const displayAbout = $derived(aboutContent ?? church.description);
</script>

<svelte:head>
	<title>Church — {SITE.name} Pastoral School</title>
	<meta
		name="description"
		content="Learn about {church.shortName}: church details, service times, and how we worship and serve together in {SITE.location}."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1">
		<section class="gradient-mesh px-4 sm:px-6 pt-24 sm:pt-28 pb-12 sm:pb-16 text-center">
			<p class="hero-enter-delay-1 mb-4 text-sm font-medium uppercase tracking-[0.2em] text-pink-600 dark:text-pink-400">
				Our Ministry
			</p>
			<h1 class="hero-enter-delay-2 mx-auto max-w-3xl text-3xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-4xl md:text-5xl">
				{church.name}
			</h1>
			<p class="hero-enter-delay-3 mx-auto mt-6 max-w-2xl text-lg text-slate-600 dark:text-slate-100">
				Details about the church and all our services.
			</p>
		</section>

		<!-- About the church -->
		<section class="border-t border-pink-200/50 section-separator bg-white/60 py-12 sm:py-16 px-4 sm:px-6 dark:bg-[#252d47]/80">
			<div class="mx-auto max-w-3xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-xl font-bold text-slate-900 dark:text-white sm:text-2xl">About the Church</h2>
					<p class="mt-4 text-slate-600 dark:text-slate-200 leading-relaxed">
						{displayAbout}
					</p>
					<dl class="mt-6 rounded-2xl border border-pink-200/50 card-premium bg-white/80 p-4 sm:p-6 dark:bg-[#1e2642]">
						<div>
							<dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Address</dt>
							<dd class="mt-1 font-medium text-slate-900 dark:text-white">{church.address}</dd>
						</div>
						<div class="mt-4">
							<dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Contact</dt>
							<dd class="mt-1">
								<a
									href="https://wa.me/{SITE.whatsappFull.replace(/\+/g, '')}"
									target="_blank"
									rel="noopener noreferrer"
									class="font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400 dark:hover:text-pink-300"
								>
									WhatsApp {SITE.whatsapp}
								</a>
								<span class="text-slate-500 dark:text-slate-400"> · </span>
								<a
									href="mailto:{SITE.contactEmail}"
									class="font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400 dark:hover:text-pink-300"
								>
									{SITE.contactEmail}
								</a>
							</dd>
						</div>
					</dl>
				</AnimateOnView>
			</div>
		</section>

		<!-- Our services -->
		<section class="border-t border-pink-200/50 section-separator bg-slate-50/80 py-12 sm:py-16 px-4 sm:px-6 dark:bg-[#1e2642]/60">
			<div class="mx-auto max-w-3xl">
				<AnimateOnView animation="fade-up">
					<h2 class="text-xl font-bold text-slate-900 dark:text-white sm:text-2xl">Our Services</h2>
					<p class="mt-2 text-slate-600 dark:text-slate-200">
						Join us for worship, teaching, and fellowship.
					</p>
				</AnimateOnView>
				<div class="mt-8 space-y-4 sm:space-y-6">
					{#each displayServices as service, i}
						<AnimateOnView animation="fade-up" stagger={i}>
							<article
								class="hover-lift flex flex-col sm:flex-row gap-4 sm:gap-6 rounded-2xl border border-pink-200/50 card-premium bg-white p-4 sm:p-6 shadow-sm hover:border-pink-300 hover:shadow-lg dark:bg-[#252d47] dark:border-pink-500/30 dark:hover:border-pink-500/50"
							>
								{#if service.image_url}
									<img
										src={churchImageUrl(service.image_url)}
										alt=""
										class="h-24 w-24 shrink-0 rounded-xl object-cover sm:h-28 sm:w-28"
									/>
								{:else}
									<div
										class="flex h-16 w-16 shrink-0 flex-col items-center justify-center rounded-xl bg-gradient-to-br from-pink-500 to-purple-600 text-center text-white"
									>
										<span class="text-xs font-medium uppercase leading-tight">{service.day_of_week}</span>
										<span class="text-sm font-semibold">{service.time}</span>
									</div>
								{/if}
								<div class="min-w-0 flex-1">
									<h3 class="text-lg font-bold text-slate-900 dark:text-white">{service.name}</h3>
									<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
										{service.day_of_week} · {service.time}
									</p>
									<p class="mt-2 text-slate-600 dark:text-slate-200">{service.description ?? ''}</p>
									{#if service.features && service.features.length > 0}
										<p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
											{service.features.join(' · ')}
										</p>
									{/if}
								</div>
							</article>
						</AnimateOnView>
					{/each}
				</div>
			</div>
		</section>

		<!-- CTA -->
		<section class="border-t border-pink-200/50 section-separator bg-white/60 py-12 sm:py-16 px-4 sm:px-6 dark:bg-[#252d47]/80">
			<div class="mx-auto max-w-2xl text-center">
				<AnimateOnView animation="fade-up">
					<h2 class="text-xl font-bold text-slate-900 dark:text-white sm:text-2xl">Visit Us</h2>
					<p class="mt-4 text-slate-600 dark:text-slate-200">
						We’d love to welcome you. For directions or more information, get in touch.
					</p>
					<div class="mt-6 flex flex-wrap justify-center gap-4">
						<a
							href="/contact"
							class="inline-flex items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700"
						>
							Contact us
						</a>
						<a
							href="https://wa.me/{SITE.whatsappFull.replace(/\+/g, '')}"
							target="_blank"
							rel="noopener noreferrer"
							class="inline-flex items-center justify-center gap-2 rounded-xl border-2 border-pink-300 bg-pink-50 px-6 py-3 text-sm font-semibold text-pink-600 transition hover:bg-pink-100 dark:border-pink-500/50 dark:bg-white/10 dark:text-pink-400 dark:hover:bg-white/15"
						>
							WhatsApp
						</a>
					</div>
				</AnimateOnView>
			</div>
		</section>
	</main>

	<Footer />
</div>
