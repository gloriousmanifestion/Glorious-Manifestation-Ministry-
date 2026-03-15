<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { SITE } from '$lib/config';
	import { userStore } from '$lib/auth';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';

	// Desktop: main nav + Discover mega menu (FAQ, Testimonials, Blog, Events) + Church, Contact.
	const desktopMainLinks = [
		{ href: '/', label: 'Home', icon: 'home' },
		{ href: '/programs', label: 'Programs', icon: 'book' },
		{ href: '/admissions', label: 'Admissions', icon: 'document' }
	];
	const discoverLinks = [
		{ href: '/events', label: 'Events', icon: 'calendar' },
		{ href: '/blog', label: 'Blog', icon: 'blog' },
		{ href: '/testimonials', label: 'Testimonials', icon: 'quote' },
		{ href: '/faq', label: 'FAQ', icon: 'faq' }
	];
	const desktopEndLinks = [
		{ href: '/church', label: 'Church', icon: 'church' },
		{ href: '/contact', label: 'Contact', icon: 'envelope' }
	];
	// Mobile: same flat list for bottom bar (first 4) + More sheet
	const navLinks = [
		...desktopMainLinks,
		...discoverLinks,
		...desktopEndLinks
	];
	const mobileNavLinks = navLinks.slice(0, 4);
	const pathname = $derived($page.url.pathname);

	function isActive(href: string): boolean {
		if (href === '/') return pathname === '/';
		return pathname === href || pathname.startsWith(href + '/');
	}

	const isDiscoverActive = $derived(
		discoverLinks.some((l) => isActive(l.href))
	);

	let discoverOpen = $state(false);
	let discoverTimeout: ReturnType<typeof setTimeout> | null = null;
	function openDiscover() {
		if (discoverTimeout) clearTimeout(discoverTimeout);
		discoverOpen = true;
	}
	function closeDiscover() {
		discoverTimeout = setTimeout(() => (discoverOpen = false), 120);
	}
	function keepDiscoverOpen() {
		if (discoverTimeout) clearTimeout(discoverTimeout);
	}

	let sheetOpen = $state(false);
	let scrolled = $state(false);
	const user = $derived($userStore);
	const isLoggedIn = $derived(user !== null);
	const canAccessAdmin = $derived(user !== null && (user.role === 'admin' || user.role === 'super_admin'));

	function iconPath(icon: string): string {
		switch (icon) {
			case 'home':
				return 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6';
			case 'book':
				return 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253';
			case 'document':
				return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
			case 'calendar':
				return 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z';
			case 'envelope':
				return 'M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
			case 'church':
				return 'M12 2L4 6v2h2v10H4v2h4v-4h4v4h4v-2h-2V8h2V6l-8-4zm0 2.5l5 2.5v1H7v-1l5-2.5zM7 11h2v2H7v-2zm6 0h2v2h-2v-2z';
			case 'faq':
				return 'M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
			case 'quote':
				return 'M7 8h10m-5 8v-4m0-4V8m5 4l3 3m0 0l3-3m-3 3V4';
			case 'blog':
				return 'M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z';
			default:
				return 'M4 6h16M4 12h16M4 18h16';
		}
	}

	$effect(() => {
		if (!sheetOpen) return;
		const onKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape') sheetOpen = false;
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	});

	onMount(() => {
		const checkScroll = () => {
			scrolled = typeof window !== 'undefined' && window.scrollY > 10;
		};
		checkScroll();
		window.addEventListener('scroll', checkScroll, { passive: true });
		return () => window.removeEventListener('scroll', checkScroll);
	});
</script>

<!-- Desktop: top header (full width) -->
<header
	class="fixed top-0 left-0 right-0 z-50 pt-[env(safe-area-inset-top)] transition-all duration-300 hidden md:block {scrolled
		? 'border-b border-pink-200/50 section-separator bg-white/90 backdrop-blur-xl dark:bg-[#252d47]/90'
		: 'border-b border-transparent bg-transparent dark:bg-transparent'}"
