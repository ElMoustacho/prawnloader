import '@tauri-apps/api';

module '@tauri-apps/api' {
	type NoParams = Record<string, never>;

	export interface Commands {
		request_download: [{ url: string }, void];
	}
}
