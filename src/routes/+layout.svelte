<script lang="ts">
	import { addLog, formatLogDownloadError, formatLogSuccess } from '$lib/log';
	import { queue } from '$lib/stores';
	import { listen } from '$lib/tauri-wrapper';
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import '../scss/app.scss';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];

	onMount(() => {
		// Download related event listeners
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
			addLog(formatLogDownloadError(...e.payload));
		});

		document.addEventListener('keydown', ctrlTabListener);
	});

	function ctrlTabListener(event: KeyboardEvent) {
		if (event.key === 'Tab' && event.ctrlKey) {
			if ($page.route.id === null) return;

			const currentTabIndex = Object.keys(Object.fromEntries(links)).indexOf($page.route.id);

			if (currentTabIndex === -1) return;
			let difference = event.shiftKey ? -1 : 1;

			let newIndex = (currentTabIndex + difference) % links.length;
			if (newIndex === -1) newIndex = links.length - 1;
			goto(links[newIndex][0]);
		}
	}
</script>

<div class="is-flex is-flex-direction-column is-maxheight">
	<div class="tabs m-0">
		<ul>
			{#each links as link}
				<!-- TODO: Add "is-active" when on a page under this link -->
				<li class:is-active={$page.route.id === link[0]}>
					<a data-sveltekit-preload-data href={link[0]}>
						{link[1]}
					</a>
				</li>
			{/each}
		</ul>
	</div>

	<div class="container py-4 is-maxheight is-fluid is-flex is-flex-direction-column">
		<slot />
	</div>
</div>

<style lang="scss">
	.container.is-maxheight.is-fluid.is-flex.is-flex-direction-column {
		overflow-y: auto;
	}
</style>
