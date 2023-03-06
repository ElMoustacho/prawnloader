use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;
use tokio::task::JoinHandle;

use crate::music::Song;

use super::DownloadableSong;

pub(super) type Queue = Vec<QueueSong>;
pub(super) type SerializableQueue = Vec<SerializableQueueSong>;

pub struct QueueSong {
    pub(super) downloadable_song: Box<dyn DownloadableSong>,
    pub(super) downloaded: bool,
    pub(super) download_handle: Option<JoinHandle<Result<PathBuf>>>,
    pub(super) progress: i8,
}

impl QueueSong {
    pub fn new(downloadable_song: Box<dyn DownloadableSong>) -> Self {
        Self {
            downloadable_song,
            downloaded: false,
            download_handle: None,
            progress: 0,
        }
    }

    pub fn get_serializable(&self) -> SerializableQueueSong {
        SerializableQueueSong {
            song: self.downloadable_song.get_song().clone(),
            downloaded: self.downloaded,
            progress: self.progress,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SerializableQueueSong {
    pub(super) song: Song,
    pub(super) downloaded: bool,
    pub(super) progress: i8,
}
