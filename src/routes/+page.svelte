<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { confirm, message } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import bufferToImg from '$lib/ts/bufferToImg';
	import unlistenAllTauriEvents from '$lib/ts/unlistenAllTauriEvents';
	import { queue } from '$lib/ts/stores';

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
		const _urls = urls.trim().split('\n');
		invoke('add_to_queue', { urls: _urls });

		urls = '';
	}

	function downloadQueue() {
		invoke('download_queue', {});
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
			{#each $queue as download}
				<div class="list-item">
					<div class="list-item-image">
						<figure class="image is-32x32">
							<img
								src={download.album.cover
									? bufferToImg(download.album.cover)
									: 'https://cdns-images.dzcdn.net/images/cover/2b944b29fc4ab95482da6e968ec03586/500x500-000000-80-0-0.jpg'}
								alt="" />
						</figure>
					</div>

					<div class="list-item-content">
						<div class="list-item-title" title={download.title}>{download.title}</div>
						<div class="list-item-description">
							<div class="is-flex is-justify-content-space-between">
								<span title={download.album.name}>{download.album.name}</span>
								<span
									title={download.album.artist}
									class="is-single-line has-text-black-bis"
									>{download.album.artist}</span>
							</div>
						</div>
					</div>

					<div class="list-item-controls">
						<div class="buttons is-right">
							<!-- TODO: Add actions to buttons -->
							<button class="button">
								<span class="icon is-small">
									<i class="fas fa-download" />
								</span>
							</button>
							<button class="button is-danger">
								<span class="icon is-small">
									<i class="fas fa-trash" />
								</span>
							</button>
						</div>
					</div>
				</div>
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
