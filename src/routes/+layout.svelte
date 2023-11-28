<script lang="ts">
	import { logs, queue } from '$lib/ts/stores';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	import '../scss/app.scss';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];

	onMount(() => {
		// Download related event listeners
		listen('Queue', e => {
			$queue.push({
				download_state: 'Inactive',
				song: e.payload,
			});
			$queue = $queue;
		});

		listen('Start', e => {
			$queue.forEach(queueSong => {
				if (queueSong.song.id == e.payload.id) {
					queueSong.download_state = 'Downloading';
				}
			});
			$queue = $queue;
		});

		listen('Finish', e => {
			const firstSongIndex = $queue.findIndex(
				queueSong => queueSong.song.id === e.payload.id,
			);

			if (firstSongIndex < 0) return;

			$queue.splice(firstSongIndex, 1);
			$queue = $queue;
		});

		// TODO: Implement these events
		// Error related event listeners
		listen('DownloadError', e => {
			$logs.push(e.payload.toString());
			$logs = $logs;
		});

		listen('SongNotFoundError', e => {
			$logs.push(e.payload.toString());
			$logs = $logs;
		});

		listen('AlbumNotFoundError', e => {
			$logs.push(e.payload.toString());
			$logs = $logs;
		});
	});
</script>

<div class="tabs">
	<ul>
		{#each links as link}
			<!-- TODO: Add "is-active" when on a page under this link -->
			<a class="navbar-item" data-sveltekit-preload-data href={link[0]}>
				{link[1]}
			</a>
		{/each}
	</ul>
</div>

<section class="section">
	<div class="container">
		<slot />
	</div>
</section>
