/**
 * Auth API client - calls backend /api/v1/auth/* endpoints.
 * Uses same-origin /api/v1 which is proxied to backend.
 */
import { safeJson } from '$lib/api/safe-json';

export interface UserResponse {
	id: string;
	email: string;
	name: string;
	phone: string;
	ministry_role: string;
	church_name: string;
	calling: string;
	selected_course_id: string | null;
	verified: boolean;
	payment_complete: boolean;
	created_at: string;
}

/** Map API UserResponse to frontend User shape (camelCase) */
export function toUser(r: UserResponse) {
	return {
		id: r.id,
		email: r.email,
		name: r.name,
		phone: r.phone,
		ministryRole: r.ministry_role,
		churchName: r.church_name,
		calling: r.calling,
		selectedCourseId: r.selected_course_id,
		verified: r.verified,
		paymentComplete: r.payment_complete,
		createdAt: r.created_at
	};
}

export interface AuthResponse {
	user: UserResponse;
	token: string;
}

export interface RegisterRequest {
	name: string;
	email: string;
	password: string;
	phone: string;
	ministry_role: string;
	church_name: string;
	calling: string;
	selected_course_id: string | null;
}

export interface LoginRequest {
	email: string;
	password: string;
}

const API_BASE = '/api/v1';

function getAuthHeaders(): HeadersInit {
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = {
		'Content-Type': 'application/json'
	};
	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}
	return headers;
}

export async function register(data: RegisterRequest): Promise<AuthResponse> {
	const res = await fetch(`${API_BASE}/auth/register`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({
			...data,
			ministry_role: data.ministry_role,
			church_name: data.church_name
		})
	});
	const json = await safeJson<Record<string, unknown>>(res);
	if (!res.ok) {
		throw new Error((json.message as string) || (json.error as string) || 'Registration failed');
	}
	return json as AuthResponse;
}

export async function login(data: LoginRequest): Promise<AuthResponse> {
	const res = await fetch(`${API_BASE}/auth/login`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<Record<string, unknown>>(res);
	if (!res.ok) {
		throw new Error((json.message as string) || (json.error as string) || 'Login failed');
	}
	return json as AuthResponse;
}

export async function getMe(): Promise<UserResponse | null> {
	const res = await fetch(`${API_BASE}/auth/me`, {
		headers: getAuthHeaders()
	});
	if (res.status === 401) return null;
	const json = await safeJson<Record<string, unknown>>(res);
	if (!res.ok) {
		throw new Error((json.message as string) || (json.error as string) || 'Failed to fetch user');
	}
	return json as UserResponse;
}

export async function verifyAccount(): Promise<UserResponse> {
	const res = await fetch(`${API_BASE}/auth/verify`, {
		method: 'POST',
		headers: getAuthHeaders()
	});
	const json = await safeJson<Record<string, unknown>>(res);
	if (!res.ok) {
		throw new Error((json.message as string) || (json.error as string) || 'Verification failed');
	}
	return json as UserResponse;
}

export async function selectCourse(courseId: string | null): Promise<UserResponse> {
	const res = await fetch(`${API_BASE}/auth/me`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify({ selected_course_id: courseId })
	});
	const json = await safeJson<Record<string, unknown>>(res);
	if (!res.ok) {
		throw new Error((json.message as string) || (json.error as string) || 'Failed to update course');
	}
	return json as UserResponse;
}

export async function completePayment(): Promise<UserResponse> {
	const res = await fetch(`${API_BASE}/auth/complete-payment`, {
		method: 'POST',
		headers: getAuthHeaders()
	});
	const json = await safeJson<Record<string, unknown>>(res);
	if (!res.ok) {
		throw new Error((json.message as string) || (json.error as string) || 'Payment failed');
	}
	return json as UserResponse;
}

export async function getCourses(): Promise<Array<{ id: string; name: string; duration: string }>> {
	const res = await fetch(`${API_BASE}/auth/courses`, {
		headers: getAuthHeaders()
	});
	const json = await safeJson<Array<{ id: string; name: string; duration: string }> | Record<string, unknown>>(res);
	if (!res.ok) {
		const err = json as Record<string, unknown>;
		throw new Error((err.message as string) || (err.error as string) || 'Failed to fetch courses');
	}
	return Array.isArray(json) ? json : [];
}
