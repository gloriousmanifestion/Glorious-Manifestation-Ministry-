<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import SeoHead from '$lib/components/SeoHead.svelte';
	import Chatbot from '$lib/components/Chatbot.svelte';
	import HeroLogoProgress from '$lib/components/HeroLogoProgress.svelte';
	import { hydrateUser } from '$lib/auth';
	import { initTheme } from '$lib/stores/theme';

	onMount(() => {
		initTheme();
		hydrateUser();
	});
</script>

<SeoHead />

<slot />

<HeroLogoProgress />
<Chatbot />
