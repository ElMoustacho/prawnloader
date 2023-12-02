import '@tauri-apps/api';

module '@tauri-apps/api' {
	type NoParams = Record<string, never>;

	export interface Commands {
		add_to_queue: [{ url: string }, void];
		request_download: [{ trackId: string }, void];
	}
}
