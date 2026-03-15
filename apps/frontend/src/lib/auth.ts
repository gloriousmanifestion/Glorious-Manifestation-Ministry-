/**
 * Client-side auth store for Pastoral School.
 * Connected to backend API. Token stored in localStorage.
 */

import { writable, get } from 'svelte/store';
import * as api from '$lib/auth-api';
import { toUser, type PaymentProof } from '$lib/auth-api';
import type { UserRole } from '$lib/auth-api';

const TOKEN_KEY = 'pms_token';
const USER_KEY = 'pms_user';

export type { UserRole };

export interface User {
	id: string;
	email: string;
	name: string;
	phone: string;
	ministryRole: string;
	churchName: string;
	calling: string;
	selectedCourseId: string | null;
	verified: boolean;
	paymentComplete: boolean;
	createdAt: string;
	/** student (default) | admin | super_admin. Only admin/super_admin can access /admin. */
	role: UserRole;
}

export interface Course {
	id: string;
	name: string;
	duration: string;
	/** Microsoft Teams class/team id; when set, dashboard shows "Open in Teams". */
	teams_class_id?: string | null;
}

export type { PaymentProof };

/** Fetch courses from backend - use this instead of hardcoded list */
export async function fetchCourses(): Promise<Course[]> {
	return api.getCourses();
}

function loadUser(): User | null {
	if (typeof window === 'undefined') return null;
	try {
		const raw = localStorage.getItem(USER_KEY);
		if (!raw) return null;
		const u = JSON.parse(raw) as User;
		if (!u.role) u.role = 'student';
		return u;
	} catch {
		return null;
	}
}

function saveUser(user: User | null): void {
	if (typeof window === 'undefined') return;
	if (user) {
		localStorage.setItem(USER_KEY, JSON.stringify(user));
	} else {
		localStorage.removeItem(USER_KEY);
		localStorage.removeItem(TOKEN_KEY);
	}
}

const userStore = writable<User | null>(loadUser());
export const authReady = writable(false);

/** Hydrate user from API using stored token (call on app init / mount) */
export async function hydrateUser(): Promise<void> {
	if (typeof window === 'undefined') return;
	authReady.set(false);
	const token = localStorage.getItem(TOKEN_KEY);
	if (!token) {
		userStore.set(null);
		authReady.set(true);
		return;
	}
	try {
		const r = await api.getMe();
		if (r) {
			const user = toUser(r);
			userStore.set(user);
			saveUser(user);
		} else {
			userStore.set(null);
			saveUser(null);
		}
	} catch {
		userStore.set(null);
		saveUser(null);
	} finally {
		authReady.set(true);
	}
}

function createAuthStore() {
	return {
		userStore,
		get user() {
			return get(userStore);
		},
		get isLoggedIn() {
			return get(userStore) !== null;
		},
		get hasPendingVerify() {
			const u = get(userStore);
			return u !== null && !u.verified;
		},
		get hasPendingPayment() {
			const u = get(userStore);
			return u !== null && !u.paymentComplete;
		},
		/** True if user can access /admin (admin or super_admin). */
		get canAccessAdmin() {
			const u = get(userStore);
			return u !== null && (u.role === 'admin' || u.role === 'super_admin');
		},
		get isSuperAdmin() {
			const u = get(userStore);
			return u !== null && u.role === 'super_admin';
		},
		async register(
			data: Omit<User, 'id' | 'verified' | 'paymentComplete' | 'createdAt' | 'role'> & {
				password: string;
				inviteToken?: string;
			}
		) {
			const res = await api.register({
				name: data.name,
				email: data.email,
				password: data.password,
				phone: data.phone,
				ministry_role: data.ministryRole,
				church_name: data.churchName,
				calling: data.calling,
				selected_course_id: data.selectedCourseId,
				invite_token: data.inviteToken
			});
			localStorage.setItem(TOKEN_KEY, res.token);
			const user = toUser(res.user);
			userStore.set(user);
			saveUser(user);
			return user;
		},
		async login(email: string, password: string) {
			const res = await api.login({ email, password });
			localStorage.setItem(TOKEN_KEY, res.token);
			const user = toUser(res.user);
			userStore.set(user);
			saveUser(user);
			return user;
		},
		async requestPasswordReset(email: string) {
			await api.requestPasswordReset(email);
		},
		async resetPassword(token: string, password: string) {
			await api.resetPassword(token, password);
		},
		async verifyAccount() {
			const r = await api.verifyAccount();
			const user = toUser(r);
			userStore.set(user);
			saveUser(user);
		},
		async completePayment() {
			const r = await api.completePayment();
			const user = toUser(r);
			userStore.set(user);
			saveUser(user);
		},
		async uploadPaymentProof(file: File, note?: string) {
			const proof = await api.uploadPaymentProof(file, note);
			return proof;
		},
		async getMyPaymentProofs() {
			return api.getMyPaymentProofs();
		},
		async selectCourse(courseId: string | null) {
			if (!courseId) return;
			const r = await api.selectCourse(courseId);
			const user = toUser(r);
			userStore.set(user);
			saveUser(user);
		},
		async updateProfile(profile: {
			name?: string;
			phone?: string;
			ministryRole?: string;
			churchName?: string;
			calling?: string;
		}) {
			const r = await api.updateProfile({
				name: profile.name,
				phone: profile.phone,
				ministry_role: profile.ministryRole,
				church_name: profile.churchName,
				calling: profile.calling
			});
			const user = toUser(r);
			userStore.set(user);
			saveUser(user);
		},
		logout() {
			userStore.set(null);
			saveUser(null);
		}
	};
}

export const auth = createAuthStore();
export { userStore };
