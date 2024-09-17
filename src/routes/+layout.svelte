<script lang="ts">
	import { queue } from '$lib/stores';
	import { listen } from '$lib/tauri-wrapper';
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import '../scss/app.scss';
	import { addLog, formatLogDownloadError, formatLogSuccess } from '$lib/log';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];

	onMount(() => {
		// Download related event listeners
		listen('start', ({ payload: uuid }) => {
			const firstSongIndex = $queue.findIndex(
				queueItem =>
					queueItem.request_id === uuid && queueItem.download_status === 'Inactive',
			);

			if (firstSongIndex < 0) return;

			$queue[firstSongIndex].download_status = 'Downloading';
		});

		listen('finish', ({ payload: uuid }) => {
			const itemIndex = $queue.findIndex(queueItem => queueItem.request_id === uuid);

			if (itemIndex < 0) return;

			addLog(formatLogSuccess($queue[itemIndex].item));

			$queue.splice(itemIndex, 1);
			$queue = $queue;
		});

		// Error related event listeners
		listen('download_error', ({ payload: [uuid, errMsg] }) => {
			const itemIndex = $queue.findIndex(item => item.request_id === uuid);

			if (itemIndex < 0) return;

			addLog(formatLogDownloadError($queue[itemIndex].item, errMsg));

			$queue.splice(itemIndex, 1);
			$queue = $queue;
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
				<li class:is-active={$page.route.id === link[0]}>
					<a data-sveltekit-preload-data href={link[0]} tabindex="-1">
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

	// Fix an issue where the tab underline was not visible
	.tabs li > a {
		margin-bottom: 1px;
	}
</style>
