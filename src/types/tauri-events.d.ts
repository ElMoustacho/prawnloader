module '@tauri-apps/api/event' {
	/** @ignore */
	interface WindowDef {
		label: string;
	}
	/** @ignore */
	declare global {
		interface Window {
			__TAURI_METADATA__: {
				__windows: WindowDef[];
				__currentWindow: WindowDef;
			};
		}
	}
	/**
	 * Create new webview windows and get a handle to existing ones.
	 *
	 * Windows are identified by a *label*  a unique identifier that can be used to reference it later.
	 * It may only contain alphanumeric characters `a-zA-Z` plus the following special characters `-`, `/`, `:` and `_`.
	 *
	 * @example
	 * ```typescript
	 * // loading embedded asset:
	 * const webview = new WebviewWindow('theUniqueLabel', {
	 *   url: 'path/to/page.html'
	 * });
	 * // alternatively, load a remote URL:
	 * const webview = new WebviewWindow('theUniqueLabel', {
	 *   url: 'https://github.com/tauri-apps/tauri'
	 * });
	 *
	 * webview.once('tauri://created', function () {
	 *  // webview window successfully created
	 * });
	 * webview.once('tauri://error', function (e) {
	 *  // an error happened creating the webview window
	 * });
	 *
	 * // emit an event to the backend
	 * await webview.emit("some event", "data");
	 * // listen to an event from the backend
	 * const unlisten = await webview.listen("event name", e => {});
	 * unlisten();
	 * ```
	 *
	 * @since 1.0.2
	 */

	interface Event<T> {
		/** Event name */
		event: EventName;
		/** The label of the window that emitted this event. */
		windowLabel: string;
		/** Event identifier used to unlisten */
		id: number;
		/** Event payload */
		payload: T;
	}
	declare type EventCallback<T> = (event: Event<T>) => void;
	declare type UnlistenFn = () => void;

	declare type EventName = TauriEvent | string;
	/**
	 * @since 1.1.0
	 */
	declare enum TauriEvent {
		WINDOW_RESIZED = 'tauri://resize',
		WINDOW_MOVED = 'tauri://move',
		WINDOW_CLOSE_REQUESTED = 'tauri://close-requested',
		WINDOW_CREATED = 'tauri://window-created',
		WINDOW_DESTROYED = 'tauri://destroyed',
		WINDOW_FOCUS = 'tauri://focus',
		WINDOW_BLUR = 'tauri://blur',
		WINDOW_SCALE_FACTOR_CHANGED = 'tauri://scale-change',
		WINDOW_THEME_CHANGED = 'tauri://theme-changed',
		WINDOW_FILE_DROP = 'tauri://file-drop',
		WINDOW_FILE_DROP_HOVER = 'tauri://file-drop-hover',
		WINDOW_FILE_DROP_CANCELLED = 'tauri://file-drop-cancelled',
		MENU = 'tauri://menu',
		CHECK_UPDATE = 'tauri://update',
		UPDATE_AVAILABLE = 'tauri://update-available',
		INSTALL_UPDATE = 'tauri://update-install',
		STATUS_UPDATE = 'tauri://update-status',
		DOWNLOAD_PROGRESS = 'tauri://update-download-progress',
	}

	export interface Events {
		'tauri://resize': object;
		'tauri://move': object;
		'tauri://close-requested': object;
		'tauri://window-created': object;
		'tauri://destroyed': object;
		'tauri://focus': object;
		'tauri://blur': object;
		'tauri://scale-change': object;
		'tauri://theme-changed': object;
		'tauri://file-drop': object;
		'tauri://file-drop-hover': object;
		'tauri://file-drop-cancelled': object;
		'tauri://menu': object;
		'tauri://update': object;
		'tauri://update-available': object;
		'tauri://update-install': object;
		'tauri://update-status': object;
		'tauri://update-download-progress': object;
	}

	type _Event = keyof Events;
	type EventReturn<C extends _Event> = Events[C];

	/**
	 * Listen to an event from the backend.
	 *
	 * @example
	 * ```typescript
	 * import { listen } from '@tauri-apps/api/event';
	 * const unlisten = await listen<string>('error', (event) => {
	 *   console.log(`Got error in window ${event.windowLabel}, payload: ${event.payload}`);
	 * });
	 *
	 * // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
	 * unlisten();
	 * ```
	 *
	 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
	 * @param handler Event handler callback.
	 * @returns A promise resolving to a function to unlisten to the event.
	 * Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
	 *
	 * @since 1.0.0
	 */
	declare function listen<T extends _Event>(
		event: T,
		handler: EventCallback<EventReturn<T>>
	): Promise<UnlistenFn>;
	/**
	 * Listen to an one-off event from the backend.
	 *
	 * @example
	 * ```typescript
	 * import { once } from '@tauri-apps/api/event';
	 * interface LoadedPayload {
	 *   loggedIn: boolean,
	 *   token: string
	 * }
	 * const unlisten = await once<LoadedPayload>('loaded', (event) => {
	 *   console.log(`App is loaded, loggedIn: ${event.payload.loggedIn}, token: ${event.payload.token}`);
	 * });
	 *
	 * // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
	 * unlisten();
	 * ```
	 *
	 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
	 * @returns A promise resolving to a function to unlisten to the event.
	 * Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
	 *
	 * @since 1.0.0
	 */
	declare function once<T>(
		event: EventName,
		handler: EventCallback<T>
	): Promise<UnlistenFn>;
	/**
	 * Emits an event to the backend.
	 * @example
	 * ```typescript
	 * import { emit } from '@tauri-apps/api/event';
	 * await emit('frontend-loaded', { loggedIn: true, token: 'authToken' });
	 * ```
	 *
	 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
	 *
	 * @since 1.0.0
	 */
	declare function emit(event: string, payload?: unknown): Promise<void>;

	export {
		// CloseRequestedEvent as C,
		Event,
		// FileDropEvent as F,
		// LogicalSize as L,
		// Monitor as M,
		// PhysicalSize as P,
		// ScaleFactorChanged as S,
		// Theme as T,
		UnlistenFn,
		// WebviewWindow as W,
		// WebviewWindowHandle as a,
		// WindowManager as b,
		// getAll as c,
		// appWindow as d,
		// event as e,
		// LogicalPosition as f,
		// getCurrent as g,
		// PhysicalPosition as h,
		// UserAttentionType as i,
		// currentMonitor as j,
		// availableMonitors as k,
		// TitleBarStyle as l,
		// WindowOptions as m,
		// CursorIcon as n,
		// WindowLabel as o,
		// primaryMonitor as p,
		EventCallback,
		listen,
		once,
		emit,
		EventName,
		TauriEvent,
		// window as w,
	};
}
