<script lang="ts">
	import type { DownloadStatus, QueueSong } from 'src/types/music';

	export let queueSong: QueueSong;

	function getIconFromDownloadStatus(status: DownloadStatus): string {
		switch (status) {
			case 'Downloading':
				return 'fa-download';
			case 'Inactive':
				return 'fa-pause';

			default:
				throw Error('Invalid download status.');
		}
	}
</script>

<div class="list-item p-2">
	<div class="list-item-image">
		<figure class="image is-32x32">
			<img src={queueSong.song.album.cover_small} alt="" />
		</figure>
	</div>

	<div class="list-item-content">
		<div class="list-item-title" title={queueSong.song.title}>
			{queueSong.song.title}
			<span class="icon" title={queueSong.download_state}>
				<i
					class="fas has-text-primary-dark {getIconFromDownloadStatus(
						queueSong.download_state,
					)}" />
			</span>
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
