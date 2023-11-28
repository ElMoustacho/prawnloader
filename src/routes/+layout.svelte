<script lang="ts">
	import { queue } from '$lib/ts/stores';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	import '../scss/app.scss';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];

	// Download related event listeners
	onMount(() => {
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
