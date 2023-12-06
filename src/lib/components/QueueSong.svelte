<script lang="ts">
	import { queue } from '$lib/stores';
	import { invoke } from '@tauri-apps/api';
	import type { QueueSong } from '../music';

	export let queueSong: QueueSong;
</script>

<div class="list-item p-2">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img src={queueSong.song.album.cover_url} alt="" />
		</figure>
	</div>

	<div class="list-item-content">
		<div class="list-item-title" title={queueSong.song.title}>
			<span>[<b>{queueSong.download_state}</b>]</span>
			<span>{queueSong.song.title}</span>
		</div>
		<div class="list-item-description">
			<div class="is-flex is-justify-content-space-between">
				<span title={queueSong.song.album.title}>{queueSong.song.album.title}</span>
				<span title={queueSong.song.artist} class="is-single-line has-text-black-bis"
					>{queueSong.song.artist}</span>
			</div>
		</div>
	</div>

	<div class="list-item-controls">
		<div class="buttons is-right">
			<button
				class="button"
				on:click={() => invoke('request_download', { trackId: queueSong.song.id })}>
				<span class="icon is-small">
					<i class="fas fa-download" />
				</span>
			</button>
			<button
				class="button is-danger"
				on:click={() =>
					queue.update(queue => queue.filter(x => x.song.id !== queueSong.song.id))}>
				<span class="icon is-small">
					<i class="fas fa-trash" />
				</span>
			</button>
		</div>
	</div>
</div>
