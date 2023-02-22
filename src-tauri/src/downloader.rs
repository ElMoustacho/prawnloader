use crate::events::EventManager;
use crate::{music::Song, parser::Parser};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::FuturesOrdered;
use futures::StreamExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::async_runtime::block_on;
use threadpool::{Builder, ThreadPool};

type Queue = Vec<Box<dyn DownloadableSong>>;

// TOOD: Improve events to be more generic when needed
pub enum Event {
    AddToQueue(Vec<Song>),
    RemoveFromQueue(Vec<Song>),
    ClearQueue,
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
    pool: ThreadPool,
}

impl Downloader {
    pub fn new() -> Downloader {
        Downloader {
            parser: Parser::new(),
            queue: Vec::new(),
            event_manager: Arc::new(Mutex::new(EventManager::new())),
            pool: Builder::new().build(),
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
            if let Ok(mut downloadables) = downloadables {
                self.queue.append(&mut downloadables);
            } else {
                self.event_manager
                    .lock()
                    .unwrap()
                    .emit_event(Event::ParseError(urls[i].clone()))
            }

            i += 1;
        }

        self.event_manager
            .lock()
            .unwrap()
            .emit_event(Event::AddToQueue(self.get_queue_as_songs()));

        Ok(())
    }

    pub fn remove_from_queue(&mut self, index: usize) -> Result<(), ()> {
        if self.queue.len() <= index {
            return Err(());
        }

        self.queue.remove(index);

        self.event_manager
            .lock()
            .unwrap()
            .emit_event(Event::RemoveFromQueue(self.get_queue_as_songs()));

        Ok(())
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();

        self.event_manager
            .lock()
            .unwrap()
            .emit_event(Event::ClearQueue);
    }

    pub fn get_queue(&self) -> &Queue {
        self.queue.as_ref()
    }

    pub fn get_queue_as_songs(&self) -> Vec<Song> {
        let mut result = Vec::new();

        for downloadable in self.queue.iter() {
            result.push(downloadable.get_song().to_owned());
        }

        result
    }

    pub fn download(&mut self, index: usize, dest_folder: &Path) -> Result<(), ()> {
        if index >= self.queue.len() {
            return Err(());
        }

        let downloadable = self.queue.swap_remove(index);
        self._download(downloadable, dest_folder);

        Ok(())
    }

    pub fn download_queue(&mut self, dest_folder: &Path) {
        while let Some(downloadable) = self.queue.pop() {
            self.event_manager
                .lock()
                .unwrap()
                .emit_event(Event::RemoveFromQueue(self.get_queue_as_songs()));

            self._download(downloadable, dest_folder)
        }
    }

    fn _download(&self, downloadable: Box<dyn DownloadableSong>, dest_folder: &Path) {
        let dest_folder = dest_folder.to_path_buf();
        let event_manager = self.event_manager.clone();

        self.pool.execute(move || {
            if let Ok(result) = block_on(downloadable.download(dest_folder)) {
                event_manager
                    .lock()
                    .unwrap()
                    .emit_event(Event::DownloadComplete(result))
            };
        });
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}
