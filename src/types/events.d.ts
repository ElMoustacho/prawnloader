import '@tauri-apps/api/event';
import type { Song } from './music';

module '@tauri-apps/api/event' {
	export interface Events {
		waiting: Song;
		start: Song;
		finish: Song;
		download_error: Song;
		add_to_queue: Song;
		remove_from_queue: Song;
	}
}
