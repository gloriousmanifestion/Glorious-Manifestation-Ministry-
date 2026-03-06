<script lang="ts">
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { submitContact } from '$lib/api/contact';
	import { SITE } from '$lib/config';

	const mapUrl = 'https://maps.google.com/?q=' + encodeURIComponent(SITE.address);
	const whatsappUrl = 'https://wa.me/' + SITE.whatsappFull.replace('+', '');

	let name = $state('');
	let email = $state('');
	let subject = $state('');
	let message = $state('');
	let loading = $state(false);
	let success = $state(false);
	let error = $state('');

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (loading) return;
		error = '';
		success = false;
		if (!name.trim() || !email.trim() || !message.trim()) {
			error = 'Please fill in name, email, and message.';
			return;
		}
		loading = true;
		try {
			await submitContact({ name: name.trim(), email: email.trim(), subject: subject.trim() || undefined, message: message.trim() });
			success = true;
			name = '';
			email = '';
			subject = '';
			message = '';
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to send message';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Contact — {SITE.name} Pastoral School</title>
	<meta
		name="description"
		content="Contact {SITE.name} Pastoral School. {SITE.infoEmail}. {SITE.address}, {SITE.location}."
	/>
</svelte:head>

<div class="min-h-screen flex flex-col bg-white">
	<Nav />

	<main class="flex-1">
		<!-- Hero -->
		<section class="px-4 sm:px-6 pt-24 sm:pt-32 pb-16 sm:pb-24">
			<div class="mx-auto max-w-2xl text-center">
				<h1 class="text-3xl font-semibold tracking-tight text-slate-900 sm:text-4xl">
					Contact the Pastoral School
				</h1>
				<p class="mt-4 text-slate-600">
					We're here to help with admissions, course enquiries, and visits. Reach out at your convenience.
				</p>
			</div>
		</section>

		<!-- Contact form + details: two columns on desktop -->
		<section class="px-4 sm:px-6 pb-20 sm:pb-28">
			<div class="mx-auto max-w-6xl grid grid-cols-1 lg:grid-cols-2 gap-10 lg:gap-16">
				<AnimateOnView animation="fade-up">
					<form onsubmit={handleSubmit} class="space-y-4 rounded-2xl border border-slate-200/80 bg-slate-50/50 p-6 sm:p-8">
						<h2 class="text-lg font-semibold text-slate-900">Send us a message</h2>
						{#if success}
							<div class="rounded-xl border border-green-200 bg-green-50 p-4 text-sm text-green-700">
								Thank you for your message. We'll get back to you soon.
							</div>
						{:else}
							{#if error}
								<div class="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-700">{error}</div>
							{/if}
							<div>
								<label for="contact-name" class="block text-sm font-medium text-slate-700">Name</label>
								<input
									id="contact-name"
									type="text"
									bind:value={name}
									required
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500"
									placeholder="Your name"
								/>
							</div>
							<div>
								<label for="contact-email" class="block text-sm font-medium text-slate-700">Email</label>
								<input
									id="contact-email"
									type="email"
									bind:value={email}
									required
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500"
									placeholder="your@email.com"
								/>
							</div>
							<div>
								<label for="contact-subject" class="block text-sm font-medium text-slate-700">Subject (optional)</label>
								<input
									id="contact-subject"
									type="text"
									bind:value={subject}
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500"
									placeholder="e.g. Course enquiry"
								/>
							</div>
							<div>
								<label for="contact-message" class="block text-sm font-medium text-slate-700">Message</label>
								<textarea
									id="contact-message"
									bind:value={message}
									required
									rows="4"
									class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500"
									placeholder="Your message..."
								></textarea>
							</div>
							<button
								type="submit"
								disabled={loading}
								class="w-full rounded-xl bg-slate-900 px-6 py-3.5 text-sm font-medium text-white transition hover:bg-slate-800 disabled:opacity-60"
							>
								{loading ? 'Sending…' : 'Send message'}
							</button>
						{/if}
					</form>
				</AnimateOnView>

				<AnimateOnView animation="fade-up" stagger={1}>
					<div class="space-y-8">
						<div class="border-b border-slate-200/80 pb-8">
							<p class="text-xs font-medium uppercase tracking-wider text-slate-400">Email</p>
							<a
								href="mailto:{SITE.infoEmail}"
								class="mt-1 block text-lg text-slate-900 hover:text-pink-600 transition break-all"
							>
								{SITE.infoEmail}
							</a>
						</div>

						<div class="border-b border-slate-200/80 pb-8">
							<p class="text-xs font-medium uppercase tracking-wider text-slate-400">Phone</p>
							<a
								href={whatsappUrl}
								target="_blank"
								rel="noopener noreferrer"
								class="mt-1 block text-lg text-slate-900 hover:text-pink-600 transition"
							>
								{SITE.whatsapp}
							</a>
						</div>

						<div class="border-b border-slate-200/80 pb-8">
							<p class="text-xs font-medium uppercase tracking-wider text-slate-400">Address</p>
							<p class="mt-1 text-slate-900 leading-relaxed">{SITE.address}</p>
							<p class="mt-0.5 text-slate-500">{SITE.location}</p>
							<a
								href={mapUrl}
								target="_blank"
								rel="noopener noreferrer"
								class="mt-3 inline-block text-sm font-medium text-pink-600 hover:text-pink-700"
							>
								Get directions →
							</a>
						</div>

						<div>
							<p class="text-xs font-medium uppercase tracking-wider text-slate-400">Hours</p>
							<p class="mt-1 text-slate-900">{SITE.schedule}</p>
							<p class="text-slate-500">{SITE.duration}</p>
						</div>
					</div>

					<div class="mt-16 text-center lg:text-left">
						<a
							href="/register"
							class="inline-block rounded-full bg-slate-900 px-8 py-3.5 text-sm font-medium text-white transition hover:bg-slate-800"
						>
							Register now
						</a>
					</div>
				</AnimateOnView>
			</div>
		</section>
	</main>

	<Footer />
</div>
