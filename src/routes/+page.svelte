<script lang="ts" context="module">
	import DeezerAlbum from '$lib/components/queue_items/DeezerAlbum.svelte';
	import DeezerTrack from '$lib/components/queue_items/DeezerTrack.svelte';
	import YoutubePlaylist from '$lib/components/queue_items/YoutubePlaylist.svelte';
	import YoutubeVideo from '$lib/components/queue_items/YoutubeVideo.svelte';
	import type { QueueItem } from '$lib/music';
	import type { ComponentType } from 'svelte';
	import { writable } from 'svelte/store';

	let urls = writable('');

	const components: Map<
		Item['type'],
		ComponentType<SvelteComponent<{ queueItem: QueueItem }>>
	> = new Map();
	components
		.set('DeezerAlbum', DeezerAlbum)
		.set('DeezerTrack', DeezerTrack)
		.set('YoutubePlaylist', YoutubePlaylist)
		.set('YoutubeVideo', YoutubeVideo);
</script>

<script lang="ts">
	import LogsList from '$lib/components/LogsList.svelte';
	import { Log, addLog, clearLogs, logs } from '$lib/log';
	import { queue } from '$lib/stores';
	import { invoke } from '$lib/tauri-wrapper';
	import type { Item } from '$models/Item';
	import { confirm } from '@tauri-apps/api/dialog';
	import type { SvelteComponent } from 'svelte';
	import { onDestroy, onMount } from 'svelte';
	import { v4 as uuidv4 } from 'uuid';

	function addToQueue() {
		if ($urls.length <= 0) return;
		$urls
			.trim()
			.split('\n')
			.forEach(url =>
				invoke('get_item', { url }).then(
					item => {
						$queue.push({
							item,
							request_id: uuidv4(),
							download_status: 'Inactive',
							error: false,
						});
						$queue = $queue;
					},
					reason => addLog(new Log(false, reason)),
				),
			);
		$urls = '';
	}

	function downloadQueue() {
		$queue.forEach(queueItem => {
			if (queueItem.download_status === 'Downloading') return;
			invoke('request_download', { request: queueItem });
		});
	}

	async function clearQueue() {
		if ((await confirm('Do you want to clear the queue?')) === true) {
			$queue = [];
		}
	}

	function ctrlEnterListener(event: KeyboardEvent) {
		if (event.key === 'Enter' && event.ctrlKey) addToQueue();
	}

	let textarea: HTMLTextAreaElement;
	onMount(() => {
		document.addEventListener('keydown', ctrlEnterListener);

		// Needs to be set in a timeout to focus properly
		setTimeout(() => textarea.focus(), 0);
	});

	onDestroy(() => {
		document.removeEventListener('keydown', ctrlEnterListener);
	});
</script>

<div class="columns is-mobile is-maxheight">
	<div class="column is-flex is-flex-direction-column">
		<textarea
			class="textarea block mb-4"
			placeholder="Enter one URL per line"
			bind:this={textarea}
			bind:value={$urls} />

		<button class="button mb-4" on:click={addToQueue}>
			<span class="icon">
				<i class="fas fa-plus" />
			</span>
			<span title="<Ctrl+Enter> To add to queue.">Add to queue</span>
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
					{#each $queue as queueItem}
						<svelte:component this={components.get(queueItem.item.type)} {queueItem} />
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
