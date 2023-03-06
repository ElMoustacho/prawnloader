<script lang="ts">
	import type { QueueSong } from 'src/types/music';
	import bufferToImg from '$lib/ts/bufferToImg';

	export let queueSong: QueueSong;
</script>

<div class="list-item">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img
				src={queueSong.song.album.cover
					? bufferToImg(queueSong.song.album.cover)
					: 'https://cdns-images.dzcdn.net/images/cover/2b944b29fc4ab95482da6e968ec03586/500x500-000000-80-0-0.jpg'}
				alt="" />
		</figure>
	</div>

	<div class="list-item-content">
		<div class="list-item-title" title={queueSong.song.title}>{queueSong.song.title}</div>
		<div class="list-item-description">
			<div class="is-flex is-justify-content-space-between">
				<span title={queueSong.song.album.name}>{queueSong.song.album.name}</span>
				<span title={queueSong.song.album.artist} class="is-single-line has-text-black-bis"
					>{queueSong.song.album.artist}</span>
			</div>
		</div>
	</div>

	<div class="list-item-controls">
		<div class="buttons is-right">
			<!-- TODO: Add actions to buttons -->
			<button class="button">
				<span class="icon is-small">
					<i class="fas fa-download" />
				</span>
			</button>
			<button class="button is-danger">
				<span class="icon is-small">
					<i class="fas fa-trash" />
				</span>
			</button>
		</div>
	</div>
</div>
