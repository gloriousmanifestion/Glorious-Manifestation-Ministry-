/**
 * Client-side auth store for Pastoral School.
 * Connected to backend API. Token stored in localStorage.
 */

import { writable, get } from 'svelte/store';
import * as api from '$lib/auth-api';
import { toUser } from '$lib/auth-api';
import type { UserResponse } from '$lib/auth-api';

const TOKEN_KEY = 'pms_token';
const USER_KEY = 'pms_user';

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
}

export interface Course {
	id: string;
	name: string;
	duration: string;
}

/** Fetch courses from backend - use this instead of hardcoded list */
export async function fetchCourses(): Promise<Course[]> {
	return api.getCourses();
}

function loadUser(): User | null {
	if (typeof window === 'undefined') return null;
	try {
		const raw = localStorage.getItem(USER_KEY);
		if (!raw) return null;
		return JSON.parse(raw) as User;
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
		async register(data: Omit<User, 'id' | 'verified' | 'paymentComplete' | 'createdAt'> & { password: string }) {
			const res = await api.register({
				name: data.name,
				email: data.email,
				password: data.password,
				phone: data.phone,
				ministry_role: data.ministryRole,
				church_name: data.churchName,
				calling: data.calling,
				selected_course_id: data.selectedCourseId
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
		async selectCourse(courseId: string | null) {
			if (!courseId) return;
			const r = await api.selectCourse(courseId);
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
