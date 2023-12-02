use deezer::models::Track;
use serde::Serialize;

use crate::models::music::Song;

#[derive(Debug, Clone, Serialize, strum_macros::Display)]
pub enum ProgressEvent {
    Waiting(Track),
    Start(Track),
    Finish(Track),
    DownloadError(Track),
}

pub enum Event {
    ProgressEvent(ProgressEvent),
    AddToQueue(Song),
    RemoveFromQueue(Song),
}
