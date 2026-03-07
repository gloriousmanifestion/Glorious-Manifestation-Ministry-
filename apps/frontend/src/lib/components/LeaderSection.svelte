<script lang="ts">
	import AnimateOnView from '$lib/components/AnimateOnView.svelte';
	import { SITE } from '$lib/config';

	const leader = SITE.leader;
</script>

<section class="border-t border-slate-200/50 section-separator bg-white py-20 sm:py-28 px-4 sm:px-6 dark:bg-[#374266]/80">
	<div class="mx-auto max-w-5xl">
		<AnimateOnView animation="fade-up">
			<div class="flex flex-col lg:flex-row lg:items-center lg:gap-20">
				<!-- Image with refined frame -->
				<div class="lg:w-[42%] shrink-0 order-2 lg:order-1">
					<div class="relative max-w-sm mx-auto lg:mx-0">
						<!-- Subtle gold-tinted frame -->
						<div class="absolute -inset-4 rounded-[1.25rem] border border-amber-200/50 bg-gradient-to-br from-amber-50/60 to-amber-100/30 dark:border-white/10 dark:from-amber-900/30 dark:to-amber-800/20" aria-hidden="true"></div>
						<div class="relative overflow-hidden rounded-xl shadow-2xl shadow-slate-300/40 ring-1 ring-slate-200/60 dark:ring-white/[0.08]">
							<img
								src="/prophetess-leader.png"
								alt="{leader.name} — {leader.title} of the Pastoral School"
								class="w-full object-cover object-top aspect-[3/4]"
							/>
						</div>
					</div>
				</div>

				<!-- Content -->
				<div class="order-1 lg:order-2 lg:flex-1">
					<h2 class="text-2xl font-semibold tracking-tight text-slate-900 dark:text-white sm:text-3xl">
						{leader.name}
					</h2>
					<p class="mt-2 text-base font-medium text-slate-600 dark:text-slate-200">{leader.title}</p>
					<div class="mt-8 w-12 h-px bg-gradient-to-r from-amber-400/80 to-amber-200/40 dark:from-amber-500/80 dark:to-amber-600/60" aria-hidden="true"></div>
					<p class="mt-8 text-slate-600 dark:text-slate-200 leading-[1.7] text-[15px]">
						{leader.bio}
					</p>
				</div>
			</div>
		</AnimateOnView>
	</div>
</section>
