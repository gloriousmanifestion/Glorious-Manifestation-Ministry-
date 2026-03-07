<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { SITE } from '$lib/config';
	import { userStore } from '$lib/auth';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';

	const navLinks = [
		{ href: '/', label: 'Home' },
		{ href: '/programs', label: 'Programs' },
		{ href: '/admissions', label: 'Admissions' },
		{ href: '/events', label: 'Events' },
		{ href: '/contact', label: 'Contact' }
	];

	let mobileOpen = $state(false);
	let scrolled = $state(false);
	const user = $derived($userStore);
	const isLoggedIn = $derived(user !== null);
	const pathname = $derived($page.url.pathname);

	function isActive(href: string): boolean {
		if (href === '/') return pathname === '/';
		return pathname === href || pathname.startsWith(href + '/');
	}

	onMount(() => {
		const checkScroll = () => {
			scrolled = typeof window !== 'undefined' && window.scrollY > 10;
		};
		checkScroll();
		window.addEventListener('scroll', checkScroll, { passive: true });
		return () => window.removeEventListener('scroll', checkScroll);
	});
</script>

<header
	class="fixed top-0 left-0 right-0 z-50 pt-[env(safe-area-inset-top)] transition-all duration-300 {scrolled || mobileOpen
		? 'border-b border-pink-200/50 section-separator bg-white/90 backdrop-blur-xl dark:bg-[#374266]/90'
		: 'border-b border-transparent bg-transparent dark:bg-transparent'}"
>
	<nav class="mx-auto flex max-w-6xl items-center justify-between px-4 py-3 sm:px-6 sm:py-4">
			<a
				href="/"
				class="inline-flex items-center gap-2 text-xl font-semibold tracking-tight text-slate-900 transition-all duration-300 hover:text-pink-600 dark:text-white dark:hover:text-pink-400"
			>
			<img
				src="/favicon.png"
				alt=""
				class="h-9 w-9 shrink-0 rounded-full object-cover"
				aria-hidden="true"
			/>
			<span class="sm:hidden">{SITE.schoolName}</span>
			<span class="hidden sm:inline">{SITE.shortName} | {SITE.schoolName}</span>
		</a>

		<!-- Desktop nav -->
		<div class="hidden items-center gap-1 md:flex">
			<ThemeToggle />
			{#each navLinks as link}
				<a
					href={link.href}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-all duration-300 {isActive(link.href)
						? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
						: 'text-slate-600 hover:bg-pink-50 hover:text-pink-600 dark:text-slate-100 dark:hover:bg-white/10 dark:hover:text-pink-400'}"
					aria-current={isActive(link.href) ? 'page' : undefined}
				>
					{link.label}
				</a>
			{/each}
			{#if isLoggedIn}
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

		<!-- Mobile menu button -->
		<button
			type="button"
			class="rounded-lg p-3 -m-1 text-slate-600 hover:bg-pink-50 hover:text-pink-600 active:bg-pink-100 dark:text-slate-100 dark:hover:bg-white/10 dark:hover:text-pink-400 md:hidden touch-manipulation"
			aria-expanded={mobileOpen}
			aria-controls="mobile-menu"
			onclick={() => (mobileOpen = !mobileOpen)}
		>
			{#if mobileOpen}
				<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
				</svg>
			{:else}
				<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
				</svg>
			{/if}
		</button>
	</nav>

	<!-- Mobile menu -->
	{#if mobileOpen}
		<div id="mobile-menu" class="border-t border-pink-200/50 section-separator bg-white px-4 py-4 pb-[max(1rem,env(safe-area-inset-bottom))] md:hidden max-h-[calc(100vh-5rem)] overflow-y-auto dark:bg-[#374266]">
			<div class="flex flex-col gap-0.5">
				{#each navLinks as link}
					<a
						href={link.href}
						class="rounded-xl px-4 py-3.5 text-base font-medium touch-manipulation {isActive(link.href)
							? 'bg-pink-50 text-pink-600 dark:bg-white/10 dark:text-pink-400'
							: 'text-slate-600 hover:bg-pink-50 hover:text-pink-600 active:bg-pink-100 dark:text-slate-100 dark:hover:bg-white/10 dark:hover:text-pink-400'}"
						aria-current={isActive(link.href) ? 'page' : undefined}
						onclick={() => (mobileOpen = false)}
					>
						{link.label}
					</a>
				{/each}
				{#if isLoggedIn}
					<a
						href="/dashboard"
						class="rounded-xl px-4 py-3.5 text-base font-medium touch-manipulation {isActive('/dashboard')
							? 'bg-pink-100 text-pink-700 dark:bg-white/15 dark:text-pink-300'
							: 'text-pink-600 active:bg-pink-50 dark:text-pink-400 dark:active:bg-white/10'}"
						aria-current={isActive('/dashboard') ? 'page' : undefined}
						onclick={() => (mobileOpen = false)}
					>
						Dashboard
					</a>
				{:else}
					<a href="/login" class="rounded-xl px-4 py-3.5 text-base font-medium text-slate-600 hover:text-pink-600 active:bg-pink-50 dark:text-slate-100 dark:hover:text-pink-400 touch-manipulation" onclick={() => (mobileOpen = false)}>
						Log in
					</a>
					<a href="/register" class="rounded-xl px-4 py-3.5 text-base font-medium text-pink-600 active:bg-pink-50 dark:text-pink-400 touch-manipulation" onclick={() => (mobileOpen = false)}>
						Register
					</a>
				{/if}
			</div>
			<div class="mt-3 pt-3 border-t border-slate-200 section-separator">
				<ThemeToggle />
			</div>
		</div>
	{/if}
</header>
