import { invoke } from '$lib/tauri-wrapper';
import type { Config } from '$models/Config';
import { writable } from 'svelte/store';

export function createConfig(config: Config) {
	const { subscribe, set } = writable(config);

	return {
		subscribe,
		set: (config: Config) => {
			invoke('update_config', { config }).then(set, err => console.error(err));
		},
	};
}
