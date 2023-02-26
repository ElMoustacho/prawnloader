use crate::events::EventManager;
use crate::{music::Song, parser::Parser};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::stream::FuturesOrdered;
use futures::StreamExt;
use queue::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

mod queue;

pub enum Event {
    UpdateQueue(Vec<Song>),
    DownloadStarted(Song),
    DownloadComplete(PathBuf),
    ParseError(String),
}

#[async_trait]
pub trait DownloadableSong: Send + Sync {
    async fn download(&self, dest_folder: PathBuf) -> Result<PathBuf>;
    fn get_song(&self) -> &Song;
}

pub struct Downloader {
    pub event_manager: Arc<Mutex<EventManager<Event>>>,
    pub parser: Parser,
    queue: Queue,
}

impl Downloader {
    pub fn new() -> Downloader {
        Downloader {
            event_manager: Arc::new(Mutex::new(EventManager::new())),
            parser: Parser::new(),
            queue: Vec::new(),
        }
    }

    pub async fn add_to_queue(&mut self, urls: Vec<String>) -> Result<(), ()> {
        let mut i = 0;
        let mut futures = FuturesOrdered::new();

        for url in urls.clone() {
            futures.push_back(async { self.parser.parse_url(&url.into()).await });
        }

        let downloadables_vec: Vec<_> = futures.collect().await;

        for downloadables in downloadables_vec {
            if let Ok(downloadables) = downloadables {
                self.queue
                    .append(&mut QueueSong::build_from_vec(downloadables));
            } else {
                self.event_manager
                    .lock()
                    .unwrap()
                    .emit_event(Event::ParseError(urls[i].clone()))
            }

            i += 1;
        }

        self.emit_queue_update();

        Ok(())
    }

    pub fn remove_from_queue(&mut self, index: usize) -> Result<(), ()> {
        if self.queue.len() <= index {
            return Err(());
        }

        self.queue.remove(index);

        self.emit_queue_update();

        Ok(())
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();

        self.emit_queue_update();
    }

    pub fn get_queue_as_songs(&self) -> Vec<Song> {
        let mut result = Vec::new();

        for downloadable in self.queue.iter() {
            result.push(downloadable.downloadable_song.get_song().to_owned());
        }

        result
    }

    pub fn download(&mut self, index: usize, dest_folder: &Path) -> Result<()> {
        let downloadable = self.queue.get_mut(index).context("Song not found.")?;

        downloadable.download(dest_folder.to_owned())
    }

    pub fn download_queue(&mut self, dest_folder: &Path) {
        for i in 0..self.queue.len() {
            _ = self.download(i, dest_folder);
        }
    }

    fn emit_queue_update(&self) {
        self.event_manager
            .lock()
            .unwrap()
            .emit_event(Event::UpdateQueue(self.get_queue_as_songs()));
    }
}
