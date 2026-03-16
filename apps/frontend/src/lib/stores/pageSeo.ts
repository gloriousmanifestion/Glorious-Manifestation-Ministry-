/**
 * Per-page SEO: title, description, keywords.
 * Set from each page so shared links show page-specific og:title/og:description.
 * Image is always global (not set here) so og:image stays the site default.
 */

import { writable } from 'svelte/store';

export interface PageSeo {
	title?: string;
	description?: string;
	keywords?: string;
	noIndex?: boolean;
}

export const pageSeo = writable<PageSeo>({});
