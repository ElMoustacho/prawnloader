import { createConfig } from '$lib/config';
import { invoke } from '$lib/tauri-wrapper';

export const prerender = true;
export const ssr = false;

/** @type {import('./$types').PageLoad} */
export async function load() {
	return { config: createConfig(await invoke('get_config', {})) };
}
