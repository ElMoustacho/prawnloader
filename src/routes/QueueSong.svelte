<script lang="ts">
	import type { QueueSong } from 'src/types/music';

	export let queueSong: QueueSong;
</script>

<div class="list-item">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img src={queueSong.song.album.cover} alt="" />
		</figure>
	</div>

	<div class="list-item-content">
		<div class="list-item-title" title={queueSong.song.title}>{queueSong.song.title}</div>
		<div class="list-item-description">
			<div class="is-flex is-justify-content-space-between">
				<span title={queueSong.song.album.title}>{queueSong.song.album.title}</span>
				<span title={queueSong.song.artist.name} class="is-single-line has-text-black-bis"
					>{queueSong.song.artist.name}</span>
			</div>
		</div>
	</div>
</div>
