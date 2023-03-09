use anyhow::Result;
use crossbeam::channel::Sender;
use futures::Future;
use serde::Serialize;
use std::{path::PathBuf, pin::Pin};
use tokio::task::JoinHandle;

use crate::music::Song;

use super::Event;

pub type Queue = Vec<QueueSong>;
pub type SerializableQueue = Vec<SerializableQueueSong>;
pub type DownloadFun =
    fn(&str, PathBuf) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>>;

pub struct QueueSong {
    pub(super) song: Song,
    pub(super) download_handle: Option<JoinHandle<Result<PathBuf>>>,
    pub(super) progress: i8,
    pub(super) download_fun: DownloadFun,
    pub(super) url: String,
}

impl QueueSong {
    pub fn new(song: Song, url: String, download_fun: DownloadFun) -> Self {
        Self {
            song,
            download_handle: None,
            progress: 0,
            download_fun,
            url,
        }
    }

    pub fn start_download(&mut self, dest_folder: PathBuf, event_sender: Sender<Event>) {
        let dl_fun = self.download_fun;
        let url = self.url.clone();

        self.download_handle = Some(tokio::spawn(async move {
            let result = dl_fun(&url, dest_folder.clone()).await;

            event_sender
                .send(Event::DownloadComplete(dest_folder))
                .expect("Channel should be connected.");

            result
        }));
    }

    pub fn get_serializable(&self) -> SerializableQueueSong {
        SerializableQueueSong {
            song: self.song.clone(),
            progress: self.progress,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SerializableQueueSong {
    pub(super) song: Song,
    pub(super) progress: i8,
}
