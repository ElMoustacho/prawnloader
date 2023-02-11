<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	onMount(() => listen('queue_update', (e) => console.info('Update: ', e)));
</script>

<h1 class="title">Downloads</h1>

<button
	on:click={() =>
		invoke('add_to_queue', {
			url: 'https://www.youtube.com/watch?v=rmI3TpefQpk',
		})}
>
	Add to queue
</button>
