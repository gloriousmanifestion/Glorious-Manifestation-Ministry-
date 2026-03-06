<svelte:options css="injected" />

<script lang="ts">
	import { page } from '$app/stores';
	import { SITE, type SeoProps } from '$lib/seo';

	/** SEO props - pass from page load or component */
	let {
		title,
		description = SITE.description,
		keywords = SITE.keywords,
		image,
		imageAlt = SITE.description,
		url,
		type = 'website',
		noIndex = false,
		canonical
	}: SeoProps = $props();

	// Resolve full URLs — use SITE.baseUrl for social sharing so og:url/og:image always point to 1nga.com
	const origin = $derived(SITE.baseUrl.replace(/\/$/, ''));
	const path = $derived($page.url.pathname);
	const fullUrl = $derived(url || canonical || `${origin}${path}`);
	const fullImage = $derived(
		image?.startsWith('http') ? image : image ? `${origin}${image.startsWith('/') ? '' : '/'}${image}` : `${origin}${SITE.ogImage}`
	);
	const fullTitle = $derived(title ? `${title} — ${SITE.name}` : `${SITE.name} — ${SITE.tagline}`);
</script>

<svelte:head>
	<title>{fullTitle}</title>
	<meta name="description" content={description} />
	<meta name="keywords" content={keywords} />
	{#if noIndex}
		<meta name="robots" content="noindex, nofollow" />
	{:else}
		<meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1" />
	{/if}
	<link rel="canonical" href={fullUrl} />

	<!-- Open Graph / Facebook -->
	<meta property="og:type" content={type} />
	<meta property="og:site_name" content={SITE.name} />
	<meta property="og:title" content={fullTitle} />
	<meta property="og:description" content={description} />
	<meta property="og:url" content={fullUrl} />
	<meta property="og:image" content={fullImage} />
	<meta property="og:image:alt" content={imageAlt} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta property="og:locale" content={SITE.locale} />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:site" content={SITE.twitterHandle} />
	<meta name="twitter:creator" content={SITE.creator} />
	<meta name="twitter:title" content={fullTitle} />
	<meta name="twitter:description" content={description} />
	<meta name="twitter:image" content={fullImage} />
	<meta name="twitter:image:alt" content={imageAlt} />

	<!-- Additional social -->
	<meta property="og:image:type" content="image/png" />
</svelte:head>
