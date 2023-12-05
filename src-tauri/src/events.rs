use serde::Serialize;
use ts_rs::TS;

use crate::{downloader::ProgressEvent, models::music::Song};

#[derive(TS, Serialize)]
#[ts(export, export_to = "../src/events.ts")]
pub enum Event {
    ProgressEvent(ProgressEvent),
    AddToQueue(Song),
    RemoveFromQueue(Song),
}
