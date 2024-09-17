<script lang="ts">
	import type { QueueItem } from '$lib/music';
	import { queue } from '$lib/stores';
	import { invoke } from '$lib/tauri-wrapper';
	import type { Album } from '$models/Album';

	export let queueItem: QueueItem;

	if (queueItem.item.type !== 'DeezerAlbum')
		throw new Error('Item should be of type DeezerAlbum.');

	const album: Album = queueItem.item;

	$: downloading = queueItem.download_status === 'Downloading';
</script>

<div class="list-item p-2">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img src={album.cover_url} alt="" />
		</figure>
	</div>

	<div class="list-item-content">
		<div class="list-item-title" title={album.title}>
			<span>[<b>{queueItem.download_status}</b>]</span>
			<span>{album.title}</span>
		</div>
		<div class="list-item-description">
			<div class="is-flex is-justify-content-space-between">
				<div>
					<i class="fa-brands fa-deezer"></i>
					<span><b>Album</b> | {album.songs.length} tracks</span>
				</div>
				<span title={album.artist} class="is-single-line has-text-black-bis"
					>{album.artist}</span>
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
