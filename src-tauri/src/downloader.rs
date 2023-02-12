use crate::{music::Song, parser::parse_url};
use std::path::Path;

type Queue = Vec<Box<dyn DownloadableSong>>;

#[derive(PartialEq)]
pub enum Event {
    AddToQueue,
    RemoveFromQueue,
}

pub trait DownloadableSong: Send {
    fn download(&self, dest_folder: &Path) -> Result<Box<Path>, ()>;
    fn get_song(&self) -> &Song;
}

struct EventListener {
    event: Event,
    callback: Box<dyn Fn(&Downloader) -> () + Send>,
}

pub struct Downloader {
    queue: Queue,
    event_listeners: Vec<EventListener>,
}

impl Downloader {
    pub fn new() -> Downloader {
        Downloader {
            queue: Vec::new(),
            event_listeners: Vec::new(),
        }
    }

    pub fn add_to_queue(&mut self, url: impl Into<String>) -> Result<(), ()> {
        let mut downloadables = parse_url(&url.into())?;

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

    pub fn download(&self, index: usize, dest_folder: &Path) -> Result<Box<Path>, ()> {
        if let Some(downloadable) = self.queue.get(index) {
            return downloadable.download(dest_folder);
        }

        Err(())
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
}
