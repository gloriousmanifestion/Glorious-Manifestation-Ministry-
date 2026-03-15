/**
 * Admin API client for dashboard, users, courses, and learning plans.
 * Calls /api/v1/admin/* — backend can implement these endpoints when ready.
 */

import { safeJson } from '$lib/api/safe-json';

const API_BASE = '/api/v1';

function getAuthHeaders(): HeadersInit {
	const token = typeof window !== 'undefined' ? localStorage.getItem('pms_token') : null;
	const headers: Record<string, string> = { 'Content-Type': 'application/json' };
	if (token) headers['Authorization'] = `Bearer ${token}`;
	return headers;
}

// --- Types ---

export interface AdminMetrics {
	totalRegistrations: number;
	verifiedCount: number;
	paidCount: number;
	pendingPaymentCount: number;
	revenueTotal: number;
	currency: string;
	registrationsThisMonth: number;
	completionRate: number;
}

export interface AdminUser {
	id: string;
	email: string;
	name: string;
	phone: string;
	ministryRole: string;
	churchName: string;
	verified: boolean;
	paymentComplete: boolean;
	selectedCourseId: string | null;
	createdAt: string;
}

export interface AdminCourse {
	id: string;
	name: string;
	duration: string;
	description?: string;
	price?: number;
	active: boolean;
	/** Microsoft Teams class/team id for "Open in Teams" on student dashboard */
	teams_class_id?: string | null;
}

export interface LearningPlan {
	id: string;
	name: string;
	description: string;
	price: number;
	currency: string;
	duration: string;
	courseIds: string[];
	active: boolean;
	createdAt: string;
}

// --- Metrics (mock until backend) ---

export async function getAdminMetrics(): Promise<AdminMetrics> {
	const res = await fetch(`${API_BASE}/admin/metrics`, { headers: getAuthHeaders() });
	if (res.status === 404 || res.status === 502) {
		return getMockMetrics();
	}
	const json = await safeJson<AdminMetrics | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to load metrics');
	return json as AdminMetrics;
}

function getMockMetrics(): AdminMetrics {
	return {
		totalRegistrations: 124,
		verifiedCount: 98,
		paidCount: 76,
		pendingPaymentCount: 48,
		revenueTotal: 2280000,
		currency: 'NGN',
		registrationsThisMonth: 18,
		completionRate: 61
	};
}

// --- Users ---

export async function getAdminUsers(params?: {
	search?: string;
	page?: number;
	limit?: number;
}): Promise<{
	users: AdminUser[];
	total: number;
}> {
	const q = new URLSearchParams();
	if (params?.search) q.set('search', params.search);
	if (params?.page) q.set('page', String(params.page));
	if (params?.limit) q.set('limit', String(params.limit ?? 20));
	const url = `${API_BASE}/admin/users${q.toString() ? '?' + q.toString() : ''}`;
	const res = await fetch(url, { headers: getAuthHeaders() });
	if (res.status === 404 || res.status === 502) {
		return { users: getMockUsers(), total: 12 };
	}
	const json = await safeJson<{ users: AdminUser[]; total: number } | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to load users');
	return json as { users: AdminUser[]; total: number };
}

/** Bulk verify selected users (admin only). */
export async function bulkVerifyUsers(userIds: string[]): Promise<{ updated: number }> {
	const res = await fetch(`${API_BASE}/admin/users/bulk-verify`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({ user_ids: userIds })
	});
	const json = await safeJson<{ updated: number } | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to verify users');
	return json as { updated: number };
}

/** Bulk mark selected users as paid (admin only). */
export async function bulkMarkPaidUsers(userIds: string[]): Promise<{ updated: number }> {
	const res = await fetch(`${API_BASE}/admin/users/bulk-mark-paid`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({ user_ids: userIds })
	});
	const json = await safeJson<{ updated: number } | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to mark users paid');
	return json as { updated: number };
}

