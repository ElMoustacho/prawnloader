<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let url = 'https://www.youtube.com/watch?v=rmI3TpefQpk';
	let downloads: any[] = [];

	onMount(() =>
		listen<any[]>('queue_update', (e) => {
			console.info('Got ', e.payload);
			downloads = e.payload;
		})
	);

	async function download() {
		try {
			await invoke('add_to_queue', { url });
		} catch (error) {
			console.warn('mauvais url, tu pues.');
		}
	}
</script>

<h1 class="title">Downloads</h1>

<div class="columns">
	<div class="column">
		<input type="text" bind:value={url} />
		<button on:click={() => download()}> Add to queue </button>
	</div>

	<button on:click={() => invoke('download_queue', {})} />

	<div class="column">
		<h1>Queue of downloads</h1>
		<div>
			{#each downloads as download}
				<p>
					{download.title}<br /><small>{download.album.name}</small>
				</p>
			{:else}
				<h2>No downloads :(</h2>
			{/each}
		</div>
	</div>
</div>
