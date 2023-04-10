use anyhow::Result;
use futures::Future;
use serde::{ser::SerializeStruct, Serialize};
use std::{path::PathBuf, pin::Pin, sync::atomic::AtomicUsize};

use crate::music::Song;

pub type Queue = Vec<QueueSong>;
pub type DownloadFun =
    fn(&str, PathBuf) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>>;
pub type Id = usize;

#[derive(Clone, Copy, Serialize)]
pub enum DownloadState {
    Downloading,
    Stopped,
    Finished,
}

pub struct QueueSong {
    pub(super) id: Id,
    pub(super) song: Song,
    pub(super) progress: i8,
    pub(super) download_state: DownloadState,
    pub(super) download_fun: DownloadFun,
    pub(super) url: String,
}

impl QueueSong {
    pub fn new(song: Song, url: String, download_fun: DownloadFun) -> Self {
        Self {
            id: generate_id(),
            song,
            progress: 0,
            download_state: DownloadState::Stopped,
            download_fun,
            url,
        }
    }
}

impl Clone for QueueSong {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            song: self.song.clone(),
            progress: self.progress.clone(),
            download_state: self.download_state.clone(),
            download_fun: self.download_fun.clone(),
            url: self.url.clone(),
        }
    }
}

impl Serialize for QueueSong {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("QueueSong", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("song", &self.song)?;
        state.serialize_field("progress", &self.progress)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("download_state", &self.download_state)?;
        state.end()
    }
}

fn generate_id() -> Id {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);

    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
