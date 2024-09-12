import type { QueueItem } from '$models/QueueItem';
import { writable, type Writable } from 'svelte/store';

export const queue: Writable<QueueItem[]> = writable([]);