/** Download registrations as CSV file (admin only). */
export async function downloadRegistrationsCsv(): Promise<void> {
	const url = `${API_BASE}/admin/reports/registrations?format=csv`;
	const res = await fetch(url, { headers: getAuthHeaders() });
	if (!res.ok) {
		const json = await safeJson<{ message?: string }>(res).catch(() => ({ message: undefined }));
		throw new Error(json?.message ?? 'Failed to download export');
	}
	const blob = await res.blob();
	const disposition = res.headers.get('Content-Disposition');
	const match = disposition?.match(/filename="?([^";\n]+)"?/);
	const filename = match?.[1] ?? `registrations-${new Date().toISOString().slice(0, 10)}.csv`;
	const a = document.createElement('a');
	a.href = URL.createObjectURL(blob);
	a.download = filename;
	a.click();
	URL.revokeObjectURL(a.href);
}

// --- Audit log ---

export interface AuditLogEntry {
	id: string;
	admin_user_id: string;
	action: string;
	target_type: string | null;
	target_id: string | null;
	details: string | null;
	created_at: string;
}

export async function getAuditLog(params?: {
	limit?: number;
	offset?: number;
}): Promise<AuditLogEntry[]> {
	const q = new URLSearchParams();
	if (params?.limit != null) q.set('limit', String(params.limit));
	if (params?.offset != null) q.set('offset', String(params.offset));
	const url = `${API_BASE}/admin/reports/audit-log${q.toString() ? '?' + q.toString() : ''}`;
	const res = await fetch(url, { headers: getAuthHeaders() });
	const json = await safeJson<AuditLogEntry[] | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to load audit log');
	return Array.isArray(json) ? json : [];
}

function getMockUsers(): AdminUser[] {
	return [
		{
			id: '1',
			email: 'pastor@church.org',
			name: 'John Adebayo',
			phone: '+234 800 000 0001',
			ministryRole: 'Pastor',
			churchName: 'Grace Assembly',
			verified: true,
			paymentComplete: true,
			selectedCourseId: 'pastoral-leadership',
			createdAt: '2025-02-01T10:00:00Z'
		},
		{
			id: '2',
			email: 'minister@faith.org',
			name: 'Chioma Okeke',
			phone: '+234 800 000 0002',
			ministryRole: 'Minister',
			churchName: 'Faith Chapel',
			verified: true,
			paymentComplete: false,
			selectedCourseId: 'prophetic-ministry',
			createdAt: '2025-02-15T14:30:00Z'
		}
	];
}

// --- Courses (backend may expose CRUD) ---

export async function getAdminCourses(): Promise<AdminCourse[]> {
	const res = await fetch(`${API_BASE}/admin/courses`, { headers: getAuthHeaders() });
	if (res.status === 404 || res.status === 502) {
		return getMockAdminCourses();
	}
	const json = await safeJson<AdminCourse[] | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to load courses');
	return Array.isArray(json) ? json : [];
}

function getMockAdminCourses(): AdminCourse[] {
	return [
		{ id: 'pastoral-leadership', name: 'Pastoral Leadership', duration: '8 weeks', active: true },
		{ id: 'prophetic-ministry', name: 'Prophetic Ministry', duration: '8 weeks', active: true },
		{ id: 'ministerial-training', name: 'Ministerial Training', duration: '8 weeks', active: true },
		{ id: 'biblical-studies', name: 'Biblical Studies', duration: '8 weeks', active: true },
		{ id: 'worship-praise', name: 'Worship & Praise', duration: '8 weeks', active: true },
		{ id: 'evangelism-outreach', name: 'Evangelism & Outreach', duration: '8 weeks', active: true }
	];
}

