/**
 * Church ministry school configuration
 */

// Use different Selar checkout URLs per environment.
// PUBLIC_APP_ENV is already used elsewhere (e.g. env banner).
const envName =
	(typeof import.meta.env !== 'undefined' && (import.meta.env as any).PUBLIC_APP_ENV) || 'production';
const selarPaymentUrl =
	envName === 'production' ? 'https://selar.com/k7t41ibp26' : 'https://selar.com/577p7vtjkr';

export const SITE = {
	name: 'Prophetic Glorious Manifestation',
	/** Short name for nav/branding (e.g. "PGM | Pastoral School") */
	shortName: 'PGM',
	/** Full legal name — used only in footer copyright */
	legalName: 'Prophetic Glorious Manifestation Ministries',
	schoolName: 'Pastoral School',
	location: 'Owerri, Imo State',
	address: 'Ezeigodo Avenue, Rochas Backgate Spilbait',
	whatsapp: '08026954759',
	/** E.164 / international format for wa.me links (no + in URL) */
	whatsappFull: '+2348026954759',
	/** International display with country code for the UI */
	whatsappDisplay: '+234 802 695 4759',
	contactEmail: 'hello@pgmm.church',
	infoEmail: 'hello@pgmm.church',
	helloEmail: 'hello@pgmm.church',
	schedule: 'Monday to Friday',
	duration: 'February to July',
	targetAudience: 'Pastors, ministers and prophets',
	/** Selar checkout URL for PGM Pastoral School payments */
	selarPaymentUrl,
	/** Leader and teacher of the Pastoral School */
	leader: {
		name: 'Our Prophetess',
		title: 'Leader & Teacher',
		bio: 'Leading the Pastoral School with wisdom, grace, and a heart for equipping ministers. Under her guidance, pastors, ministers, and prophets are nurtured in sound doctrine and practical ministry.'
	},
	/** Church details and services (parent ministry) */
	church: {
		name: 'Prophetic Glorious Manifestation Ministries',
		shortName: 'PGM Ministries',
		description:
			'We are a ministry dedicated to raising pastors, ministers, and prophets through the Pastoral School and weekly worship, prayer, and teaching. Our church family gathers to grow in faith and serve our community.',
		address: 'Ezeigodo Avenue, Rochas Backgate Spilbait',
		services: [
			{
				name: 'Sunday Worship',
				description: 'Main worship service with praise, worship, and the Word.',
				day: 'Sunday',
				time: '10:00 AM'
			},
			{
				name: 'Midweek Service',
				description: 'Bible study and fellowship midweek.',
				day: 'Wednesday',
				time: '5:00 PM'
			},
			{
				name: 'Prayer Meeting',
				description: 'Corporate prayer and intercession.',
				day: 'Friday',
				time: '6:00 PM'
			}
		]
	}
} as const;

export interface DefaultFaqItem {
	q: string;
	a: string;
}

/** Default FAQ entries shown on the public FAQ page when admin has not set any. Admin can override via Dashboard → FAQ. */
export function getDefaultFaqs(): DefaultFaqItem[] {
	return [
		{
			q: 'Who is this program for?',
			a: 'Our Pastoral School is designed for pastors, ministers, and prophets who desire to grow in their calling. Whether you are already in ministry or sensing a call, this program will equip you with biblical foundations and practical skills.'
		},
		{
			q: 'What are the admission requirements?',
			a: 'We look for a genuine call to ministry, commitment to attend classes, and basic literacy. A recommendation from a church or ministry leader is optional but preferred. See our Admissions page for the full process.'
		},
		{
			q: 'How do I register?',
			a: 'Create an account on our site, verify your email, then select your course and complete payment. You can also contact us via WhatsApp for assistance.'
		},
		{
			q: 'Where are classes held?',
			a: `Classes are held at our facility in ${SITE.location}. Our address is ${SITE.address}.`
		},
		{
			q: 'What is the program schedule?',
			a: `The program runs ${SITE.duration}, ${SITE.schedule}. Check the Programs page for course details.`
		},
		{
			q: 'How can I contact the school?',
			a: `You can reach us by WhatsApp (${SITE.whatsappDisplay}) or email at ${SITE.contactEmail}. We’re happy to answer your questions.`
		}
	];
}
