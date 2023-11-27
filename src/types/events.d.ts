import '@tauri-apps/api/event';
import type { Song } from './music';

module '@tauri-apps/api/event' {
	export interface Events {
		Queue: Song;
		Start: Song;
		Finish: Song;
		DownloadError: Song;
		SongNotFoundError: number;
		AlbumNotFoundError: number;
	}
}
