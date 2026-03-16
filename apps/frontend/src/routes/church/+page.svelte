<script lang="ts">
	import { onMount } from 'svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { SITE } from '$lib/config';
	import { getChurchServices, getChurchContent, getChurchVideos, churchImageUrl, churchVideoUrl } from '$lib/api/church';
	import type { ChurchService, ChurchVideo } from '$lib/api/church';
	import EmptySection from '$lib/components/EmptySection.svelte';
	import { pageSeo } from '$lib/stores/pageSeo';

	const church = SITE.church;

	pageSeo.set({
		title: `Church — ${SITE.name} Pastoral School`,
		description: `Learn about ${church.shortName}: church details, service times, and how we worship and serve together in ${SITE.location}.`
	});

	let services = $state<ChurchService[]>([]);
	let aboutContent = $state<string | null>(null);
	let videos = $state<ChurchVideo[]>([]);
	let loaded = $state(false);

	onMount(async () => {
		try {
			const [list, about, videoList] = await Promise.all([
				getChurchServices(),
				getChurchContent('church_about'),
				getChurchVideos()
			]);
			services = list;
			aboutContent = about;
			videos = videoList;
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

	/** Video href: external URL or uploaded file URL */
	function videoHref(v: ChurchVideo): string {
		if (v.url && v.url.trim()) return v.url;
		if (v.file_path) return churchVideoUrl(v.file_path);
		return '#';
	}
</script>

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
									WhatsApp {SITE.whatsappDisplay}
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
								class="hover-lift flex flex-col sm:flex-row gap-4 sm:gap-6 overflow-hidden rounded-2xl border border-pink-200/50 card-premium bg-white p-4 sm:p-6 shadow-sm hover:border-pink-300 hover:shadow-lg dark:bg-[#252d47] dark:border-pink-500/30 dark:hover:border-pink-500/50"
							>
								{#if service.image_url}
									<img
										src={churchImageUrl(service.image_url)}
										alt=""
										class="h-24 w-24 shrink-0 rounded-xl object-cover sm:h-28 sm:w-28"
									/>
								{:else}
									<div
										class="flex h-16 w-16 shrink-0 flex-col items-center justify-center gap-0.5 overflow-hidden rounded-xl bg-gradient-to-br from-pink-500 to-purple-600 px-1 text-center text-white"
										title="{service.day_of_week} · {service.time}"
									>
										<span class="max-w-full truncate text-[10px] font-medium uppercase leading-tight sm:text-xs">{service.day_of_week}</span>
										<span class="max-w-full truncate text-xs font-semibold sm:text-sm">{service.time}</span>
									</div>
								{/if}
								<div class="min-w-0 flex-1 overflow-hidden">
									<h3 class="break-words text-lg font-bold text-slate-900 dark:text-white">{service.name}</h3>
									<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
										{service.day_of_week} · {service.time}
									</p>
									<p class="mt-2 break-words text-slate-600 dark:text-slate-200">{service.description ?? ''}</p>
									{#if service.features && service.features.length > 0}
										<p class="mt-2 break-words text-sm text-slate-500 dark:text-slate-400">
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

		<!-- Video gallery -->
		{#if loaded}
		<section class="border-t border-pink-200/50 section-separator bg-gradient-to-b from-slate-50 to-white py-14 sm:py-20 px-4 sm:px-6 dark:from-[#1e2642]/80 dark:to-[#252d47]/80">
			<div class="mx-auto max-w-5xl">
				<AnimateOnView animation="fade-up">
					<div class="text-center">
						<p class="text-sm font-medium uppercase tracking-[0.2em] text-pink-600 dark:text-pink-400">Watch & share</p>
						<h2 class="mt-2 text-2xl font-bold text-slate-900 dark:text-white sm:text-3xl">Ministry videos</h2>
						<p class="mx-auto mt-3 max-w-xl text-slate-600 dark:text-slate-200">
							Worship, teaching, and moments from our services. Click any card to watch.
						</p>
					</div>
				</AnimateOnView>
				{#if videos.length > 0}
					<div class="mt-10 grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-3 lg:gap-8">
						{#each videos as video, i}
							<AnimateOnView animation="fade-up" stagger={i}>
								<a
									href={videoHref(video)}
									target="_blank"
									rel="noopener noreferrer"
									class="group relative block overflow-hidden rounded-2xl border border-pink-200/50 bg-white/90 shadow-md transition-all duration-300 hover:scale-[1.02] hover:border-pink-400/60 hover:shadow-xl hover:shadow-pink-500/10 dark:border-pink-500/30 dark:bg-[#252d47]/90 dark:hover:border-pink-500/60 dark:hover:shadow-pink-500/10"
								>
									<!-- Card aspect ratio -->
									<div class="relative aspect-video w-full bg-gradient-to-br from-pink-500/20 via-purple-500/20 to-slate-600/20 dark:from-pink-600/30 dark:via-purple-600/30 dark:to-slate-700/30">
										<!-- Play overlay -->
										<div class="absolute inset-0 flex items-center justify-center">
											<div class="flex h-14 w-14 items-center justify-center rounded-full bg-white/95 text-pink-600 shadow-lg ring-4 ring-pink-200/50 transition group-hover:scale-110 group-hover:bg-pink-50 group-hover:text-pink-700 dark:bg-white/90 dark:ring-pink-500/30 dark:group-hover:bg-pink-100/90">
												<svg class="ml-1 h-6 w-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
													<path d="M8 5v14l11-7L8 5z" />
												</svg>
											</div>
										</div>
									</div>
									<div class="px-4 py-3 text-center">
										<span class="text-sm font-medium text-slate-700 dark:text-slate-200 group-hover:text-pink-600 dark:group-hover:text-pink-400">{video.title}</span>
										<span class="mt-1 block text-xs text-slate-500 dark:text-slate-400">{video.url ? 'Watch video →' : 'Play →'}</span>
									</div>
								</a>
							</AnimateOnView>
						{/each}
					</div>
				{:else}
					<div class="mt-10">
						<EmptySection type="gallery" />
					</div>
				{/if}
			</div>
		</section>
		{/if}

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
