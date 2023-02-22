<script lang="ts">
	import type { Song } from 'src/types/music';
	import { invoke } from '@tauri-apps/api';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { confirm, message } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import bufferToImg from '$lib/ts/bufferToImg';
	import unlistenAllTauriEvents from '$lib/ts/unlistenAllTauriEvents';

	let urls = '';
	let downloads: Song[] = [
		// {
		// 	title: 'Violence No Matter What (Duet with Lzzy Hale)',
		// 	track: 2,
		// 	album: {
		// 		artist: 'Avatar',
		// 		cover: null,
		// 		name: 'Dance Devil Dance',
		// 		year: 2023,
		// 	},
		// },
		// {
		// 	title: 'Gotta Wanna Riot',
		// 	track: 3,
		// 	album: {
		// 		artist: 'Avatar',
		// 		cover: null,
		// 		name: 'Dance Devil Dance',
		// 		year: 2023,
		// 	},
		// },
	];

	onMount(() => {
		let events: Promise<UnlistenFn>[] = [];

		events.push(listen('queue_update', e => (downloads = e.payload)));

		events.push(listen('parse_error', e => message(`Error parsing url: "${e.payload}"`)));

		events.push(listen('download_complete', e => message(`Download "${e.payload}" complete!`)));

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
		<button class="button" on:click={addToQueue}> Add to queue </button>
	</div>

	<div class="column is-5-desktop">
		<div class="pb-4 is-flex">
			<button class="mx-1 is-flex-grow-1 button is-primary" on:click={downloadQueue}
				>Download</button>
			<button class="mx-1 is-flex-grow-1 button is-danger" on:click={clearQueue}
				>Clear queue</button>
		</div>

		<div class="block box">
			{#each downloads as download}
				<div class="song p-3 is-flex is-align-items-center is-unselectable">
					<figure class="is-flex-shrink-0 image is-32x32">
						<img
							src={download.album.cover
								? bufferToImg(download.album.cover)
								: 'https://cdns-images.dzcdn.net/images/cover/2b944b29fc4ab95482da6e968ec03586/500x500-000000-80-0-0.jpg'}
							alt="" />
					</figure>

					<div class="px-3 is-flex-grow-1 is-single-line">
						<p
							class="is-size-6 has-text-weight-bold is-single-line"
							title={download.title}>
							{download.title}
						</p>

						<div class="is-flex is-justify-content-space-between">
							<span class="is-single-line has-text-black-bis"
								>{download.album.artist}</span>
							<span class="is-single-line" title={download.album.name}
								>{download.album.name}</span>
						</div>
					</div>

					<button class="delete" aria-label="delete" />
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

	.song {
		border-radius: 5px;

		&:hover {
			background-color: $light;
		}
	}

	// DEBUG: Max height
	.column > .block {
		height: 300px;
		overflow-y: auto;
	}
</style>
