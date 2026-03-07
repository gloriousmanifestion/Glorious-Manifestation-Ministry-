<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { auth } from '$lib/auth';
	import { SITE } from '$lib/config';
	import { getCourseDetailById } from '$lib/courses';

	const steps = [
		{ id: 1, title: 'Personal info', desc: 'Your contact details' },
		{ id: 2, title: 'Ministry info', desc: 'Your calling & church' },
		{ id: 3, title: 'Review', desc: 'Confirm & submit' }
	];

	let currentStep = $state(1);
	let formData = $state({
		name: '',
		email: '',
		phone: '',
		password: '',
		confirmPassword: '',
		ministryRole: '',
		churchName: '',
		calling: ''
	});
	let errors = $state<Record<string, string>>({});
	let submitting = $state(false);
	let submitError = $state('');

	const inviteTokenFromUrl = $derived($page.url.searchParams.get('invite'));
	const isInviteSignup = $derived(!!inviteTokenFromUrl);

	function validateStep(step: number): boolean {
		errors = {};
		submitError = '';
		if (step === 1) {
			if (!formData.name.trim()) errors.name = 'Name is required';
			if (!formData.email.trim()) errors.email = 'Email is required';
			else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) errors.email = 'Valid email required';
			if (!formData.phone.trim()) errors.phone = 'Phone is required';
			if (!formData.password) errors.password = 'Password is required';
			else if (formData.password.length < 8) errors.password = 'Password must be at least 8 characters';
			if (formData.password !== formData.confirmPassword) errors.confirmPassword = 'Passwords do not match';
		}
		if (step === 2) {
			if (!formData.calling.trim()) errors.calling = 'Ministry calling is required';
			if (!formData.ministryRole.trim()) errors.ministryRole = 'Role is required';
			if (!formData.churchName.trim()) errors.churchName = 'Church name is required';
		}
		return Object.keys(errors).length === 0;
	}

	function next() {
		if (validateStep(currentStep)) {
			if (currentStep < 3) currentStep++;
			else submit();
		}
	}

	function prev() {
		if (currentStep > 1) currentStep--;
	}

	const courseIdFromUrl = $derived($page.url.searchParams.get('course'));
	const preselectedCourse = $derived(courseIdFromUrl ? getCourseDetailById(courseIdFromUrl) : null);

	async function submit() {
		if (submitting) return;
		submitting = true;
		submitError = '';
		try {
			await auth.register({
				name: formData.name,
				email: formData.email,
				phone: formData.phone,
				password: formData.password,
				ministryRole: formData.ministryRole,
				churchName: formData.churchName,
				calling: formData.calling,
				selectedCourseId: preselectedCourse ? preselectedCourse.id : null,
				inviteToken: inviteTokenFromUrl ?? undefined
			});
			goto(isInviteSignup ? '/admin' : '/dashboard');
		} catch (e) {
			submitError = e instanceof Error ? e.message : 'Registration failed. Please try again.';
		} finally {
			submitting = false;
		}
	}

</script>

<svelte:head>
	<title>Register — {SITE.name} Pastoral School</title>
	<meta name="description" content="Register for the Pastoral School at Prophetic Glorious Manifestation." />
</svelte:head>

