<script lang="ts">
	import type { QueueItem } from '$lib/music';
	import { queue } from '$lib/stores';
	import { invoke } from '$lib/tauri-wrapper';

	export let queueItem: QueueItem;

	$: downloading = queueItem.download_state === 'Downloading';
</script>

<div class="list-item p-2">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img src={queueItem.song.album.cover_url} alt="" />
		</figure>
	</div>

	<div class="list-item-content">
		<div class="list-item-title" title={queueItem.song.title}>
			<span>[<b>{queueItem.download_state}</b>]</span>
			<span>{queueItem.song.title}</span>
		</div>
		<div class="list-item-description">
			<div class="is-flex is-justify-content-space-between">
				<span title={queueItem.song.album.title}>{queueItem.song.album.title}</span>
				<span title={queueItem.song.artist} class="is-single-line has-text-black-bis"
					>{queueItem.song.artist}</span>
			</div>
		</div>
	</div>

	<div class="list-item-controls">
		<div class="buttons is-right">
			<button
				class="button"
				on:click={() => invoke('request_download', { song: queueItem.song })}
				disabled={downloading}>
				<span class="icon is-small">
					<i class="fas fa-download" />
				</span>
			</button>
			<button
				class="button is-danger"
				on:click={() =>
					queue.update(queue => queue.filter(x => x.song.id !== queueItem.song.id))}
				disabled={downloading}>
				<span class="icon is-small">
					<i class="fas fa-trash" />
				</span>
			</button>
		</div>
	</div>
</div>
