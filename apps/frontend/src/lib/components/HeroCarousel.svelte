<script lang="ts">
	import { onMount } from 'svelte';

	interface Slide {
		title: string;
		subtitle: string;
		gradient: string;
		icon?: string;
	}

	interface Props {
		slides: Slide[];
		/** Auto-advance interval in ms */
		interval?: number;
		class?: string;
	}

	let { slides, interval = 5000, class: className = '' }: Props = $props();

	let currentIndex = $state(0);
	let isTransitioning = $state(false);
	let intervalId: ReturnType<typeof setInterval>;

	function goTo(index: number) {
		if (isTransitioning) return;
		isTransitioning = true;
		currentIndex = index;
		setTimeout(() => (isTransitioning = false), 500);
		// Reset auto-advance timer on manual interaction
		clearInterval(intervalId);
		intervalId = setInterval(next, interval);
	}

	function next() {
		goTo((currentIndex + 1) % slides.length);
	}

	onMount(() => {
		intervalId = setInterval(next, interval);
		return () => clearInterval(intervalId);
	});
</script>

<div
	class="hero-carousel relative min-h-[50vh] sm:min-h-[55vh] overflow-hidden rounded-2xl {className}"
	role="region"
	aria-label="Hero carousel"
>
	{#each slides as slide, i}
		<div
			class="absolute inset-0 flex flex-col items-center justify-center px-6 py-16 text-center transition-opacity duration-500"
			class:opacity-0={currentIndex !== i}
			class:opacity-100={currentIndex === i}
			class:pointer-events-none={currentIndex !== i}
			style="background: {slide.gradient};"
			role="group"
			aria-roledescription="slide"
			aria-label="Slide {i + 1} of {slides.length}"
		>
			{#if slide.icon}
				<div class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-white/20 text-4xl backdrop-blur-sm">
					{slide.icon}
				</div>
			{/if}
			<h2 class="text-2xl font-bold tracking-tight text-white drop-shadow-sm sm:text-3xl md:text-4xl">
				{slide.title}
			</h2>
			<p class="mt-3 max-w-xl text-base text-white/90 sm:text-lg">
				{slide.subtitle}
			</p>
		</div>
	{/each}

	<!-- Dot indicators -->
	<div class="absolute bottom-4 left-0 right-0 flex justify-center gap-2" role="tablist" aria-label="Carousel navigation">
		{#each slides as _, i}
			<button
				type="button"
				class="h-2.5 w-2.5 rounded-full transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-transparent {currentIndex === i ? 'bg-white scale-125' : 'bg-white/50'}"
				aria-label="Go to slide {i + 1}"
				aria-selected={currentIndex === i}
				role="tab"
				onclick={() => goTo(i)}
			></button>
		{/each}
	</div>

	<!-- Prev/Next buttons (optional, for accessibility) -->
	<button
		type="button"
		class="absolute left-2 top-1/2 -translate-y-1/2 rounded-full bg-white/20 p-2 text-white backdrop-blur-sm transition hover:bg-white/30 focus:outline-none focus:ring-2 focus:ring-white"
		aria-label="Previous slide"
		onclick={() => goTo((currentIndex - 1 + slides.length) % slides.length)}
	>
		<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
		</svg>
	</button>
	<button
		type="button"
		class="absolute right-2 top-1/2 -translate-y-1/2 rounded-full bg-white/20 p-2 text-white backdrop-blur-sm transition hover:bg-white/30 focus:outline-none focus:ring-2 focus:ring-white"
		aria-label="Next slide"
		onclick={next}
	>
		<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
		</svg>
	</button>
</div>

