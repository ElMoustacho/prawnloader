use serde::Serialize;
use ts_rs::TS;

use crate::{downloader::ProgressEvent, models::music::Song};

#[derive(TS, Serialize)]
#[ts(export, export_to = "../src/events.ts", rename_all = "snake_case")]
pub enum Event {
    Waiting(Song),
    Start(Song),
    Finish(Song),
    DownloadError(Song),
    AddToQueue(Song),
    RemoveFromQueue(Song),
}

impl From<ProgressEvent> for Event {
    fn from(event: ProgressEvent) -> Self {
        match event {
            ProgressEvent::Waiting(track) => Self::Waiting(Song::from(track)),
            ProgressEvent::Start(track) => Self::Start(Song::from(track)),
            ProgressEvent::Finish(track) => Self::Finish(Song::from(track)),
            ProgressEvent::DownloadError(track) => Self::DownloadError(Song::from(track)),
        }
    }
}
