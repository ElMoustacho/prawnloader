import '@tauri-apps/api';

module '@tauri-apps/api' {
	type NoParams = Record<string, never>;

	export interface Commands {
		add_to_queue: [{ urls: string[] }, void];
		remove_from_queue: [{ id: number }, void];
		download: [{ id: number }, void];
		download_queue: [NoParams, void];
		clear_queue: [NoParams, void];
	}
}