>
	<nav class="mx-auto flex w-full max-w-[100rem] items-center justify-between gap-6 px-4 py-3 sm:px-6 sm:py-4">
		<a
			href="/"
			class="inline-flex shrink-0 items-center gap-2 text-xl font-semibold tracking-tight text-slate-900 transition-all duration-300 hover:text-pink-600 dark:text-white dark:hover:text-pink-400"
		>
			<img
				src="/favicon.png"
				alt=""
				class="h-9 w-9 shrink-0 rounded-full object-cover"
				aria-hidden="true"
			/>
			<span class="hidden sm:inline">{SITE.shortName} | {SITE.schoolName}</span>
		</a>

		<div class="flex min-w-0 flex-1 items-center justify-center gap-0.5">
			{#each desktopMainLinks as link}
				<a
					href={link.href}
					class="rounded-xl px-4 py-2.5 text-sm font-medium transition-all duration-300 {isActive(link.href)
						? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
						: 'text-slate-600 hover:bg-pink-50 hover:text-pink-600 dark:text-slate-100 dark:hover:bg-white/10 dark:hover:text-pink-400'}"
					aria-current={isActive(link.href) ? 'page' : undefined}
				>
					{link.label}
				</a>
			{/each}
			<!-- Discover mega menu (FAQ, Testimonials, Blog, Events) -->
			<div
				class="relative"
				role="group"
				aria-label="Discover menu"
				onmouseenter={openDiscover}
				onmouseleave={closeDiscover}
				onfocusin={openDiscover}
				onfocusout={closeDiscover}
			>
				<button
					type="button"
					class="rounded-xl px-4 py-2.5 text-sm font-medium transition-all duration-300 flex items-center gap-1 {discoverOpen || isDiscoverActive
						? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
						: 'text-slate-600 hover:bg-pink-50 hover:text-pink-600 dark:text-slate-100 dark:hover:bg-white/10 dark:hover:text-pink-400'}"
					aria-expanded={discoverOpen}
					aria-haspopup="true"
					aria-label="Discover: Events, Blog, Testimonials, FAQ"
				>
					Discover
					<svg class="h-4 w-4 transition-transform {discoverOpen ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
				{#if discoverOpen}
					<div
						class="absolute left-1/2 top-full z-50 mt-1 w-[min(28rem,90vw)] -translate-x-1/2 rounded-2xl border border-pink-200/50 bg-white/95 p-3 shadow-xl backdrop-blur-xl dark:border-pink-500/30 dark:bg-[#252d47]/95"
						role="group"
						aria-label="Discover links"
						onmouseenter={keepDiscoverOpen}
					>
						<div class="grid grid-cols-2 gap-1">
							{#each discoverLinks as link}
								<a
									href={link.href}
									class="flex items-center gap-3 rounded-xl px-4 py-3 text-left text-sm font-medium transition {isActive(link.href)
										? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
										: 'text-slate-700 hover:bg-pink-50 hover:text-pink-600 dark:text-slate-200 dark:hover:bg-white/10 dark:hover:text-pink-400'}"
									aria-current={isActive(link.href) ? 'page' : undefined}
								>
									<svg class="h-5 w-5 shrink-0 text-pink-500/80 dark:text-pink-400/80" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={iconPath(link.icon)} />
									</svg>
									{link.label}
								</a>
							{/each}
						</div>
					</div>
				{/if}
			</div>
			{#each desktopEndLinks as link}
				<a
					href={link.href}
					class="rounded-xl px-4 py-2.5 text-sm font-medium transition-all duration-300 {isActive(link.href)
						? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
						: 'text-slate-600 hover:bg-pink-50 hover:text-pink-600 dark:text-slate-100 dark:hover:bg-white/10 dark:hover:text-pink-400'}"
					aria-current={isActive(link.href) ? 'page' : undefined}
				>
					{link.label}
				</a>
			{/each}
		</div>

		<div class="flex shrink-0 items-center gap-1">
			<ThemeToggle />
			{#if isLoggedIn}
				{#if canAccessAdmin}
					<a
						href="/admin"
						class="ml-2 rounded-lg px-4 py-2.5 text-sm font-medium text-slate-600 transition hover:bg-pink-50 hover:text-pink-600 dark:text-slate-300 dark:hover:bg-white/10 dark:hover:text-pink-400"
						aria-current={pathname.startsWith('/admin') ? 'page' : undefined}
					>
						Admin
					</a>
				{/if}
				<a
					href="/dashboard"
					class="ml-2 rounded-full px-5 py-2.5 text-sm font-semibold transition {isActive('/dashboard')
						? 'border-2 border-pink-500 bg-pink-100 text-pink-700 dark:border-pink-400 dark:bg-white/15 dark:text-pink-300'
						: 'border border-pink-300 bg-pink-50 text-pink-600 hover:bg-pink-100 dark:border-pink-500/50 dark:bg-white/10 dark:text-pink-400 dark:hover:bg-white/15'}"
					aria-current={isActive('/dashboard') ? 'page' : undefined}
				>
					Dashboard
				</a>
			{:else}
				<a
					href="/login"
					class="ml-2 rounded-lg px-4 py-2.5 text-sm font-medium text-slate-600 transition hover:bg-pink-50 hover:text-pink-600 dark:text-slate-100 dark:hover:bg-white/10 dark:hover:text-pink-400"
				>
					Log in
				</a>
				<a
					href="/register"
					class="ml-2 rounded-full border border-pink-300 bg-pink-50 px-5 py-2.5 text-sm font-semibold text-pink-600 transition hover:bg-pink-100 dark:border-pink-500/50 dark:bg-white/10 dark:text-pink-400 dark:hover:bg-white/15"
				>
					Register
				</a>
			{/if}
		</div>
	</nav>
</header>

<!-- Mobile: top bar (logo only) -->
<header
	class="fixed top-0 left-0 right-0 z-40 flex items-center justify-center pt-[env(safe-area-inset-top)] pb-3 md:hidden {scrolled
		? 'border-b border-pink-200/50 bg-white/90 backdrop-blur-xl dark:bg-[#252d47]/90'
		: 'border-b border-transparent bg-transparent dark:bg-transparent'}"
>
	<a
		href="/"
		class="inline-flex items-center gap-2 text-lg font-semibold tracking-tight text-slate-900 transition hover:text-pink-600 dark:text-white dark:hover:text-pink-400"
	>
		<img src="/favicon.png" alt="" class="h-8 w-8 shrink-0 rounded-full object-cover" aria-hidden="true" />
		<span>{SITE.schoolName}</span>
	</a>
</header>

<!-- Mobile: bottom nav (exactly 5 items: 4 links + More) -->
<nav
	class="mobile-bottom-nav fixed bottom-0 left-0 right-0 z-50 flex w-full min-w-0 items-center justify-around border-t border-pink-200/50 bg-white/95 pb-[env(safe-area-inset-bottom)] pt-2 backdrop-blur-xl md:hidden dark:border-slate-600 dark:bg-[#252d47]/95"
	aria-label="Main navigation"
>
	{#each mobileNavLinks as link}
		<a
			href={link.href}
			class="flex min-w-0 flex-1 basis-0 flex-col items-center gap-0.5 rounded-lg px-1 py-1.5 text-[10px] font-medium transition touch-manipulation {isActive(link.href)
				? 'text-pink-600 dark:text-pink-400'
				: 'text-slate-500 active:bg-pink-50 dark:text-slate-300 dark:active:bg-white/10'}"
			aria-current={isActive(link.href) ? 'page' : undefined}
		>
			<svg class="h-5 w-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={iconPath(link.icon)} />
			</svg>
			<span class="truncate max-w-full">{link.label}</span>
		</a>
	{/each}
	<button
		type="button"
		class="flex min-w-0 flex-1 basis-0 flex-col items-center gap-0.5 rounded-lg px-1 py-1.5 text-[10px] font-medium text-slate-500 transition touch-manipulation active:bg-pink-50 dark:text-slate-300 dark:active:bg-white/10"
		aria-expanded={sheetOpen}
		aria-label="More menu"
		onclick={() => (sheetOpen = true)}
	>
		<svg class="h-5 w-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
		</svg>
		<span>More</span>
	</button>
</nav>

<!-- Mobile: pull-up sheet (theme + auth) -->
{#if sheetOpen}
	<div
		class="fixed inset-0 z-[60] md:hidden"
		aria-modal="true"
		role="dialog"
		aria-label="More options"
	>
		<button
			type="button"
			class="absolute inset-0 bg-slate-900/50 backdrop-blur-sm transition-opacity"
			aria-label="Close menu"
			onclick={(e) => e.target === e.currentTarget && (sheetOpen = false)}
		></button>
		<div
			class="nav-sheet absolute bottom-0 left-0 right-0 rounded-t-2xl border-t border-pink-200/50 bg-white pb-[env(safe-area-inset-bottom)] dark:border-slate-600 dark:bg-[#252d47]"
		>
			<div class="flex justify-center pt-3 pb-2">
				<span class="h-1 w-12 rounded-full bg-slate-300 dark:bg-slate-500" aria-hidden="true"></span>
			</div>
			<div class="max-h-[60vh] overflow-y-auto overflow-x-hidden px-4 pb-4">
				<p class="mb-3 text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-400">
					More
				</p>
				<div class="mb-4 flex flex-col gap-1">
					<a
						href="/church"
						class="rounded-xl px-4 py-3.5 text-base font-medium touch-manipulation {isActive('/church')
							? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
							: 'text-slate-700 hover:bg-pink-50 dark:text-slate-100 dark:hover:bg-white/10'}"
						aria-current={isActive('/church') ? 'page' : undefined}
						onclick={() => (sheetOpen = false)}
					>
						Church
					</a>
					<a
						href="/contact"
						class="rounded-xl px-4 py-3.5 text-base font-medium touch-manipulation {isActive('/contact')
							? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
							: 'text-slate-700 hover:bg-pink-50 dark:text-slate-100 dark:hover:bg-white/10'}"
						aria-current={isActive('/contact') ? 'page' : undefined}
						onclick={() => (sheetOpen = false)}
					>
						Contact
					</a>
				</div>
				<p class="mb-3 text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-400">
					Theme
				</p>
				<div class="mb-6 flex justify-start">
					<ThemeToggle />
				</div>
				<p class="mb-3 text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-400">
					Account
				</p>
				<div class="flex flex-col gap-1">
					{#if isLoggedIn}
						{#if canAccessAdmin}
							<a
								href="/admin"
								class="rounded-xl px-4 py-3.5 text-base font-medium touch-manipulation {pathname.startsWith('/admin')
									? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
									: 'text-slate-700 hover:bg-pink-50 dark:text-slate-100 dark:hover:bg-white/10'}"
								aria-current={pathname.startsWith('/admin') ? 'page' : undefined}
								onclick={() => (sheetOpen = false)}
							>
								Admin
							</a>
						{/if}
						<a
							href="/dashboard"
							class="rounded-xl px-4 py-3.5 text-base font-medium touch-manipulation {isActive('/dashboard')
								? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
								: 'text-slate-700 hover:bg-pink-50 dark:text-slate-100 dark:hover:bg-white/10'}"
							aria-current={isActive('/dashboard') ? 'page' : undefined}
							onclick={() => (sheetOpen = false)}
						>
							Dashboard
						</a>
					{:else}
						<a
							href="/login"
							class="rounded-xl px-4 py-3.5 text-base font-medium text-slate-700 hover:bg-pink-50 touch-manipulation dark:text-slate-100 dark:hover:bg-white/10"
							onclick={() => (sheetOpen = false)}
						>
							Log in
						</a>
						<a
							href="/register"
							class="rounded-xl px-4 py-3.5 text-base font-medium text-pink-600 hover:bg-pink-50 touch-manipulation dark:text-pink-400 dark:hover:bg-white/10"
							onclick={() => (sheetOpen = false)}
						>
							Register
						</a>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.mobile-bottom-nav {
		overflow-x: hidden;
	}
	.nav-sheet {
		animation: nav-sheet-in 0.25s ease-out;
	}
	@keyframes nav-sheet-in {
		from {
			transform: translateY(100%);
			opacity: 0.9;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}
</style>
