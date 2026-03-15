<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { SITE } from '$lib/config';
	import type { Resource } from '$lib/api/resources';
	import EmptySection from '$lib/components/EmptySection.svelte';

	interface Data {
		resources: Resource[];
		courseId: string | null;
	}

	let { data }: { data: Data } = $props();

	function typeLabel(t: string): string {
		if (t === 'pdf') return 'PDF';
		if (t === 'video') return 'Video';
		return 'Link';
	}
</script>

<svelte:head>
	<title>Resource library — {SITE.schoolName}</title>
	<meta name="description" content="Downloads, links, and resources for students." />
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Nav />

	<main class="flex-1 pt-16 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-4xl px-4 sm:px-6">
			<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-3xl">
				Resource library
			</h1>
			<p class="mt-2 text-slate-600 dark:text-slate-200">
				Links, PDFs, and videos for your course.
			</p>

			{#if data.resources.length === 0}
				<div class="mt-12">
					<EmptySection type="resources" />
				</div>
			{:else}
				<ul class="mt-8 space-y-4" role="list">
					{#each data.resources as res}
						<li
							class="rounded-xl border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800/50 p-4 shadow-sm"
						>
							<a
								href={res.url}
								target="_blank"
								rel="noopener noreferrer"
								class="flex items-start gap-4 group"
							>
								<span
									class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-pink-100 dark:bg-pink-900/40 text-pink-600 dark:text-pink-400"
								>
									{res.resource_type === 'pdf'
										? 'PDF'
										: res.resource_type === 'video'
											? '▶'
											: '↗'}
								</span>
								<div class="min-w-0 flex-1">
									<p class="font-medium text-slate-900 dark:text-white group-hover:text-pink-600 dark:group-hover:text-pink-400">
										{res.title}
									</p>
									<p class="mt-0.5 text-sm text-slate-500 dark:text-slate-400">
										{typeLabel(res.resource_type)}
									</p>
								</div>
								<svg
									class="h-5 w-5 shrink-0 text-slate-400 group-hover:text-pink-500"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
									/>
								</svg>
							</a>
						</li>
					{/each}
				</ul>
			{/if}
		</div>
	</main>

	<Footer />
</div>
