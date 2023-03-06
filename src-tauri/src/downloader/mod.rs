use crate::events::EventManager;
use crate::{music::Song, parser::Parser};
use anyhow::{Context, Result};
use async_trait::async_trait;
use queue::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

mod queue;

pub enum Event {
    UpdateQueue(SerializableQueue),
    DownloadStarted(Song),
    DownloadComplete(PathBuf),
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

    pub async fn add_to_queue(&mut self, url: String) -> Result<()> {
        let downloadables = self.parser.parse_url(&url.into()).await?;

        for downloadable in downloadables {
            self.queue.push(QueueSong::new(downloadable));
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

    pub async fn download(&mut self, index: usize, dest_folder: &Path) {
        let queue_item = self.queue.get_mut(index).context("Song not found.");
        let queue_item = if let Ok(x) = queue_item { x } else { return };

        if queue_item.downloaded {
            return;
        };

        let dest_folder = dest_folder.to_owned();
        let event_manager = Arc::clone(&self.event_manager);

        queue_item.download_handle = {
            Some(tokio::spawn(async move {
                let result = queue_item
                    .downloadable_song
                    .download(dest_folder.clone())
                    .await;

                event_manager
                    .lock()
                    .unwrap()
                    .emit_event(Event::DownloadComplete(dest_folder));

                result
            }))
        };
    }

    fn emit_queue_update(&self) {
        self.event_manager
            .lock()
            .unwrap()
            .emit_event(Event::UpdateQueue(
                self.queue
                    .iter()
                    .map(|song| song.get_serializable())
                    .collect(),
            ));
    }
}
