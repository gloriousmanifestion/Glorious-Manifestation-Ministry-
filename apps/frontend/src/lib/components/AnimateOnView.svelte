<script lang="ts">
	import { onMount } from 'svelte';

	interface Props {
		/** Animation variant */
		animation?: 'fade-up' | 'fade-down' | 'fade' | 'scale' | 'slide-left' | 'slide-right';
		/** Delay before animation starts (ms) */
		delay?: number;
		/** Duration in ms */
		duration?: number;
		/** Stagger index for list items */
		stagger?: number;
		/** Root margin for Intersection Observer */
		rootMargin?: string;
		/** Threshold 0-1 */
		threshold?: number;
		/** Additional class for the wrapper */
		class?: string;
	}

	let {
		animation = 'fade-up',
		delay = 0,
		duration = 600,
		stagger = 0,
		rootMargin = '0px 0px -60px 0px',
		threshold = 0.1,
		class: className = '',
		children
	}: Props & { children?: import('svelte').Snippet } = $props();

	let visible = $state(false);
	let el: HTMLElement;

	onMount(() => {
		const observer = new IntersectionObserver(
			([entry]) => {
				visible = entry.isIntersecting;
			},
			{ rootMargin, threshold }
		);
		if (el) observer.observe(el);
		return () => observer.disconnect();
	});
</script>

<div
	bind:this={el}
	class="animate-on-view {animation} {className}"
	class:visible
	style="--anim-delay: {delay + stagger * 100}ms; --anim-duration: {duration}ms;"
>
	{@render children?.()}
</div>
