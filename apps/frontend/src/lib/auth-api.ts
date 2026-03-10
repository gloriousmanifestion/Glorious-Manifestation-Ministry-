/**
 * Auth API client - calls backend /api/v1/auth/* endpoints.
 * Uses same-origin /api/v1 which is proxied to backend.
 */
import { safeJson } from '$lib/api/safe-json';

/** User role: student (default), admin, or super_admin (created via invitation) */
export type UserRole = 'student' | 'admin' | 'super_admin';

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
	/** Omitted by backend = student. Backend must return for admin/super_admin. */
	role?: UserRole;
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
		createdAt: r.created_at,
		role: (r.role ?? 'student') as UserRole
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
	/** When present, backend creates admin/super_admin account (validates token). */
	invite_token?: string;
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

type ErrorResponse = {
	error?: unknown;
	message?: unknown;
};

function getErrorMessage(json: unknown, fallback: string): string {
	if (!json || typeof json !== 'object') return fallback;
	const data = json as ErrorResponse;
	const message = typeof data.message === 'string' ? data.message : undefined;
	const error = typeof data.error === 'string' ? data.error : undefined;
	return message ?? error ?? fallback;
}

export async function register(data: RegisterRequest): Promise<AuthResponse> {
	const d = data as RegisterRequest & { inviteToken?: string };
	const res = await fetch(`${API_BASE}/auth/register`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({
			...data,
			ministry_role: data.ministry_role,
			church_name: data.church_name,
			invite_token: data.invite_token ?? d.inviteToken
		})
	});
	const json = await safeJson<unknown>(res);
	if (!res.ok) {
		throw new Error(getErrorMessage(json, 'Registration failed'));
	}
	return json as AuthResponse;
}

export async function login(data: LoginRequest): Promise<AuthResponse> {
	const res = await fetch(`${API_BASE}/auth/login`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<unknown>(res);
	if (!res.ok) {
		throw new Error(getErrorMessage(json, 'Login failed'));
	}
	return json as AuthResponse;
}

export async function getMe(): Promise<UserResponse | null> {
	const res = await fetch(`${API_BASE}/auth/me`, {
		headers: getAuthHeaders()
	});
	if (res.status === 401) return null;
	const json = await safeJson<unknown>(res);
	if (!res.ok) {
		throw new Error(getErrorMessage(json, 'Failed to fetch user'));
	}
	return json as UserResponse;
}

export async function verifyAccount(): Promise<UserResponse> {
	const res = await fetch(`${API_BASE}/auth/verify`, {
		method: 'POST',
		headers: getAuthHeaders()
	});
	const json = await safeJson<unknown>(res);
	if (!res.ok) {
		throw new Error(getErrorMessage(json, 'Verification failed'));
	}
	return json as UserResponse;
}

export async function selectCourse(courseId: string | null): Promise<UserResponse> {
	const res = await fetch(`${API_BASE}/auth/me`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify({ selected_course_id: courseId })
	});
	const json = await safeJson<unknown>(res);
	if (!res.ok) {
		throw new Error(getErrorMessage(json, 'Failed to update course'));
	}
	return json as UserResponse;
}

export async function completePayment(): Promise<UserResponse> {
	const res = await fetch(`${API_BASE}/auth/complete-payment`, {
		method: 'POST',
		headers: getAuthHeaders()
	});
	const json = await safeJson<unknown>(res);
	if (!res.ok) {
		throw new Error(getErrorMessage(json, 'Payment failed'));
	}
	return json as UserResponse;
}

// --- Payment proofs (POP upload) ---

export type PaymentProofStatus = 'pending' | 'approved' | 'rejected';

export interface PaymentProof {
	id: string;
	file_url: string;
	status: PaymentProofStatus;
	uploaded_at: string;
	reviewed_at?: string | null;
	reviewed_by?: string | null;
	comment?: string | null;
}

export async function getMyPaymentProofs(): Promise<PaymentProof[]> {
	const res = await fetch(`${API_BASE}/payments/proofs/me`, {
		headers: getAuthHeaders()
	});
	const json = await safeJson<PaymentProof[] | Record<string, unknown>>(res);
	if (!res.ok) {
		const err = json as Record<string, unknown>;
		throw new Error(
			(err.message as string) || (err.error as string) || 'Failed to load payment proofs'
		);
	}
	return Array.isArray(json) ? (json as PaymentProof[]) : [];
}

export async function uploadPaymentProof(file: File, note?: string): Promise<PaymentProof> {
	const formData = new FormData();
	formData.append('file', file);
	if (note) formData.append('note', note);

	const token =
		typeof window !== 'undefined' ? window.localStorage.getItem('pms_token') : undefined;

	const res = await fetch(`${API_BASE}/payments/proofs`, {
		method: 'POST',
		headers: token ? { Authorization: `Bearer ${token}` } : undefined,
		body: formData
	});
	const json = await safeJson<unknown>(res);
	if (!res.ok) {
		throw new Error(getErrorMessage(json, 'Failed to upload proof of payment'));
	}
	return json as PaymentProof;
}

export async function getCourses(): Promise<Array<{ id: string; name: string; duration: string }>> {
	const res = await fetch(`${API_BASE}/auth/courses`, {
		headers: getAuthHeaders()
	});
	const json = await safeJson<
		Array<{ id: string; name: string; duration: string }> | Record<string, unknown>
	>(res);
	if (!res.ok) {
		const err = json as Record<string, unknown>;
		throw new Error((err.message as string) || (err.error as string) || 'Failed to fetch courses');
	}
	return Array.isArray(json) ? json : [];
}

// --- Invitations (admin/super_admin signup) ---

/**
 * Bootstrap token for first-time setup (Option A).
 * Backend contract:
 * - GET /auth/invite/bootstrap (or GET /auth/invite/:token when token === ADMIN_BOOTSTRAP_TOKEN from env):
 *   If no super_admin exists yet (or token matches), return { valid: true, role: 'super_admin' }.
 * - POST /auth/register with invite_token: 'bootstrap' (or env token):
 *   Validate bootstrap token, create user with role 'super_admin'.
 * Use env ADMIN_BOOTSTRAP_TOKEN for a secret URL (e.g. /invite/<secret>) instead of literal "bootstrap" if preferred.
 */
export const BOOTSTRAP_INVITE_TOKEN = 'bootstrap';

export interface InviteInfo {
	valid: boolean;
	role: UserRole;
	email?: string;
	expires_at?: string;
}

/** Validate invite token; returns role and optional email. Backend: GET /auth/invite/:token */
export async function getInvite(token: string): Promise<InviteInfo | null> {
	const res = await fetch(`${API_BASE}/auth/invite/${encodeURIComponent(token)}`, {
		headers: getAuthHeaders()
	});
	if (res.status === 404 || res.status === 410) return null;
	const json = await safeJson<InviteInfo | { message?: string }>(res);
	if (!res.ok) {
		throw new Error((json as { message?: string }).message ?? 'Invalid or expired invite');
	}
	const info = json as InviteInfo;
	return info.valid ? info : null;
}
