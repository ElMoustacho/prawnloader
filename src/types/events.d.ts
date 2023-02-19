import '@tauri-apps/api/event';
import type { Song } from './music';

module '@tauri-apps/api/event' {
	export interface Events {
		queue_update: Song[];
		download_complete: string;
		download_started: Song;
	}
}
