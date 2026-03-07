<script lang="ts">
	import { onMount } from 'svelte';
	import { SITE } from '$lib/config';

	let open = $state(false);
	let mode = $state<'choice' | 'form'>('choice');
	let formData = $state({ name: '', email: '', message: '' });
	let formSent = $state(false);
	/** When closed: 0 = chat icon, 1 = whatsapp icon; toggles for blink effect */
	let fabIcon = $state(0);

	const whatsappUrl = `https://wa.me/${SITE.whatsappFull.replace('+', '')}`;

	onMount(() => {
		const id = setInterval(() => {
			if (!open) fabIcon = fabIcon === 0 ? 1 : 0;
		}, 1500);
		return () => clearInterval(id);
	});

	function openChat() {
		open = true;
		mode = 'choice';
		formSent = false;
		formData = { name: '', email: '', message: '' };
	}

	function closeChat() {
		open = false;
		mode = 'choice';
	}

	function showForm() {
		mode = 'form';
	}

	function backToChoice() {
		mode = 'choice';
	}

	function submitForm(e: Event) {
		e.preventDefault();
		const { name, email, message } = formData;
		const subject = encodeURIComponent(`Pastoral School enquiry from ${name}`);
		const body = encodeURIComponent(
			`Name: ${name}\nEmail: ${email}\n\nMessage:\n${message}`
		);
		window.location.href = `mailto:${SITE.contactEmail}?subject=${subject}&body=${body}`;
		formSent = true;
	}

	function openWhatsApp() {
		window.open(whatsappUrl, '_blank', 'noopener,noreferrer');
		closeChat();
	}
</script>

