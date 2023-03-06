import '@tauri-apps/api/event';
import type { QueueSong, Song } from './music';

module '@tauri-apps/api/event' {
	export interface Events {
		queue_update: QueueSong[];
		download_complete: string;
		download_started: Song;
		parse_error: string;
	}
}
