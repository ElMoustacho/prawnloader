import {
	listen,
	type TauriEvent,
	type EventCallback,
	type UnlistenFn,
} from '@tauri-apps/api/event';
import type { Event } from '$models/Event';
import { invoke } from '@tauri-apps/api';
import type { Song } from '$models/Song';

type EventMap = {
	[K in Event['type']]: Extract<Event, { type: K }>['payload'];
};

type EventName = keyof EventMap;
type EventPayload<T extends EventName> = EventMap[T];

function _listen<T extends EventName>(
	event: T,
	handler: EventCallback<EventPayload<T>>
): Promise<UnlistenFn>;
function _listen<T>(event: TauriEvent, handler: EventCallback<T>): Promise<UnlistenFn>;
function _listen<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn> {
	return listen(event, handler);
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
type NoParams = Record<string, never>;
type Command = keyof Commands;
type CommandArgs<C extends Command> = Commands[C][0];
type CommandReturn<C extends Command> = Commands[C][1];

export interface Commands {
	get_songs: [{ url: string }, Song[]];
	request_download: [{ song: Song }, void];
}

function _invoke<C extends Command>(cmd: C, args: CommandArgs<C>): Promise<CommandReturn<C>> {
	return invoke(cmd, args);
}

export { _listen as listen, _invoke as invoke };
