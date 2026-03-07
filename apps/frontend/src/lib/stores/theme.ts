/**
 * Theme store: 'light' | 'dark' | 'system'.
 * Persists to localStorage and applies .dark to document.documentElement when effective theme is dark.
 */
import { writable } from 'svelte/store';

const STORAGE_KEY = 'pms_theme';

export type Theme = 'light' | 'dark' | 'system';

function getStored(): Theme {
	if (typeof window === 'undefined') return 'light';
	try {
		const v = localStorage.getItem(STORAGE_KEY);
		if (v === 'light' || v === 'dark' || v === 'system') return v;
	} catch {
		/* ignore */
	}
	return 'light';
}

function isDarkEffective(theme: Theme): boolean {
	if (theme === 'dark') return true;
	if (theme === 'light') return false;
	if (typeof window === 'undefined') return false;
	return window.matchMedia('(prefers-color-scheme: dark)').matches;
}

function apply(theme: Theme) {
	const dark = isDarkEffective(theme);
	document.documentElement.classList.toggle('dark', dark);
	document.documentElement.setAttribute('data-theme', theme);
	const meta = document.querySelector('meta[name="theme-color"]');
	if (meta) {
		meta.setAttribute('content', dark ? '#374266' : '#FFFCFA');
	}
}

export const themeStore = writable<Theme>(getStored());

export function getTheme(): Theme {
	let v: Theme;
	themeStore.subscribe((x) => (v = x))();
	return v!;
}

export function setTheme(value: Theme) {
	themeStore.set(value);
	try {
		localStorage.setItem(STORAGE_KEY, value);
	} catch {
		/* ignore */
	}
	apply(value);
}

export function initTheme() {
	apply(getStored());
	let current: Theme = getStored();
	themeStore.subscribe((v) => {
		current = v;
		apply(v);
	});
	if (typeof window !== 'undefined') {
		window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
			apply(current);
		});
	}
}
