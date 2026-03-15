/**
 * Events API - public list + admin CRUD
 */
import { safeJson } from '$lib/api/safe-json';

export interface Event {
	id: string;
	title: string;
	date: string;
	description: string | null;
	/** Present when listing events (public list). */
	rsvp_count?: number;
}

const API_BASE = '/api/v1';

function getAuthHeaders(): HeadersInit {
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = { 'Content-Type': 'application/json' };
	if (token) headers['Authorization'] = `Bearer ${token}`;
	return headers;
}

export async function getEvents(): Promise<Event[]> {
	const res = await fetch(`${API_BASE}/events`);
	const json = await safeJson<Event[] | { message?: string; error?: string }>(res);
	if (!res.ok) {
		const err = json as { message?: string; error?: string };
		throw new Error(err.message || err.error || 'Failed to fetch events');
	}
	return Array.isArray(json) ? json : [];
}

export async function listEventsAdmin(): Promise<Event[]> {
	const res = await fetch(`${API_BASE}/admin/events`, { headers: getAuthHeaders() });
	const json = await safeJson<Event[] | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to list events');
	return Array.isArray(json) ? json : [];
}

export async function createEvent(body: {
	title: string;
	date: string;
	description?: string | null;
}): Promise<Event> {
	const res = await fetch(`${API_BASE}/admin/events`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<Event | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to create event');
	return json as Event;
}

export async function updateEvent(
	id: string,
	body: { title?: string; date?: string; description?: string | null }
): Promise<Event> {
	const res = await fetch(`${API_BASE}/admin/events/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(body)
	});
	const json = await safeJson<Event | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to update event');
	return json as Event;
}

export async function deleteEvent(id: string): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/events/${encodeURIComponent(id)}`, {
		method: 'DELETE',
		headers: getAuthHeaders()
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json.message ?? 'Failed to delete event');
	}
}

// --- RSVP (authenticated students) ---

/** Event IDs the current user has RSVP'd to. */
export async function getMyEventRsvps(): Promise<string[]> {
	const res = await fetch(`${API_BASE}/me/event-rsvps`, { headers: getAuthHeaders() });
	const json = await safeJson<string[] | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to load RSVPs');
	return Array.isArray(json) ? json : [];
}

export async function rsvpEvent(eventId: string): Promise<void> {
	const res = await fetch(`${API_BASE}/events/${encodeURIComponent(eventId)}/rsvp`, {
		method: 'POST',
		headers: getAuthHeaders()
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json?.message ?? 'Could not RSVP');
	}
}

export async function cancelRsvp(eventId: string): Promise<void> {
	const res = await fetch(`${API_BASE}/events/${encodeURIComponent(eventId)}/rsvp`, {
		method: 'DELETE',
		headers: getAuthHeaders()
	});
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res);
		throw new Error(json?.message ?? 'Could not cancel RSVP');
	}
}
