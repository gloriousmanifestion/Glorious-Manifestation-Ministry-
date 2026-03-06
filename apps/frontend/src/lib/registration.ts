/**
 * Global and country-specific registration numbers for footer display.
 * Shown based on user's detected country (geo lookup).
 */

/** DUNS number (global) */
export const DUNS = '366868595';

/** Country-specific registration and VAT numbers (ISO 3166-1 alpha-2) */
export const REGISTRATION: Record<string, { registration?: string; vat?: string }> = {
	ZA: {
		registration: '2026/166205/07',
		vat: '9899362199'
	}
	// Add more countries as needed, e.g. MW: { registration: '...', vat: '...' }
};