export async function createCourse(data: {
	name: string;
	duration: string;
	description?: string;
	price?: number;
	teams_class_id?: string | null;
}): Promise<AdminCourse> {
	const res = await fetch(`${API_BASE}/admin/courses`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<AdminCourse | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to create course');
	return json as AdminCourse;
}

export interface AdminCourseModule {
	id: string;
	course_id: string;
	title: string;
	sort_order: number;
	link: string | null;
	description: string | null;
	created_at: string;
}

export async function getCourseModules(courseId: string): Promise<AdminCourseModule[]> {
	const res = await fetch(`${API_BASE}/admin/courses/${encodeURIComponent(courseId)}/modules`, {
		headers: getAuthHeaders()
	});
	const json = await safeJson<AdminCourseModule[] | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to load modules');
	return Array.isArray(json) ? json : [];
}

export async function createCourseModule(
	courseId: string,
	data: { title: string; sort_order?: number; link?: string | null; description?: string | null }
): Promise<AdminCourseModule> {
	const res = await fetch(`${API_BASE}/admin/courses/${encodeURIComponent(courseId)}/modules`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<AdminCourseModule | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to create module');
	return json as AdminCourseModule;
}

export async function updateCourseModule(
	courseId: string,
	moduleId: string,
	data: Partial<{
		title: string;
		sort_order: number;
		link: string | null;
		description: string | null;
	}>
): Promise<AdminCourseModule> {
	const res = await fetch(
		`${API_BASE}/admin/courses/${encodeURIComponent(courseId)}/modules/${encodeURIComponent(moduleId)}`,
		{
			method: 'PATCH',
			headers: getAuthHeaders(),
			body: JSON.stringify(data)
		}
	);
	const json = await safeJson<AdminCourseModule | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to update module');
	return json as AdminCourseModule;
}

export async function deleteCourseModule(courseId: string, moduleId: string): Promise<void> {
	const res = await fetch(
		`${API_BASE}/admin/courses/${encodeURIComponent(courseId)}/modules/${encodeURIComponent(moduleId)}`,
		{ method: 'DELETE', headers: getAuthHeaders() }
	);
	if (res.status === 204) return;
	const json = await safeJson<{ message?: string }>(res).catch(() => ({ message: undefined }));
	throw new Error(json?.message ?? 'Failed to delete module');
}

// --- Resources (library) ---

export interface AdminResource {
	id: string;
	course_id: string | null;
	title: string;
	url: string;
	resource_type: string;
	sort_order: number;
	created_at: string;
}

export async function getAdminResources(courseId?: string | null): Promise<AdminResource[]> {
	const url = courseId
		? `${API_BASE}/admin/resources?course_id=${encodeURIComponent(courseId)}`
		: `${API_BASE}/admin/resources`;
	const res = await fetch(url, { headers: getAuthHeaders() });
	const json = await safeJson<AdminResource[] | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to load resources');
	return Array.isArray(json) ? json : [];
}

export async function createResource(data: {
	course_id?: string | null;
	title: string;
	url: string;
	resource_type?: string;
	sort_order?: number;
}): Promise<AdminResource> {
	const res = await fetch(`${API_BASE}/admin/resources`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<AdminResource | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to create resource');
	return json as AdminResource;
}

export async function updateResource(
	id: string,
	data: Partial<{
		course_id: string | null;
		title: string;
		url: string;
		resource_type: string;
		sort_order: number;
	}>
): Promise<AdminResource> {
	const res = await fetch(`${API_BASE}/admin/resources/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<AdminResource | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to update resource');
	return json as AdminResource;
}

export async function deleteResource(id: string): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/resources/${encodeURIComponent(id)}`, {
		method: 'DELETE',
		headers: getAuthHeaders()
	});
	if (res.status === 204) return;
	const json = await safeJson<{ message?: string }>(res).catch(() => ({ message: undefined }));
	throw new Error(json?.message ?? 'Failed to delete resource');
}

export async function updateCourse(
	id: string,
	data: Partial<{
		name: string;
		duration: string;
		description: string;
		price: number;
		active: boolean;
		teams_class_id: string | null;
	}>
): Promise<AdminCourse> {
	const res = await fetch(`${API_BASE}/admin/courses/${id}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<AdminCourse | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to update course');
	return json as AdminCourse;
}

// --- Learning plans ---

export async function getLearningPlans(): Promise<LearningPlan[]> {
	const res = await fetch(`${API_BASE}/admin/learning-plans`, { headers: getAuthHeaders() });
	if (res.status === 404 || res.status === 502) {
		return getMockLearningPlans();
	}
	const json = await safeJson<LearningPlan[] | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to load learning plans');
	return Array.isArray(json) ? json : [];
}

function getMockLearningPlans(): LearningPlan[] {
	return [
		{
			id: 'full-program-2025',
			name: 'Full Program 2025',
			description: 'Full pastoral school program, February–July.',
			price: 30000,
			currency: 'NGN',
			duration: 'February to July',
			courseIds: [
				'pastoral-leadership',
				'prophetic-ministry',
				'ministerial-training',
				'biblical-studies',
				'worship-praise',
				'evangelism-outreach'
			],
			active: true,
			createdAt: '2025-01-01T00:00:00Z'
		}
	];
}

export async function createLearningPlan(data: {
	name: string;
	description: string;
	price: number;
	currency: string;
	duration: string;
	courseIds: string[];
}): Promise<LearningPlan> {
	const res = await fetch(`${API_BASE}/admin/learning-plans`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<LearningPlan | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to create learning plan');
	return json as LearningPlan;
}

export async function updateLearningPlan(
	id: string,
	data: Partial<{
		name: string;
		description: string;
		price: number;
		duration: string;
		courseIds: string[];
		active: boolean;
	}>
): Promise<LearningPlan> {
	const res = await fetch(`${API_BASE}/admin/learning-plans/${id}`, {
		method: 'PATCH',
		headers: getAuthHeaders(),
		body: JSON.stringify(data)
	});
	const json = await safeJson<LearningPlan | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to update learning plan');
	return json as LearningPlan;
}

// --- Payment proofs (POP review) ---

export type AdminPaymentProofStatus = 'pending' | 'approved' | 'rejected';

export interface AdminPaymentProof {
	id: string;
	userId: string;
	userName: string;
	userEmail: string;
	userPhone?: string | null;
	uploadedAt: string;
	status: AdminPaymentProofStatus;
	fileUrl: string;
	note?: string | null;
}

export async function getPaymentProofs(params?: {
	status?: AdminPaymentProofStatus;
	limit?: number;
}): Promise<AdminPaymentProof[]> {
	const q = new URLSearchParams();
	if (params?.status) q.set('status', params.status);
	if (params?.limit) q.set('limit', String(params.limit ?? 20));
	const url = `${API_BASE}/admin/payment-proofs${q.toString() ? '?' + q.toString() : ''}`;
	const res = await fetch(url, { headers: getAuthHeaders() });
	if (res.status === 404 || res.status === 502) {
		return getMockPaymentProofs();
	}
	const json = await safeJson<AdminPaymentProof[] | { message?: string }>(res);
	if (!res.ok)
		throw new Error((json as { message?: string }).message ?? 'Failed to load payment proofs');
	return Array.isArray(json) ? (json as AdminPaymentProof[]) : [];
}

export async function reviewPaymentProof(
	id: string,
	status: AdminPaymentProofStatus,
	comment?: string
): Promise<void> {
	const res = await fetch(`${API_BASE}/admin/payment-proofs/${encodeURIComponent(id)}/review`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({ status, comment })
	});
	const json = await safeJson<{ message?: string } | Record<string, unknown>>(res);
	if (!res.ok) {
		throw new Error((json as { message?: string }).message ?? 'Failed to update proof status');
	}
}

function getMockPaymentProofs(): AdminPaymentProof[] {
	return [
		{
			id: 'pop_1',
			userId: '1',
			userName: 'John Adebayo',
			userEmail: 'pastor@church.org',
			userPhone: '+234 800 000 0001',
			uploadedAt: new Date().toISOString(),
			status: 'pending',
			fileUrl: '#',
			note: 'GTBank transfer on 3rd March, 45,000 NGN'
		},
		{
			id: 'pop_2',
			userId: '2',
			userName: 'Chioma Okeke',
			userEmail: 'minister@faith.org',
			userPhone: '+234 800 000 0002',
			uploadedAt: new Date(Date.now() - 3600_000).toISOString(),
			status: 'pending',
			fileUrl: '#',
			note: 'UBA transfer, full payment'
		}
	];
}

// --- Invitations (super_admin only) ---

export type InviteRole = 'admin' | 'super_admin';

export interface InviteResult {
	token: string;
	link: string;
	expires_at: string;
	role: InviteRole;
}

/** Create an invite link. Backend: POST /admin/invites. Only super_admin can call. */
export async function createInvite(role: InviteRole): Promise<InviteResult> {
	const res = await fetch(`${API_BASE}/admin/invites`, {
		method: 'POST',
		headers: getAuthHeaders(),
		body: JSON.stringify({ role })
	});
	const json = await safeJson<InviteResult | { message?: string }>(res);
	if (!res.ok) throw new Error((json as { message?: string }).message ?? 'Failed to create invite');
	const result = json as InviteResult;
	if (!result.link && result.token && typeof window !== 'undefined') {
		result.link = `${window.location.origin}/invite/${result.token}`;
	}
	return result;
}
