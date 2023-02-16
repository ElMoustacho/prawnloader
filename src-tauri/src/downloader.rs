use crate::{music::Song, parser::Parser};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tauri::async_runtime::block_on;
use threadpool::{Builder, ThreadPool};

type Queue = Vec<Box<dyn DownloadableSong>>;

#[derive(PartialEq)]
pub enum Event {
    AddToQueue,
    RemoveFromQueue,
}

#[async_trait]
pub trait DownloadableSong: Send + Sync {
    async fn download(&self, dest_folder: PathBuf) -> Result<PathBuf, ()>;
    fn get_song(&self) -> &Song;
}

struct EventListener {
    event: Event,
    callback: Box<dyn Fn(&Downloader) -> () + Send>,
}

pub struct Downloader {
    pub parser: Parser,
    queue: Queue,
    event_listeners: Vec<EventListener>,
    pool: ThreadPool,
}

impl Downloader {
    pub fn new() -> Downloader {
        Downloader {
            queue: Vec::new(),
            event_listeners: Vec::new(),
            parser: Parser::new(),
            pool: Builder::new().build(),
        }
    }

    pub async fn add_to_queue(&mut self, url: impl Into<String>) -> Result<(), ()> {
        let mut downloadables = self.parser.parse_url(&url.into()).await?;

        self.queue.append(&mut downloadables);

        self.emit_event(Event::AddToQueue);

        Ok(())
    }

    pub fn remove_from_queue(&mut self, index: usize) -> Result<(), ()> {
        if self.queue.len() <= index {
            return Err(());
        }

        self.queue.remove(index);

        self.emit_event(Event::RemoveFromQueue);

        Ok(())
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
        if index < self.queue.len() {
            return Err(());
        }

        let downloadable = self.queue.swap_remove(index);
        self._download(downloadable, dest_folder);

        Ok(())
    }

    pub fn download_queue(&mut self, dest_folder: &Path) {
        while let Some(downloadable) = self.queue.pop() {
            self._download(downloadable, dest_folder)
        }
    }

    pub fn on<F>(&mut self, event: Event, callback: F)
    where
        F: Fn(&Downloader) -> () + Send + 'static,
    {
        self.event_listeners.push(EventListener {
            event,
            callback: Box::new(callback),
        });
    }

    fn emit_event(&self, event: Event) {
        self.event_listeners.iter().for_each(|listener| {
            if listener.event == event {
                (listener.callback)(&self);
            }
        })
    }

    fn _download(&self, downloadable: Box<dyn DownloadableSong>, dest_folder: &Path) {
        let dest_folder = dest_folder.to_path_buf();

        self.pool.execute(move || {
            block_on(downloadable.download(dest_folder));
        });
    }
}
