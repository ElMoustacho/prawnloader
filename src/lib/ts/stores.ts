import type { QueueSong } from './music';
import { writable, type Writable } from 'svelte/store';

export const queue: Writable<QueueSong[]> = writable([]);
