<script lang="ts">
	import type { Song } from 'src/types/music';
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import bufferToImg from '$lib/bufferToImg';

	let url = '';
	let downloads: Song[] = [
		{
			title: 'Violence No Matter What (Duet with Lzzy Hale)',
			track: 2,
			album: {
				artist: 'Avatar',
				cover: null,
				name: 'Dance Devil Dance',
				year: 2023,
			},
		},
		{
			title: 'Gotta Wanna Riot',
			track: 3,
			album: {
				artist: 'Avatar',
				cover: null,
				name: 'Dance Devil Dance',
				year: 2023,
			},
		},
	];

	onMount(() => {
		listen('queue_update', e => {
			console.info('Got ', e.payload);
			downloads = e.payload;
		});
	});

	function addToQueue() {
		invoke('add_to_queue', { url });
	}

	function downloadQueue(e: MouseEvent) {
		invoke('download_queue', {});
	}
</script>

<h1 class="title">Downloads</h1>

<div class="columns">
	<div class="column is-8 ">
		<textarea
			class="textarea block"
			placeholder="Enter one URL per line"
			rows="10"
			bind:value={url}
		/>
		<button class="button" on:click={addToQueue}> Add to queue </button>
	</div>

	<div class="column is-4">
		<div class="block">
			{#each downloads as download}
				<article class="message">
					<div class="message-header">
						<p title={download.title}>{download.title}</p>
						<button class="delete" aria-label="delete" />
					</div>
					<div class="p-4 is-flex is-align-items-start message-body">
						<div class=" is-narrow img-wrapper">
							<img
								src={download.album.cover
									? bufferToImg(download.album.cover)
									: 'https://pbs.twimg.com/media/FlbeIf6X0AE45kc?format=jpg&name=small'}
								alt=""
							/>
						</div>
						<div class="ml-1">
							<p><small><b>Artist:</b> {download.album.artist}</small></p>
							<p><small><b>Album:</b> {download.album.name}</small></p>
							<p><small><b>Year:</b> {download.album.year}</small></p>
							<p><small><b>Track:</b> {download.track}</small></p>
						</div>
					</div>
				</article>
			{:else}
				<h2>No downloads :(</h2>
			{/each}
		</div>

		<button class="button is-primary" on:click={downloadQueue}>Download</button>
	</div>
</div>

<style lang="scss">
	@import 'bulma/sass/utilities/derived-variables.sass';
	@import 'bulma/sass/utilities/functions.sass';

	.textarea {
		white-space: pre;
		overflow-wrap: normal;
		overflow-x: scroll;
		resize: none;
	}

	.img-wrapper {
		width: 3em;
		height: 3em;
	}

	.message-header > p {
		white-space: nowrap;
		text-overflow: ellipsis;
		overflow: hidden;
	}
</style>
