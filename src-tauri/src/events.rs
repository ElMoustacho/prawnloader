use crate::{downloader::ProgressEvent, models::music::Song};

pub enum Event {
    ProgressEvent(ProgressEvent),
    AddToQueue(Song),
    RemoveFromQueue(Song),
}
