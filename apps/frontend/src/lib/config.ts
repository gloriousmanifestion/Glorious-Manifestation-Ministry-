/**
 * Church ministry school configuration
 */

export const SITE = {
	name: 'Prophetic Glorious Manifestation',
	/** Short name for nav/branding (e.g. "PGM | Pastoral School") */
	shortName: 'PGM',
	/** Full legal name — used only in footer copyright */
	legalName: 'Prophetic Glorious Manifestation Ministries',
	schoolName: 'Pastoral School',
	location: 'Owerri, Imo State',
	address: 'Ezeigodo Avenue, Rochas Backgate Spilbait, Owerri, Imo State',
	whatsapp: '08026954759',
	whatsappFull: '+2348026954759',
	contactEmail: 'hello@pgmm.church',
	infoEmail: 'hello@pgmm.church',
	helloEmail: 'hello@pgmm.church',
	schedule: 'Monday to Friday',
	duration: 'February to July',
	targetAudience: 'Pastors, ministers and prophets',
	/** Selar checkout URL for PGM Pastoral School payments */
	selarPaymentUrl: 'https://selar.com/577p7vtjkr',
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
		address: 'Ezeigodo Avenue, Rochas Backgate Spilbait, Owerri, Imo State',
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
