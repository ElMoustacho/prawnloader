import type { Song } from '$models/Song';

type DownloadStatus = 'Downloading' | 'Inactive';

export interface QueueSong {
	song: Song;
	download_state: DownloadStatus;
}