<div class="fixed bottom-[max(1.5rem,env(safe-area-inset-bottom))] right-[max(1rem,env(safe-area-inset-right))] z-50">
	<!-- Chat panel -->
	{#if open}
		<div
			class="mb-4 w-[min(90vw,24rem)] max-w-sm overflow-hidden rounded-2xl border border-slate-200/80 card-premium bg-white shadow-2xl shadow-slate-900/20 dark:bg-[#2f3a52] dark:shadow-none"
		>
			<!-- Header -->
			<div class="flex items-center justify-between border-b border-slate-100 section-separator bg-gradient-to-r from-pink-50 to-purple-50 px-5 py-4 dark:from-[#374266] dark:to-[#45527a]">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-pink-500 to-purple-600 text-white">
						<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
							/>
						</svg>
					</div>
					<div>
						<p class="font-semibold text-slate-900 dark:text-white">Need help?</p>
						<p class="text-xs text-slate-500 dark:text-slate-200">We're here to assist</p>
					</div>
				</div>
				<button
					type="button"
					onclick={closeChat}
					class="rounded-lg p-2 text-slate-600 hover:bg-white/80 hover:text-slate-900 dark:text-slate-100 dark:hover:bg-white/20 dark:hover:text-white"
					aria-label="Close chat"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Content -->
			<div class="p-5">
				{#if mode === 'choice'}
					<p class="mb-4 text-sm text-slate-600 dark:text-slate-100">How would you like to reach us?</p>
					<div class="flex flex-col gap-3">
						<button
							type="button"
							onclick={showForm}
							class="flex items-center gap-4 rounded-xl border-2 border-slate-200 p-4 text-left transition-all hover:border-pink-300 hover:bg-pink-50/50 dark:border-white/[0.12] dark:text-white dark:hover:border-pink-500/50 dark:hover:bg-white/10"
						>
							<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-pink-500 to-purple-600 text-white">
								<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
									/>
								</svg>
							</div>
							<div>
								<p class="font-semibold text-slate-900 dark:text-white">Send a message</p>
								<p class="text-xs text-slate-500 dark:text-slate-200">We'll reply via email</p>
							</div>
						</button>
						<button
							type="button"
							onclick={openWhatsApp}
							class="flex items-center gap-4 rounded-xl border-2 border-slate-200 p-4 text-left transition-all hover:border-[#25d366]/50 hover:bg-[#25d366]/5 dark:border-white/[0.12] dark:text-white dark:hover:border-[#25d366]/60 dark:hover:bg-[#25d366]/10"
						>
							<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-[#25d366] text-white">
								<svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
									<path
										d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"
									/>
								</svg>
							</div>
							<div>
								<p class="font-semibold text-slate-900 dark:text-white">Chat on WhatsApp</p>
								<p class="text-xs text-slate-500 dark:text-slate-200">Quick replies, usually within hours</p>
							</div>
						</button>
					</div>
				{:else}
					{#if formSent}
						<div class="py-8 text-center">
							<div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-green-100 text-green-600 dark:bg-green-900/40 dark:text-green-300">
								<svg class="h-7 w-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
							</div>
							<p class="font-medium text-slate-900 dark:text-white">Your email client will open</p>
							<p class="mt-1 text-sm text-slate-500 dark:text-slate-200">Send the message to reach us.</p>
							<button
								type="button"
								onclick={backToChoice}
								class="mt-4 text-sm font-medium text-pink-600 hover:text-pink-700 dark:text-pink-400 dark:hover:text-pink-300"
							>
								← Back to options
							</button>
						</div>
					{:else}
						<div class="mb-4 flex items-center gap-2">
							<button
								type="button"
								onclick={backToChoice}
								class="rounded-lg p-1 text-slate-500 hover:bg-slate-100 hover:text-slate-700 dark:text-slate-200 dark:hover:bg-white/20 dark:hover:text-white"
								aria-label="Back"
							>
								<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
								</svg>
							</button>
							<p class="font-semibold text-slate-900 dark:text-white">Send a message</p>
						</div>
						<form onsubmit={submitForm} class="space-y-4">
							<div>
								<label for="chat-name" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Name</label>
								<input
									id="chat-name"
									type="text"
									bind:value={formData.name}
									required
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-2.5 text-sm text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#374266] dark:text-white dark:placeholder-slate-400"
									placeholder="Your name"
								/>
							</div>
							<div>
								<label for="chat-email" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Email</label>
								<input
									id="chat-email"
									type="email"
									bind:value={formData.email}
									required
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-2.5 text-sm text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#374266] dark:text-white dark:placeholder-slate-400"
									placeholder="you@example.com"
								/>
							</div>
							<div>
								<label for="chat-message" class="block text-sm font-medium text-slate-700 dark:text-slate-100">Message</label>
								<textarea
									id="chat-message"
									bind:value={formData.message}
									required
									rows="3"
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-2.5 text-sm text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500 dark:border-white/[0.12] dark:bg-[#374266] dark:text-white dark:placeholder-slate-400"
									placeholder="How can we help?"
								></textarea>
							</div>
							<button
								type="submit"
								class="w-full rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-4 py-3 text-sm font-semibold text-white transition hover:from-pink-600 hover:to-purple-700"
							>
								Send via email
							</button>
						</form>
					{/if}
				{/if}
			</div>
		</div>
	{/if}

	<!-- Chat button: blinks and switches chat / WhatsApp when closed -->
	<button
		type="button"
		onclick={() => (open ? closeChat() : openChat())}
		class="flex h-14 w-14 min-w-[3.5rem] min-h-[3.5rem] items-center justify-center rounded-full bg-gradient-to-br from-pink-500 to-purple-600 text-white shadow-lg shadow-pink-500/30 transition hover:scale-105 hover:shadow-xl active:scale-95 touch-manipulation {!open ? 'animate-chatbot-blink' : ''}"
		aria-label="Open chat"
	>
		{#if open}
			<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
			</svg>
		{:else if fabIcon === 0}
			<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
				/>
			</svg>
		{:else}
			<svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
				<path
					d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"
				/>
			</svg>
		{/if}
	</button>
</div>

<style>
	@keyframes chatbot-blink {
		0%,
		100% {
			opacity: 1;
			box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1),
				0 0 0 0 rgb(236 72 153 / 0.3);
		}
		50% {
			opacity: 0.92;
			box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1),
				0 0 0 8px rgb(236 72 153 / 0.15);
		}
	}
	.animate-chatbot-blink {
		animation: chatbot-blink 1.5s ease-in-out infinite;
	}
</style>
