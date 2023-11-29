<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { confirm } from '@tauri-apps/api/dialog';
	import { queue } from '$lib/ts/stores';
	import { addLog, logs, Log } from '$lib/ts/log';
	import QueueSong from '$lib/svelte/QueueSong.svelte';
	import LogComponent from '$lib/svelte/Log.svelte';

	let urls = '';

	function addToQueue() {
		if (urls.length <= 0) return;

		urls.trim()
			.split('\n')
			.forEach(url =>
				invoke('add_to_queue', { url }).catch(reason => addLog(new Log(false, reason))),
			);

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

<div class="columns is-desktop">
	<div class="column is-7-desktop is-flex is-flex-direction-column">
		<textarea
			class="textarea block mb-4"
			placeholder="Enter one URL per line"
			rows="6"
			bind:value={urls} />

		<button class="button mb-4" on:click={addToQueue}>
			<span class="icon">
				<i class="fas fa-plus" />
			</span>
			<span>Add to queue</span>
		</button>

		<div class="box">
			<h2 class="subtitle">Logs</h2>
			<div class="logs-wrapper">
				{#each $logs as log}
					<LogComponent {log} />
				{/each}
			</div>
		</div>
	</div>

	<div class="column is-5-desktop is-flex is-flex-direction-column">
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

		<div class="block box list has-overflow-ellipsis is-flex-grow-1">
			<h2 class="subtitle">Queue</h2>
			<div class="list">
				{#each $queue as queueSong}
					<QueueSong {queueSong} />
				{:else}
					<h2 class="subtitle has-text-centered m-auto pt-6 is-unselectable">
						Queue empty
					</h2>
				{/each}
			</div>
		</div>
	</div>
</div>

<!-- FIXME: Make content fit window size -->
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

	.logs-wrapper {
		display: flex;
		flex-direction: column-reverse;
		height: 20em;
		overflow: hidden auto;
	}
</style>
