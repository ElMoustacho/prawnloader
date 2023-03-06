<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { confirm } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import unlistenAllTauriEvents from '$lib/ts/unlistenAllTauriEvents';
	import { queue } from '$lib/ts/stores';
	import QueueSong from './QueueSong.svelte';

	let urls = '';

	onMount(() => {
		let events: Promise<UnlistenFn>[] = [];

		events.push(listen('parse_error', e => console.info(`Error parsing url: "${e.payload}"`)));

		events.push(
			listen('download_complete', e => console.info(`Download "${e.payload}" complete!`))
		);

		return () => unlistenAllTauriEvents(events);
	});

	function addToQueue() {
		urls.trim()
			.split('\n')
			.forEach(url => invoke('add_to_queue', { url }));

		urls = '';
	}

	function downloadQueue() {
		for (let i = 0; i < $queue.length; i++) {
			invoke('download', { index: i });
		}
	}

	async function clearQueue() {
		if ((await confirm('Do you want to clear the queue?')) === true) {
			invoke('clear_queue', {});
		}
	}
</script>

<h1 class="title">Downloads</h1>

<div class="columns is-desktop">
	<div class="column is-7-desktop">
		<textarea
			class="textarea block"
			placeholder="Enter one URL per line"
			rows="10"
			bind:value={urls} />
		<button class="button" on:click={addToQueue}>
			<span class="icon">
				<i class="fas fa-plus" />
			</span>
			<span>Add to queue</span>
		</button>
	</div>

	<div class="column is-5-desktop">
		<div class="pb-4 is-flex">
			<button class="mx-1 is-flex-grow-1 button is-primary" on:click={downloadQueue}>
				<span class="icon">
					<i class="fa fa-download" />
				</span>
				<span>Download all</span>
			</button>
			<button class="mx-1 is-flex-grow-1 button is-danger" on:click={clearQueue}>
				<span class="icon">
					<i class="fa fa-trash" />
				</span>
				<span>Clear all</span>
			</button>
		</div>

		<div class="block box list has-overflow-ellipsis">
			{#each $queue as queueSong}
				<QueueSong {queueSong} />
			{:else}
				<h2 class="subtitle has-text-centered">You have no song in the queue</h2>
			{/each}
		</div>
	</div>
</div>

<style lang="scss">
	@import 'bulma/sass/utilities/derived-variables.sass';
	@import 'bulma/sass/utilities/functions.sass';

	.textarea {
		white-space: pre;
		overflow-wrap: normal;
		overflow-x: auto;
		resize: none;
		scrollbar-width: thin;
	}

	// DEBUG: Max height
	.column > .block {
		height: 300px;
		overflow-y: auto;
	}
</style>
