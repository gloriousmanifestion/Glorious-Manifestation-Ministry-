<script lang="ts">
	import { themeStore, setTheme, type Theme } from '$lib/stores/theme';

	const options: { value: Theme; label: string; icon: string }[] = [
		{ value: 'light', label: 'Light', icon: 'sun' },
		{ value: 'dark', label: 'Dark', icon: 'moon' },
		{ value: 'system', label: 'System', icon: 'monitor' }
	];

	function iconSvg(name: string) {
		switch (name) {
			case 'sun':
				return 'M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z';
			case 'moon':
				return 'M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z';
			default:
				return 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
		}
	}
</script>

<div class="flex items-center gap-0.5 rounded-full border border-slate-200 bg-slate-100/80 p-0.5 dark:border-white/[0.1] dark:bg-slate-700/80" role="group" aria-label="Theme">
	{#each options as opt}
		<button
			type="button"
			class="rounded-full p-2 transition {$themeStore === opt.value
				? 'bg-white text-pink-600 shadow dark:bg-[#252d47] dark:text-pink-400'
				: 'text-slate-500 hover:text-slate-700 dark:text-slate-100 dark:hover:text-white'}"
			aria-pressed={$themeStore === opt.value}
			aria-label="{opt.label} theme"
			onclick={() => setTheme(opt.value)}
		>
			<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={iconSvg(opt.icon)} />
			</svg>
		</button>
	{/each}
</div>
