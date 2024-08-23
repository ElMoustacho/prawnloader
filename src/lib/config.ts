import { invoke } from '$lib/tauri-wrapper';
import type { Config } from '$models/Config';
import { writable, type Writable } from 'svelte/store';

export type ConfigStore = Omit<Writable<Config>, 'update'>;

export function createConfig(config: Config): ConfigStore {
	const { subscribe, set } = writable(config);

	return {
		subscribe,
		set: (config: Config) => {
			invoke('update_config', { config }).then(set, err => console.error(err));
		},
	};
}
