<script lang="ts">
	import { queue } from '$lib/ts/stores';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	import '../scss/app.scss';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];

	onMount(() => listen('queue_update', e => ($queue = e.payload)));

	$: console.log($queue);
</script>

<div class="tabs">
	<ul>
		{#each links as link}
			<!-- TODO: Add "is-active" when on a page under this link -->
			<a class="navbar-item" data-sveltekit-preload-data href={link[0]}>
				{link[1]}
			</a>
		{/each}
	</ul>
</div>

<section class="section">
	<div class="container">
		<slot />
	</div>
</section>
