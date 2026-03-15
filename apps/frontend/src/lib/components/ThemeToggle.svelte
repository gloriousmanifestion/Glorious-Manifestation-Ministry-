<script lang="ts">
	import { themeStore, setTheme, type Theme } from '$lib/stores/theme';

	const order: Theme[] = ['light', 'dark', 'system'];
	const labels: Record<Theme, string> = { light: 'Light', dark: 'Dark', system: 'System' };

	function cycleTheme() {
		const i = order.indexOf($themeStore);
		const next = order[(i + 1) % order.length];
		setTheme(next);
	}

	function iconSvg(theme: Theme): string {
		switch (theme) {
			case 'light':
				return 'M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z';
			case 'dark':
				return 'M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z';
			default:
				return 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
		}
	}
</script>

<button
	type="button"
	class="rounded-full border border-slate-200 bg-slate-100/80 p-2 transition hover:bg-slate-200/80 hover:text-pink-600 dark:border-white/[0.1] dark:bg-slate-700/80 dark:hover:bg-slate-600/80 dark:hover:text-pink-400"
	aria-label="Theme: {$themeStore}. Click to switch to {order[(order.indexOf($themeStore) + 1) % order.length]}"
	title="Theme: {labels[$themeStore]} (click to cycle)"
	onclick={cycleTheme}
>
	<svg class="h-4 w-4 text-slate-600 dark:text-slate-200" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
		<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={iconSvg($themeStore)} />
	</svg>
</button>
