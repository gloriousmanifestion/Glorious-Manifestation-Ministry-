
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	export interface AppTypes {
		RouteId(): "/" | "/about" | "/admissions" | "/api" | "/api/v1" | "/api/v1/[...path]" | "/careers" | "/contact" | "/cookies" | "/dashboard" | "/events" | "/login" | "/privacy" | "/programs" | "/register" | "/terms";
		RouteParams(): {
			"/api/v1/[...path]": { path: string }
		};
		LayoutParams(): {
			"/": { path?: string };
			"/about": Record<string, never>;
			"/admissions": Record<string, never>;
			"/api": { path?: string };
			"/api/v1": { path?: string };
			"/api/v1/[...path]": { path: string };
			"/careers": Record<string, never>;
			"/contact": Record<string, never>;
			"/cookies": Record<string, never>;
			"/dashboard": Record<string, never>;
			"/events": Record<string, never>;
			"/login": Record<string, never>;
			"/privacy": Record<string, never>;
			"/programs": Record<string, never>;
			"/register": Record<string, never>;
			"/terms": Record<string, never>
		};
		Pathname(): "/" | "/admissions" | `/api/v1/${string}` & {} | "/careers" | "/contact" | "/cookies" | "/dashboard" | "/events" | "/login" | "/privacy" | "/programs" | "/register" | "/terms";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/.DS_Store" | "/cookies.png" | "/favicon.png" | "/favicon.svg" | "/logo.svg" | "/og-image.png" | "/oneauth-logo.svg" | "/pastoral-school-flyer.png" | "/prophetess-leader.png" | "/voice/.DS_Store" | "/voice/hold.mp3" | "/voice/welcome.mp3" | string & {};
	}
}