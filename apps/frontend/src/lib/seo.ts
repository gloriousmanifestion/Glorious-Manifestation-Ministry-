/**
 * SEO configuration for Prophetic Glorious Manifestation Pastoral School
 * Canonical and Open Graph URLs reference https://www.pgmm.church
 */

export const SITE = {
	baseUrl: 'https://www.pgmm.church',
	name: 'Prophetic Glorious Manifestation',
	tagline: 'Pastoral School',
	description:
		'Pastoral School in Owerri, Imo State — train for ministry with Prophetic Glorious Manifestation Ministries. February–July, Mon–Fri. Pastors, ministers & prophets welcome.',
	keywords: [
		'pastoral school Nigeria',
		'ministry training Owerri',
		'Prophetic Glorious Manifestation Ministries',
		'PGMM church',
		'pastoral school Owerri Imo State',
		'Bible school Nigeria',
		'pastoral leadership course',
		'prophetic ministry training',
		'church ministry certification',
		'minister training Imo State',
		'pastor training program Nigeria',
		'prophetic ministry school',
		'Christian ministry education Owerri',
		'pastoral formation Nigeria',
		'church leadership training'
	].join(', '),
	locale: 'en_NG',
	twitterHandle: '@pgmmchurch',
	creator: 'Prophetic Glorious Manifestation Ministries',
	ogImage: '/og-image.png',
	ogImageAlt: 'Pastoral School — Prophetic Glorious Manifestation Ministries, Owerri, Imo State'
} as const;

export type SeoProps = {
	title?: string;
	description?: string;
	keywords?: string;
	image?: string;
	imageAlt?: string;
	url?: string;
	type?: 'website' | 'article';
	noIndex?: boolean;
	canonical?: string;
};
