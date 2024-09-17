<script lang="ts">
	import type { QueueItem } from '$lib/music';
	import { queue } from '$lib/stores';
	import { invoke } from '$lib/tauri-wrapper';
	import type { Song } from '$models/Song';

	export let queueItem: QueueItem;

	if (queueItem.item.type !== 'YoutubeVideo')
		throw new Error('Item should be of type YoutubeVideo.');

	let video: Song = queueItem.item;

	$: downloading = queueItem.download_status === 'Downloading';
</script>

<div class="list-item p-2">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img src={video.album.cover_url} alt="" />
		</figure>
	</div>

	<div class="list-item-content">
		<div class="list-item-title" title={video.title}>
			<span>[<b>{queueItem.download_status}</b>]</span>
			<span>{video.title}</span>
		</div>
		<div class="list-item-description">
			<div class="is-flex is-justify-content-space-between">
				<span title={video.album.title}>{video.album.title}</span>
				<span title={video.artist} class="is-single-line has-text-black-bis"
					>{video.artist}</span>
			</div>
		</div>
	</div>

	<div class="list-item-controls">
		<div class="buttons is-right">
			<button
				class="button"
				on:click={() => invoke('request_download', { request: queueItem })}
				disabled={downloading}>
				<span class="icon is-small">
					<i class="fas fa-download" />
				</span>
			</button>
			<button
				class="button is-danger"
				on:click={() =>
					queue.update(queue => queue.filter(x => x.request_id === queueItem.request_id))}
				disabled={downloading}>
				<span class="icon is-small">
					<i class="fas fa-trash" />
				</span>
			</button>
		</div>
	</div>
</div>
