import type { Item } from '$models/Item';
import { writable, type Writable } from 'svelte/store';

export class Log {
	success: boolean;
	content: string;
	timestamp: Date;

	constructor(success: boolean, content: string) {
		this.success = success;
		this.content = content;
		this.timestamp = new Date();
	}
}

export const logs: Writable<Log[]> = writable([]);

export function addLog(log: Log) {
	logs.update(logs => {
		logs.push(log);
		return logs;
	});
}

export function clearLogs() {
	logs.set([]);
}

export function formatLogSuccess(item: Item): Log {
	return new Log(true, `Downloaded ${item.artist} - ${item.title}`);
}

export function formatLogAlbumNotFound(albumId: number): Log {
	return new Log(false, `Album ${albumId} not found`);
}

export function formatLogSongNotFound(songId: number): Log {
	return new Log(false, `Song ${songId} not found`);
}

export function formatLogDownloadError(item: Item, message: string): Log {
	return new Log(false, `Error while downloading ${item.artist} - ${item.title} (${message}).`);
}
