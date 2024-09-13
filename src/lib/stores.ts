import { writable, type Writable } from 'svelte/store';
import type { QueueItem } from './music';

export const queue: Writable<QueueItem[]> = writable([]);