<div class="min-h-screen flex flex-col bg-slate-50 dark:bg-[#252d47]">
	<Nav />

	<main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16">
		<div class="mx-auto max-w-5xl px-4 sm:px-6">
			<div class="mb-10 text-center">
				<h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white sm:text-4xl">
					{isInviteSignup ? 'Create your admin account' : 'Create your account'}
				</h1>
				<p class="mt-2 text-slate-600 dark:text-slate-100">
					{isInviteSignup
						? 'You were invited to join the platform as an admin. Complete the form below.'
						: 'Join our Pastoral School. Complete the form below to get started.'}
				</p>
				{#if isInviteSignup}
					<div class="mt-4 inline-flex items-center gap-2 rounded-xl border border-pink-200 bg-pink-50/80 px-4 py-3 text-sm dark:border-pink-800 dark:bg-pink-900/30">
						<span class="font-medium text-pink-700 dark:text-pink-300">Admin / Super Admin signup</span>
					</div>
				{:else if preselectedCourse}
					<div class="mt-4 inline-flex items-center gap-2 rounded-xl border border-pink-200 bg-pink-50/80 px-4 py-3 text-sm dark:border-pink-800 dark:bg-pink-900/30">
						<span class="font-medium text-pink-700 dark:text-pink-300">Registering for:</span>
						<span class="font-semibold text-slate-900 dark:text-white">{preselectedCourse.name}</span>
					</div>
				{/if}
			</div>

			<div class="flex flex-col overflow-hidden rounded-2xl border border-pink-200/50 card-premium bg-white shadow-xl dark:bg-[#1e2642] lg:flex-row">
				<!-- Stepper (left) -->
				<aside class="border-b border-pink-200/50 section-separator bg-gradient-to-b from-pink-50/80 to-purple-50/50 px-4 py-6 sm:px-8 sm:py-10 dark:from-[#252d47] dark:to-[#1e2642] lg:w-72 lg:border-b-0 lg:border-r">
					<nav class="flex flex-col gap-1" aria-label="Registration steps">
						{#each steps as step}
							<button
								type="button"
								class="flex items-center gap-4 rounded-xl px-4 py-3 text-left transition-all duration-200 {currentStep === step.id
									? 'bg-white shadow-md ring-1 ring-pink-200/60 dark:bg-white/10 dark:ring-slate-500'
									: currentStep > step.id
										? 'text-pink-600 dark:text-pink-400'
										: 'text-slate-500 hover:bg-white/60 dark:text-slate-300 dark:hover:bg-white/10'}"
								onclick={() => currentStep > step.id && (currentStep = step.id)}
							>
								<span
									class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold {currentStep === step.id
										? 'bg-gradient-to-br from-pink-500 to-purple-600 text-white'
										: currentStep > step.id
											? 'bg-pink-100 text-pink-600 dark:bg-pink-900/50 dark:text-pink-300'
											: 'bg-slate-200 text-slate-500 dark:bg-slate-600 dark:text-slate-400'}"
								>
									{#if currentStep > step.id}
										<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
											<path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
										</svg>
									{:else}
										{step.id}
									{/if}
								</span>
								<div>
									<p class="font-medium text-slate-900 dark:text-white">{step.title}</p>
									<p class="text-xs text-slate-500 dark:text-slate-200">{step.desc}</p>
								</div>
							</button>
						{/each}
					</nav>
				</aside>

				<!-- Form content (right) -->
				<div class="flex-1 p-4 sm:p-6 lg:p-12">
					{#if currentStep === 1}
						<div class="space-y-6">
							<h2 class="text-xl font-semibold text-slate-900 dark:text-white">Personal information</h2>
							<div>
								<label for="name" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Full name</label>
								<input
									id="name"
									type="text"
									bind:value={formData.name}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400 {errors.name ? 'border-red-300 dark:border-red-600' : ''}"
									placeholder="John Doe"
								/>
								{#if errors.name}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.name}</p>
								{/if}
							</div>
							<div>
								<label for="email" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Email</label>
								<input
									id="email"
									type="email"
									bind:value={formData.email}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400 {errors.email ? 'border-red-300 dark:border-red-600' : ''}"
									placeholder="John@example.com"
								/>
								{#if errors.email}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.email}</p>
								{/if}
							</div>
							<div>
								<label for="phone" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Phone number</label>
								<input
									id="phone"
									type="tel"
									bind:value={formData.phone}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400 {errors.phone ? 'border-red-300 dark:border-red-600' : ''}"
									placeholder="080 1234 5678"
								/>
								{#if errors.phone}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.phone}</p>
								{/if}
							</div>
							<div>
								<label for="password" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Password</label>
								<input
									id="password"
									type="password"
									bind:value={formData.password}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400 {errors.password ? 'border-red-300 dark:border-red-600' : ''}"
									placeholder="At least 8 characters"
								/>
								{#if errors.password}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.password}</p>
								{/if}
							</div>
							<div>
								<label for="confirmPassword" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Confirm password</label>
								<input
									id="confirmPassword"
									type="password"
									bind:value={formData.confirmPassword}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400 {errors.confirmPassword ? 'border-red-300 dark:border-red-600' : ''}"
									placeholder="Re-enter your password"
								/>
								{#if errors.confirmPassword}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.confirmPassword}</p>
								{/if}
							</div>
						</div>
					{:else if currentStep === 2}
						<div class="space-y-6">
							<h2 class="text-xl font-semibold text-slate-900 dark:text-white">Ministry information</h2>
							<div>
								<label for="calling" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Ministry calling</label>
								<select
									id="calling"
									bind:value={formData.calling}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white {errors.calling ? 'border-red-300 dark:border-red-600' : ''}"
								>
									<option value="">Select your calling</option>
									<option value="pastor">Pastor</option>
									<option value="minister">Minister</option>
									<option value="prophet">Prophet</option>
									<option value="evangelist">Evangelist</option>
									<option value="teacher">Teacher</option>
									<option value="other">Other</option>
								</select>
								{#if errors.calling}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.calling}</p>
								{/if}
							</div>
							<div>
								<label for="ministryRole" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Current ministry role</label>
								<input
									id="ministryRole"
									type="text"
									bind:value={formData.ministryRole}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400 {errors.ministryRole ? 'border-red-300 dark:border-red-600' : ''}"
									placeholder="e.g. Associate Pastor, Youth Minister"
								/>
								{#if errors.ministryRole}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.ministryRole}</p>
								{/if}
							</div>
							<div>
								<label for="churchName" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Church / ministry name</label>
								<input
									id="churchName"
									type="text"
									bind:value={formData.churchName}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#252d47] dark:text-white dark:placeholder-slate-400 {errors.churchName ? 'border-red-300 dark:border-red-600' : ''}"
									placeholder="Your church or ministry"
								/>
								{#if errors.churchName}
									<p class="mt-1 text-sm text-red-600 dark:text-red-300">{errors.churchName}</p>
								{/if}
							</div>
						</div>
					{:else}
						<div class="space-y-6">
							<h2 class="text-xl font-semibold text-slate-900 dark:text-white">Review your application</h2>
							<div class="rounded-xl border border-slate-200 card-premium bg-slate-50/50 p-6 dark:bg-white/5">
								<dl class="grid gap-4 sm:grid-cols-2">
									<div>
										<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Name</dt>
										<dd class="mt-1 font-medium text-slate-900 dark:text-white">{formData.name}</dd>
									</div>
									<div>
										<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Email</dt>
										<dd class="mt-1 font-medium text-slate-900 dark:text-white">{formData.email}</dd>
									</div>
									<div>
										<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Phone</dt>
										<dd class="mt-1 font-medium text-slate-900 dark:text-white">{formData.phone}</dd>
									</div>
									<div>
										<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Calling</dt>
										<dd class="mt-1 font-medium text-slate-900 dark:text-white">{formData.calling}</dd>
									</div>
									<div>
										<dt class="text-sm font-medium text-slate-500 dark:text-slate-300">Church</dt>
										<dd class="mt-1 font-medium text-slate-900 dark:text-white">{formData.churchName}</dd>
									</div>
								</dl>
							</div>
							<p class="text-sm text-slate-600 dark:text-slate-200">
								{#if preselectedCourse}
									By submitting, you'll be taken to your dashboard to review your course and complete payment—no need to select a course again.
								{:else}
									By submitting, you'll be taken to your dashboard. There you'll verify your account, choose a course, and complete payment.
								{/if}
							</p>
							{#if submitError}
								<p class="text-sm text-red-600 dark:text-red-300">{submitError}</p>
							{/if}
						</div>
					{/if}

					<!-- Actions -->
					<div class="mt-6 sm:mt-10 flex items-center justify-between gap-4 border-t border-slate-200 section-separator pt-6 sm:pt-8">
						<button
							type="button"
							onclick={prev}
							class="rounded-xl px-5 py-2.5 text-sm font-medium text-slate-600 transition hover:bg-slate-100 dark:text-slate-200 dark:hover:bg-white/10 {currentStep === 1 ? 'invisible' : ''}"
						>
							Back
						</button>
						<button
							type="button"
							onclick={next}
							disabled={submitting}
							class="rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-2.5 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 disabled:opacity-60 disabled:cursor-not-allowed"
						>
							{currentStep === 3 ? (submitting ? 'Creating account…' : 'Create account') : 'Continue'}
						</button>
					</div>
				</div>
			</div>
		</div>
	</main>

	<Footer />
</div>
