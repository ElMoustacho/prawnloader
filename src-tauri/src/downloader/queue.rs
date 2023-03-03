use anyhow::Result;
use std::{path::PathBuf, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};

use super::DownloadableSong;

pub(super) type Queue = Vec<Arc<Mutex<QueueSong>>>;

pub(super) struct QueueSong {
    pub downloadable_song: Box<dyn DownloadableSong>,
    pub(super) downloaded: bool,
    pub(super) download_handle: Option<JoinHandle<Result<PathBuf>>>,
}

impl QueueSong {
    pub fn build(downloadable_song: Box<dyn DownloadableSong>) -> Self {
        Self {
            downloadable_song,
            downloaded: false,
            download_handle: None,
        }
    }

    pub fn build_from_vec(mut downloadable_songs: Vec<Box<dyn DownloadableSong>>) -> Queue {
        let mut result = Vec::new();

        while let Some(downloadable_song) = downloadable_songs.pop() {
            result.push(Arc::new(Mutex::new(Self::build(downloadable_song))));
        }

        result
    }
}
