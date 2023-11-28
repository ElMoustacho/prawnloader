<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { confirm } from '@tauri-apps/api/dialog';
	import { queue } from '$lib/ts/stores';
	import QueueSong from './QueueSong.svelte';

	let urls = '';

	function addToQueue() {
		urls.trim()
			.split('\n')
			.forEach(url => invoke('request_download', { url }));

		urls = '';
	}

	function downloadQueue() {
		throw new Error('Not implemented yet.');
	}

	async function clearQueue() {
		if ((await confirm('Do you want to clear the queue?')) === true) {
			throw new Error('Not implemented yet.');
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
			<button disabled class="mx-1 is-flex-grow-1 button is-primary" on:click={downloadQueue}>
				<span class="icon">
					<i class="fa fa-download" />
				</span>
				<span>Download all</span>
			</button>
			<button disabled class="mx-1 is-flex-grow-1 button is-danger" on:click={clearQueue}>
				<span class="icon">
					<i class="fa fa-trash" />
				</span>
				<span>Clear all</span>
			</button>
		</div>

		<div class="block box p-1 list has-overflow-ellipsis">
			{#each $queue as queueSong}
				<QueueSong {queueSong} />
			{:else}
				<h2 class="subtitle has-text-centered m-auto">You have no song in the queue</h2>
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
