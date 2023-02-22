import type { Song } from 'src/types/music';
import { writable, type Writable } from 'svelte/store';

export const queue: Writable<Song[]> = writable([]);
