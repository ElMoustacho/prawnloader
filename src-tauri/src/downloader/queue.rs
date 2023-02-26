use anyhow::{bail, Result};
use std::{path::PathBuf, sync::Arc};
use tokio::{spawn, task::JoinHandle};

use super::DownloadableSong;

pub(super) type Queue = Vec<QueueSong>;

pub(super) struct QueueSong {
    pub downloadable_song: Arc<Box<dyn DownloadableSong>>,
    downloaded: bool,
    download_handle: Option<JoinHandle<Result<PathBuf>>>,
}

impl QueueSong {
    pub fn build(downloadable_song: Box<dyn DownloadableSong>) -> Self {
        Self {
            downloadable_song: Arc::new(downloadable_song),
            downloaded: false,
            download_handle: None,
        }
    }

    pub fn build_from_vec(mut downloadable_songs: Vec<Box<dyn DownloadableSong>>) -> Vec<Self> {
        let mut result = Vec::new();

        while let Some(downloadable_song) = downloadable_songs.pop() {
            result.push(Self::build(downloadable_song));
        }

        result
    }

    pub fn is_downloaded(&self) -> bool {
        self.downloaded
    }

    pub fn download(&mut self, dest_folder: PathBuf) -> Result<()> {
        if self.downloaded {
            bail!("Download already started.");
        };

        let downloadable_song = Arc::clone(&self.downloadable_song);

        self.download_handle = Some(spawn(async move {
            downloadable_song.download(dest_folder).await
        }));

        Ok(())
    }
}
