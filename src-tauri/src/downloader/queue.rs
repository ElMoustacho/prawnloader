use anyhow::Result;
use futures::Future;
use serde::Serialize;
use std::{path::PathBuf, pin::Pin};
use tokio::task::JoinHandle;

use crate::music::Song;

pub type Queue = Vec<QueueSong>;
pub type SerializableQueue = Vec<SerializableQueueSong>;
pub type DownloadFun =
    fn(&str, PathBuf) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>>;

pub struct QueueSong {
    pub(super) song: Song,
    pub(super) downloaded: bool,
    pub(super) download_handle: Option<JoinHandle<Result<PathBuf>>>,
    pub(super) progress: i8,
    pub(super) download_fun: DownloadFun,
    pub(super) url: String,
}

impl QueueSong {
    pub fn new(song: Song, url: String, download_fun: DownloadFun) -> Self {
        Self {
            song,
            downloaded: false,
            download_handle: None,
            progress: 0,
            download_fun,
            url,
        }
    }

    pub async fn download(&self, dest_file: PathBuf) -> Result<PathBuf> {
        (self.download_fun)(&self.url, dest_file).await
    }

    pub fn get_serializable(&self) -> SerializableQueueSong {
        SerializableQueueSong {
            song: self.song.clone(),
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
