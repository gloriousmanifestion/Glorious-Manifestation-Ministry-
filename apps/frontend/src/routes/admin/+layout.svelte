<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { userStore, authReady } from '$lib/auth';
	import { SITE } from '$lib/config';

	let mounted = $state(false);
	const user = $derived($userStore);
	const pathname = $derived($page.url.pathname);

	const navItems = $derived([
		{ href: '/admin', label: 'Dashboard', icon: 'chart' as const },
		{ href: '/admin/users', label: 'Users', icon: 'users' as const },
		{ href: '/admin/courses', label: 'Courses', icon: 'book' as const },
		{ href: '/admin/learning-plans', label: 'Learning plans', icon: 'academic' as const },
		...(user?.role === 'super_admin' ? [{ href: '/admin/team', label: 'Team & invites', icon: 'invite' as const }] : [])
	]);

	$effect(() => {
		if (!mounted) return;
		if (!$authReady) return;
		if (!user) {
			goto('/login?redirect=' + encodeURIComponent(pathname));
			return;
		}
		if (user.role !== 'admin' && user.role !== 'super_admin') {
			goto('/dashboard');
		}
	});

	onMount(() => {
		mounted = true;
	});

	function isActive(href: string): boolean {
		if (href === '/admin') return pathname === '/admin';
		return pathname === href || pathname.startsWith(href + '/');
	}

	function iconSvg(icon: string): string {
		switch (icon) {
			case 'chart':
				return 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z';
			case 'users':
				return 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z';
			case 'book':
				return 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253';
			case 'academic':
				return 'M12 14l9-5-9-5-9 5 9 5zm0 0l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14z';
			case 'invite':
				return 'M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z';
			default:
				return 'M4 6h16M4 12h16M4 18h16';
		}
	}
</script>

<svelte:head>
	<title>Admin — {SITE.schoolName}</title>
</svelte:head>

{#if !mounted || !$authReady}
	<div class="flex min-h-screen items-center justify-center bg-slate-950">
		<div class="h-10 w-10 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div>
	</div>
{:else if !user}
	<!-- Redirecting to login -->
	<div class="flex min-h-screen items-center justify-center bg-slate-950">
		<p class="text-slate-400">Redirecting to login…</p>
	</div>
{:else if user.role !== 'admin' && user.role !== 'super_admin'}
	<!-- Not an admin: redirect to student dashboard -->
	<div class="flex min-h-screen items-center justify-center bg-slate-950">
		<p class="text-slate-400">Redirecting to dashboard…</p>
	</div>
{:else}
	<div class="flex min-h-screen bg-slate-950 text-slate-200">
		<!-- Sidebar -->
		<aside
			class="fixed inset-y-0 left-0 z-40 flex w-64 flex-col border-r border-slate-800 bg-slate-900/95 backdrop-blur"
		>
			<div class="flex h-14 items-center gap-2 border-b border-slate-800 px-4">
				<a href="/" class="flex items-center gap-2 font-semibold text-white">
					<span class="text-pink-400">{SITE.shortName}</span>
					<span class="text-slate-400">Admin</span>
				</a>
			</div>
			<nav class="flex-1 space-y-0.5 p-3">
				{#each navItems as item}
					<a
						href={item.href}
						class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition {isActive(item.href)
							? 'bg-pink-500/20 text-pink-300'
							: 'text-slate-400 hover:bg-slate-800 hover:text-slate-200'}"
					>
						<svg
							class="h-5 w-5 shrink-0"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d={iconSvg(item.icon)}
							/>
						</svg>
						{item.label}
					</a>
				{/each}
			</nav>
			<div class="border-t border-slate-800 p-3">
				<div class="rounded-lg bg-slate-800/50 px-3 py-2 text-xs text-slate-500">
					{user.name}
					<span class="ml-1 text-pink-400">{user.role === 'super_admin' ? ' (Super Admin)' : ' (Admin)'}</span>
				</div>
				<a
					href="/dashboard"
					class="mt-2 flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-slate-400 hover:bg-slate-800 hover:text-slate-200"
				>
					← Back to site
				</a>
			</div>
		</aside>

		<!-- Main content -->
		<main class="flex-1 pl-64">
			<div class="min-h-screen px-6 py-8">
				<slot />
			</div>
		</main>
	</div>
{/if}
