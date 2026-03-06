<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import SeoHead from '$lib/components/SeoHead.svelte';
	import Chatbot from '$lib/components/Chatbot.svelte';
	import { hydrateUser } from '$lib/auth';

	onMount(() => {
		hydrateUser();
	});
</script>

<SeoHead />

<slot />

<Chatbot />
