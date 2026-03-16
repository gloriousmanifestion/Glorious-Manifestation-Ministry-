<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import SeoHead from '$lib/components/SeoHead.svelte';
	import Chatbot from '$lib/components/Chatbot.svelte';
	import HeroLogoProgress from '$lib/components/HeroLogoProgress.svelte';
	import { hydrateUser } from '$lib/auth';
	import { initTheme } from '$lib/stores/theme';
	import { pageSeo } from '$lib/stores/pageSeo';

	onMount(() => {
		initTheme();
		hydrateUser();
	});
</script>

<SeoHead
	title={$pageSeo.title}
	description={$pageSeo.description}
	keywords={$pageSeo.keywords}
	noIndex={$pageSeo.noIndex}
/>

<div class="pb-20 md:pb-0">
	<slot />
</div>

<HeroLogoProgress />
<Chatbot />
