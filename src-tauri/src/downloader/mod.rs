use anyhow::Context;
use anyhow::Result;
use crossbeam::channel::{unbounded, Receiver, Sender};
use queue::*;
use std::path::{Path, PathBuf};

use crate::youtube::YoutubeParser;
use crate::{music::Song, parser::Parser};

pub(crate) mod queue;

pub enum Event {
    UpdateQueue(SerializableQueue),
    DownloadStarted(Song),
    DownloadComplete(PathBuf),
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
            self.queue.push(downloadable);
        }

        self.emit_queue_update();

        Ok(())
    }

    pub fn remove_from_queue(&mut self, index: Id) -> Result<(), ()> {
        self.queue.retain(|song| song.id != index);

        self.emit_queue_update();

        Ok(())
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();

        self.emit_queue_update();
    }

    pub fn start_download(&mut self, index: usize, dest_folder: &Path) -> Result<()> {
        let queue_song = self
            .queue
            .iter_mut()
            .find(|song| song.id == index)
            .context("Song not found.")?;

        queue_song.start_download(dest_folder.to_owned(), self.event_sender.clone());

        Ok(())
    }

    pub fn start_download_queue(&mut self, dest_folder: &Path) {
        for song in self.queue.iter_mut() {
            song.start_download(dest_folder.to_owned(), self.event_sender.clone());
        }
    }

    fn emit_queue_update(&self) {
        self.event_sender
            .send(Event::UpdateQueue(
                self.queue
                    .iter()
                    .map(|song| song.get_serializable())
                    .collect(),
            ))
            .expect("Channel should be connected.");
    }
}
