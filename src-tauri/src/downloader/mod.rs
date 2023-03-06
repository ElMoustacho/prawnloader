use crate::youtube::YoutubeParser;
use crate::{music::Song, parser::Parser};
use anyhow::Result;
use async_trait::async_trait;
use crossbeam::channel::{unbounded, Receiver, Sender};
use queue::*;
use std::path::{Path, PathBuf};

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
    pub parser: Parser,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
    queue: Queue,
}

impl Downloader {
    pub fn new() -> Downloader {
        let (event_sender, event_receiver) = unbounded();

        Downloader {
            parser: Parser::new(vec![Box::new(YoutubeParser::new())]),
            event_sender,
            event_receiver,
            queue: Vec::new(),
        }
    }

    pub fn get_event_receiver(&self) -> Receiver<Event> {
        self.event_receiver.clone()
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
        // let queue_item = self.queue.get_mut(index).context("Song not found.");
        // let queue_item = if let Ok(x) = queue_item { x } else { return };

        // if queue_item.downloaded {
        //     return;
        // };

        // let dest_folder = dest_folder.to_owned();
        // let event_sender = self.event_sender.clone();

        // queue_item.download_handle = {
        //     Some(tokio::spawn(async move {
        //         let result = queue_item
        //             .downloadable_song
        //             .download(dest_folder.clone())
        //             .await;

        //         event_sender.send(Event::DownloadComplete(dest_folder));

        //         result
        //     }))
        // };
    }

    fn emit_queue_update(&self) {
        self.event_sender.send(Event::UpdateQueue(
            self.queue
                .iter()
                .map(|song| song.get_serializable())
                .collect(),
        ));
    }
}
