/**
 * SEO configuration for Prophetic Glorious Manifestation Pastoral School
 */

export const SITE = {
	baseUrl: 'https://propheticgloriousmanifestation.org',
	name: 'Prophetic Glorious Manifestation',
	tagline: 'Pastoral School',
	description:
		'Pastoral School under Prophetic Glorious Manifestation. Training pastors, ministers and prophets in Owerri, Imo State. February to July program, Monday to Friday.',
	keywords: [
		'Prophetic Glorious Manifestation',
		'Pastoral School',
		'Owerri',
		'Imo State',
		'church ministry',
		'pastors',
		'ministers',
		'prophets',
		'ministry training',
		'biblical studies',
		'prophetic ministry'
	].join(', '),
	locale: 'en_NG',
	twitterHandle: '@propheticgloriousmanifestation',
	creator: 'Prophetic Glorious Manifestation',
	ogImage: '/og-image.png'
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
