<script lang="ts" context="module">
	import { writable } from 'svelte/store';

	let urls = writable('');
</script>

<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { confirm } from '@tauri-apps/api/dialog';
	import { queue } from '$lib/ts/stores';
	import { addLog, logs, Log } from '$lib/ts/log';
	import QueueSong from '$lib/svelte/QueueSong.svelte';
	import LogComponent from '$lib/svelte/Log.svelte';

	function addToQueue() {
		if ($urls.length <= 0) return;
		$urls
			.trim()
			.split('\n')
			.forEach(url =>
				invoke('add_to_queue', { url }).catch(reason => addLog(new Log(false, reason))),
			);
		$urls = '';
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

<div class="columns m-0 is-flex-grow-1 max-overflow">
	<div class="max-overflow column is-flex is-flex-direction-column">
		<textarea
			class="textarea block mb-4"
			placeholder="Enter one URL per line"
			bind:value={$urls} />

		<button class="button mb-4" on:click={addToQueue}>
			<span class="icon">
				<i class="fas fa-plus" />
			</span>
			<span>Add to queue</span>
		</button>

		<fieldset class="box is-flex-grow-1 max-overflow">
			<legend class="subtitle m-0 is-unselectable">Logs</legend>
			{#if $logs.length > 0}
				<div class="logs-wrapper max-overflow">
					{#each $logs as log}
						<LogComponent {log} />
					{/each}
				</div>
			{:else}
				<h2 class="subtitle pt-2 has-text-centered has-text-grey-lighter is-unselectable">
					Logs empty
				</h2>
			{/if}
		</fieldset>
	</div>

	<div class="max-overflow column is-flex is-flex-direction-column max-overflow">
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

		<fieldset class="block box list has-overflow-ellipsis is-flex-grow-1 max-overflow">
			<legend class="subtitle m-0 is-unselectable">Queue</legend>
			{#if $queue.length > 0}
				<div class="list max-overflow">
					{#each $queue as queueSong}
						<QueueSong {queueSong} />
					{/each}
				</div>
			{:else}
				<h2 class="subtitle pt-2 has-text-centered has-text-grey-lighter is-unselectable">
					Queue empty
				</h2>
			{/if}
		</fieldset>
	</div>
</div>

<style lang="scss">
	@import 'bulma/sass/utilities/derived-variables.sass';
	@import 'bulma/sass/utilities/functions.sass';

	.textarea {
		// 6 lines times the line-height (1.5)
		height: 6 * 1.5em;
		white-space: pre;
		overflow-wrap: normal;
		overflow-x: auto;
		resize: none;
		scrollbar-width: thin;
	}

	.logs-wrapper {
		display: flex;
		flex-direction: column-reverse;
		overflow: hidden auto;
	}

	// 🍝 Allows to scroll lists without overflowing
	.max-overflow {
		max-height: 100%;
		overflow: auto;
	}
</style>
