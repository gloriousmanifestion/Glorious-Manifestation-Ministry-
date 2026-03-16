<script lang="ts">
	import type { Snippet } from 'svelte';

	/** Section type for icon and default copy. Use 'generic' for custom title/description. */
	type SectionType = 'blog' | 'testimonials' | 'gallery' | 'events' | 'resources' | 'faq' | 'generic';
	interface Props {
		type?: SectionType;
		title?: string;
		description?: string;
		action?: Snippet;
	}
	let { type = 'generic', title = '', description = '', action }: Props = $props();

	const config = $derived.by(() => {
		const t = type;
		if (title && description) return { title, description, icon: t === 'generic' ? 'generic' : t };
		const map: Record<string, { title: string; description: string; icon: string }> = {
			blog: {
				title: 'No posts yet',
				description: 'We’re preparing news and updates. Check back soon for articles and stories from the Pastoral School.',
				icon: 'blog'
			},
			testimonials: {
				title: 'Stories coming soon',
				description: 'We’re gathering testimonies from students and alumni. Return later to read what God is doing through our community.',
				icon: 'testimonials'
			},
			gallery: {
				title: 'No videos yet',
				description: 'Ministry videos and recordings will appear here. Check back for worship, teaching, and moments from our services.',
				icon: 'gallery'
			},
			events: {
				title: 'No upcoming events',
				description: 'New events will be listed here. Stay tuned for school openings, revivals, graduation, and weekly chapel.',
				icon: 'events'
			},
			resources: {
				title: 'No resources yet',
				description: 'Downloads, links, and course materials will be added here. Check back or contact us if you need something specific.',
				icon: 'resources'
			},
			faq: {
				title: 'FAQ coming soon',
				description: 'We’re preparing answers to common questions. In the meantime, feel free to contact us with any enquiries.',
				icon: 'faq'
			},
			generic: {
				title: title || 'Nothing here yet',
				description: description || 'Content will appear here soon.',
				icon: 'generic'
			}
		};
		const c = map[t] ?? map.generic;
		return { title: title || c.title, description: description || c.description, icon: c.icon };
	});

	const iconPath = $derived.by(() => {
		switch (config.icon) {
			case 'blog':
				return 'M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h2a2 2 0 012-2h6a2 2 0 012 2h2a2 2 0 012 2v12a2 2 0 01-2 2zm-7-8v4m0-4l2 2m-2-2l-2 2';
			case 'testimonials':
				return 'M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z';
			case 'gallery':
				return 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z';
			case 'events':
				return 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z';
			case 'resources':
				return 'M5 19a2 2 0 01-2 2H3a1 1 0 01-1-1V7a1 1 0 011-1h2a2 2 0 012 2v1m0 16V5a2 2 0 104 0v14m0-16v1a2 2 0 104 0V5m0 16H9m3 0h3';
			case 'faq':
				return 'M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
			default:
				return 'M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m-4 0v4m0-4h4m-4 0H6';
		}
	});
</script>

<div
	class="empty-section flex flex-col items-center justify-center rounded-2xl border border-slate-200/80 bg-gradient-to-b from-slate-50/95 to-white/95 px-8 py-14 text-center shadow-sm dark:border-white/10 dark:from-[#1e2642]/95 dark:to-[#252d47]/95"
	role="status"
	aria-label="{config.title}"
>
	<div
		class="mb-5 flex h-16 w-16 items-center justify-center rounded-2xl bg-gradient-to-br from-pink-100 to-purple-100 dark:from-pink-500/20 dark:to-purple-500/20"
		aria-hidden="true"
	>
		<svg
			class="h-8 w-8 text-pink-500 dark:text-pink-400"
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke-linecap="round"
			stroke-linejoin="round"
		>
			<path d={iconPath} />
		</svg>
	</div>
	<h2 class="text-xl font-semibold text-slate-800 dark:text-white sm:text-2xl">
		{config.title}
	</h2>
	<p class="mt-3 max-w-md text-slate-600 dark:text-slate-300 leading-relaxed">
		{config.description}
	</p>
	<div class="mt-6 h-px w-16 rounded-full bg-gradient-to-r from-transparent via-pink-300/80 to-transparent dark:via-pink-500/50" role="presentation"></div>
	{#if action}
		<div class="mt-6">
			{@render action()}
		</div>
	{/if}
</div>
