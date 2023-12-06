use serde::Serialize;
use ts_rs::TS;

use crate::{downloader::ProgressEvent, models::music::Song};

#[derive(Clone, TS, Serialize, strum_macros::Display)]
#[ts(export, export_to = "../src/models/")]
#[serde(rename_all = "snake_case", tag = "type", content = "payload")]
#[strum(serialize_all = "snake_case")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_snake_case() {
        let event = Event::Waiting(Song::default());
        assert_eq!(event.to_string()[..], *"waiting");

        let event = Event::RemoveFromQueue(Song::default());
        assert_eq!(event.to_string()[..], *"remove_from_queue");
    }
}
