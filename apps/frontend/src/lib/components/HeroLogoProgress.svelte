<script lang="ts">
	import { onMount } from 'svelte';

	const size = 140;
	const stroke = 5;
	const cx = size / 2;
	// Progress path radius: outer edge of stroke sits on logo edge (no gap)
	const r = cx - stroke / 2;
	const circumference = 2 * Math.PI * r;

	let progress = $state(0);

	onMount(() => {
		const update = () => {
			if (typeof window === 'undefined' || typeof document === 'undefined') return;
			const { scrollTop, scrollHeight, clientHeight } = document.documentElement;
			const maxScroll = Math.max(0, scrollHeight - clientHeight);
			progress = maxScroll === 0 ? 0 : Math.min(1, scrollTop / maxScroll);
		};
		update();
		window.addEventListener('scroll', update, { passive: true });
		window.addEventListener('resize', update);
		return () => {
			window.removeEventListener('scroll', update);
			window.removeEventListener('resize', update);
		};
	});

	const dashOffset = $derived(circumference * (1 - progress));
</script>

<div
	class="hero-logo-progress fixed z-40 select-none pointer-events-none rounded-full overflow-hidden
		w-14 h-14 left-3 top-24
		sm:w-16 sm:h-16 sm:left-4 sm:top-24
		lg:w-[140px] lg:h-[140px] lg:left-6 lg:top-1/2 lg:-translate-y-1/2"
	aria-hidden="true"
>
	<!-- White circle behind logo so it reads clearly -->
	<div class="absolute inset-0 rounded-full bg-white dark:bg-white/95 shadow-sm lg:shadow-md" aria-hidden="true"></div>
	<!-- Logo fills the circle (same size as progress), no ring -->
	<img
		src="/favicon.png"
		alt=""
		class="absolute inset-0 h-full w-full rounded-full object-cover"
		width={size}
		height={size}
	/>
	<!-- Progress stroke sits on the edge of the logo, no gap -->
	<svg
		class="absolute inset-0 w-full h-full text-pink-500 dark:text-pink-400 pointer-events-none"
		width={size}
		height={size}
		viewBox="0 0 140 140"
	>
		<circle
			cx={cx}
			cy={cx}
			r={r}
			fill="none"
			stroke="currentColor"
			stroke-width={stroke}
			stroke-linecap="round"
			stroke-dasharray={circumference}
			stroke-dashoffset={dashOffset}
			transform="rotate(-90 70 70)"
			class="transition-[stroke-dashoffset] duration-150 drop-shadow-sm"
		/>
	</svg>
</div>
