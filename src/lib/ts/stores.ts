import type { QueueSong } from 'src/types/music';
import { writable, type Writable } from 'svelte/store';

export const queue: Writable<QueueSong[]> = writable([]);
