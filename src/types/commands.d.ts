import '@tauri-apps/api';

module '@tauri-apps/api' {
	type NoParams = Record<string, never>;

	export interface Commands {
		add_to_queue: [{ url: string }, void];
		remove_from_queue: [{ id: number }, void];
		download: [{ index: number }, void];
		clear_queue: [NoParams, void];
	}
}
