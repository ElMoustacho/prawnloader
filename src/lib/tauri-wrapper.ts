import {
	listen,
	type TauriEvent,
	type EventCallback,
	type UnlistenFn,
} from '@tauri-apps/api/event';
import type { Song } from './ts/music';
import { invoke } from '@tauri-apps/api';

type Events = {
	waiting: Song;
	start: Song;
	finish: Song;
	download_error: Song;
	add_to_queue: Song;
	remove_from_queue: Song;
};

type EventReturn<C extends keyof Events> = Events[C];

function _listen<T extends keyof Events>(
	event: T,
	handler: EventCallback<EventReturn<T>>
): Promise<UnlistenFn>;
function _listen<T>(event: TauriEvent, handler: EventCallback<T>): Promise<UnlistenFn>;
function _listen<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn> {
	return listen(event, handler);
}

type NoParams = Record<string, never>;
type Command = keyof Commands;
type CommandArgs<C extends Command> = Commands[C][0];
type CommandReturn<C extends Command> = Commands[C][1];

export interface Commands {
	add_to_queue: [{ url: string }, void];
	request_download: [{ trackId: string }, void];
}

function _invoke<C extends Command>(cmd: C, args: CommandArgs<C>): Promise<CommandReturn<C>> {
	return invoke(cmd, args);
}

export { _listen as listen, _invoke as invoke };
