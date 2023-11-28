import type { QueueSong, Song } from 'src/types/music';
import { writable, type Writable } from 'svelte/store';

export const queue: Writable<QueueSong[]> = writable([]);
export const logs: Writable<string[]> = writable([]);

export function formatLogSuccess(song: Song) {
	// TODO
	throw new Error('Not implemented');
}

export function formatLogAlbumNotFound(song: Song) {
	// TODO
	throw new Error('Not implemented');
}

export function formatLogSongNotFound(song: Song) {
	// TODO
	throw new Error('Not implemented');
}

export function formatLogDownloadError(song: Song) {
	// TODO
	throw new Error('Not implemented');
}
