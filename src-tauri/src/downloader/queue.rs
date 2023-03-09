use anyhow::Result;
use crossbeam::channel::Sender;
use futures::Future;
use serde::Serialize;
use std::{path::PathBuf, pin::Pin, sync::atomic::AtomicUsize};
use tokio::task::JoinHandle;

use crate::music::Song;

use super::Event;

pub type Queue = Vec<QueueSong>;
pub type SerializableQueue = Vec<SerializableQueueSong>;
pub type DownloadFun =
    fn(&str, PathBuf) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>>;
pub type Id = usize;

pub struct QueueSong {
    pub(super) id: Id,
    pub(super) song: Song,
    pub(super) progress: i8,
    pub download_handle: Option<JoinHandle<Result<PathBuf>>>,
    pub download_fun: DownloadFun,
    pub url: String,
}

impl QueueSong {
    pub fn new(song: Song, url: String, download_fun: DownloadFun) -> Self {
        Self {
            id: generate_id(),
            song,
            progress: 0,
            download_handle: None,
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
            id: self.id,
            song: self.song.clone(),
            progress: self.progress,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SerializableQueueSong {
    pub id: Id,
    pub song: Song,
    pub progress: i8,
}

fn generate_id() -> Id {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);

    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
