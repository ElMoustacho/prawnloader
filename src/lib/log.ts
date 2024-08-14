import type { Song } from '$models/Song';
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

export function formatLogSuccess(song: Song): Log {
	return new Log(true, `Downloaded ${song.artist} - ${song.title}`);
}

export function formatLogAlbumNotFound(albumId: number): Log {
	return new Log(false, `Album ${albumId} not found`);
}

export function formatLogSongNotFound(songId: number): Log {
	return new Log(false, `Song ${songId} not found`);
}

export function formatLogDownloadError(song: Song, message: string): Log {
	return new Log(false, `Error while downloading ${song.artist} - ${song.title} (${message}).`);
}
