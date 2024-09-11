use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::downloaders::ProgressEvent;

#[derive(Clone, TS, Serialize, strum_macros::Display)]
#[ts(export)]
#[serde(rename_all = "snake_case", tag = "type", content = "payload")]
#[strum(serialize_all = "snake_case")]
pub enum Event {
    Waiting(#[ts(type = "string")] Uuid),
    Start(#[ts(type = "string")] Uuid),
    Finish(#[ts(type = "string")] Uuid),
    DownloadError(#[ts(type = "string")] Uuid, String),
    RemoveFromQueue(#[ts(type = "string")] Uuid),
}

impl From<ProgressEvent> for Event {
    fn from(event: ProgressEvent) -> Self {
        match event {
            ProgressEvent::Waiting(id) => Self::Waiting(id),
            ProgressEvent::Start(id) => Self::Start(id),
            ProgressEvent::Finish(id) => Self::Finish(id),
            ProgressEvent::DownloadError(id, err_msg) => Self::DownloadError(id, err_msg),
        }
    }
}
