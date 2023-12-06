import type { Song } from 'src/models/Song';

type DownloadStatus = 'Downloading' | 'Inactive';

export interface QueueSong {
	song: Song;
	download_state: DownloadStatus;
}
