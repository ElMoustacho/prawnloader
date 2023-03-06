import '@tauri-apps/api/event';
import type { QueueSong, Song } from './music';

module '@tauri-apps/api/event' {
	export interface Events {
		queue_update: QueueSong[];
		download_started: Song;
		download_complete: string;
	}
}
