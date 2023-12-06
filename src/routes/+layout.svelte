<script lang="ts">
	import { addLog, formatLogDownloadError, formatLogSuccess } from '$lib/ts/log';
	import { listen } from '$lib/tauri-wrapper';
	import { queue } from '$lib/ts/stores';
	import { onMount } from 'svelte';

	import '../scss/app.scss';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];

	onMount(() => {
		// Download related event listeners
		listen('add_to_queue', e => {
			$queue.push({
				download_state: 'Inactive',
				song: e.payload,
			});
			$queue = $queue;
		});

		listen('start', e => {
			const song = e.payload;
			const firstSongIndex = $queue.findIndex(
				queueSong =>
					queueSong.song.id === song.id && queueSong.download_state === 'Inactive',
			);

			if (firstSongIndex < 0) return;

			$queue[firstSongIndex].download_state = 'Downloading';
		});

		listen('finish', e => {
			const song = e.payload;
			const firstSongIndex = $queue.findIndex(queueSong => queueSong.song.id === song.id);

			if (firstSongIndex < 0) return;

			$queue.splice(firstSongIndex, 1);
			$queue = $queue;

			addLog(formatLogSuccess(song));
		});

		// Error related event listeners
		listen('download_error', e => {
			addLog(formatLogDownloadError(e.payload));
		});
	});
</script>

<div class="tabs has-background-white">
	<ul>
		{#each links as link}
			<!-- TODO: Add "is-active" when on a page under this link -->
			<a class="navbar-item" data-sveltekit-preload-data href={link[0]}>
				{link[1]}
			</a>
		{/each}
	</ul>
</div>

<div class="container is-fluid is-flex is-flex-direction-column">
	<slot />
</div>

<style lang="scss">
	.tabs {
		position: sticky;
		top: 0;
		z-index: 10;
	}

	.container {
		// 🍝 Allows to enfore height rules in flexboxes,
		// but doesn't affect it because it is in a flexbox
		height: 0;
		width: 100%;
		overflow: visible;
	}
</style>
