// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Song } from "./models\\Song";

export type Event = { "type": "waiting", "payload": Song } | { "type": "start", "payload": Song } | { "type": "finish", "payload": Song } | { "type": "download_error", "payload": Song } | { "type": "add_to_queue", "payload": Song } | { "type": "remove_from_queue", "payload": Song };