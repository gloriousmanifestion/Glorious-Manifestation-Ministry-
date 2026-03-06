/**
 * Events API - fetches events from backend
 */
import { safeJson } from '$lib/api/safe-json';

export interface Event {
	id: string;
	title: string;
	date: string;
	description: string | null;
}

const API_BASE = '/api/v1';

export async function getEvents(): Promise<Event[]> {
	const res = await fetch(`${API_BASE}/events`);
	const json = await safeJson<Event[] | { message?: string; error?: string }>(res);
	if (!res.ok) {
		const err = json as { message?: string; error?: string };
		throw new Error(err.message || err.error || 'Failed to fetch events');
	}
	return Array.isArray(json) ? json : [];
}
