<script lang="ts" context="module">
	import { writable } from 'svelte/store';

	let urls = writable('');
</script>

<script lang="ts">
	import { invoke } from '$lib/tauri-wrapper';
	import { confirm } from '@tauri-apps/api/dialog';
	import { queue } from '$lib/stores';
	import { addLog, logs, Log, clearLogs } from '$lib/log';
	import QueueSong from '$lib/components/QueueSong.svelte';
	import LogsList from '$lib/components/LogsList.svelte';

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
		$queue.forEach(queueItem => {
			if (queueItem.download_state === 'Downloading') return;

			invoke('request_download', {
				trackId: queueItem.song.id,
			});
		});
	}

	async function clearQueue() {
		if ((await confirm('Do you want to clear the queue?')) === true) {
			$queue = [];
		}
	}
</script>

<div class="columns is-mobile is-maxheight">
	<div class="column is-flex is-flex-direction-column">
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

		<fieldset class="box is-flex-grow-1">
			<legend class="subtitle m-0 is-unselectable" style="width: 100%;"
				>Logs
				<button
					class="button is-bordered is-pulled-right is-rounded s-y_bCXRrkrYfP"
					on:click={clearLogs}>
					<span class="icon">
						<i class="fa fa-trash" />
					</span>
					<span>Clear logs</span>
				</button>
			</legend>
			{#if $logs.length > 0}
				<LogsList />
			{:else}
				<h2 class="subtitle pt-2 has-text-centered has-text-grey-lighter is-unselectable">
					Logs empty
				</h2>
			{/if}
		</fieldset>
	</div>

	<div class="column is-flex is-flex-direction-column">
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

		<fieldset class="block box has-overflow-ellipsis is-flex-grow-1">
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
	.textarea {
		// 6 lines times the line-height (1.5)
		height: 6 * 1.5em;
		white-space: pre;
		overflow-wrap: normal;
		overflow-x: auto;
		resize: none;
		scrollbar-width: thin;
	}

	fieldset {
		// With flexbox, prevents the fieldset from growing beyond its initial size
		height: 0;
	}
</style>
