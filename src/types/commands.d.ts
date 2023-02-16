import '@tauri-apps/api';

module '@tauri-apps/api' {
	export interface Commands {
		add_to_queue: [{ url: string }, void];
		remove_from_queue: [{ id: number }, void];
		download_queue: [Record<string, never>, void];
	}
}
