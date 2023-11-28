<script lang="ts">
	import type { DownloadStatus, QueueSong } from 'src/types/music';

	export let queueSong: QueueSong;
</script>

<div class="list-item p-2">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img src={queueSong.song.album.cover_small} alt="" />
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
				<span title={queueSong.song.artist.name} class="is-single-line has-text-black-bis"
					>{queueSong.song.artist.name}</span>
			</div>
		</div>
	</div>
</div>
