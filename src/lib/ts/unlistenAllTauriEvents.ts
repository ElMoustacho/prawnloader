import type { UnlistenFn } from '@tauri-apps/api/event';

/**
 * Unlistens all tauri event listeners passed after all callback funtions
 * have been resolved.
 */
export default function unlistenAllTauriEvents(fns: Promise<UnlistenFn>[]) {
	fns.forEach(async fn => (await fn)());
}
