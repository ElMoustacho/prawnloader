use serde::Serialize;
use ts_rs::TS;

use crate::{downloaders::ProgressEvent, models::music::Song};

#[derive(Clone, TS, Serialize, strum_macros::Display)]
#[ts(export, export_to = "../src/models/")]
#[serde(rename_all = "snake_case", tag = "type", content = "payload")]
#[strum(serialize_all = "snake_case")]
pub enum Event {
    Waiting(Song),
    Start(Song),
    Finish(Song),
    DownloadError(Song),
    RemoveFromQueue(Song),
}

impl From<ProgressEvent> for Event {
    fn from(event: ProgressEvent) -> Self {
        match event {
            ProgressEvent::Waiting(song) => Self::Waiting(song),
            ProgressEvent::Start(song) => Self::Start(song),
            ProgressEvent::Finish(song) => Self::Finish(song),
            ProgressEvent::DownloadError(song) => Self::DownloadError(song),
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
