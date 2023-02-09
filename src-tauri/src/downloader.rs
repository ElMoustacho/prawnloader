use crate::{music::Song, parser::parse_url};
use std::path::Path;

pub trait DownloadableSong: Send {
    fn download(&self, dest_folder: &Path) -> Result<Box<Path>, ()>;
    fn get_song(&self) -> &Song;
}

pub struct Downloader {
    queue: Vec<Box<dyn DownloadableSong>>,
}

impl Downloader {
    pub fn new() -> Downloader {
        Downloader { queue: vec![] }
    }

    pub fn add_to_queue(&mut self, url: impl Into<String>) -> Result<(), ()> {
        let mut downloadables = parse_url(&url.into())?;

        self.queue.append(&mut downloadables);

        Ok(())
    }

    pub fn remove_from_queue(&mut self, index: usize) -> Result<(), ()> {
        if self.queue.len() <= index {
            return Err(());
        }

        self.queue.remove(index);

        Ok(())
    }

    pub fn download(&self, index: usize, dest_folder: &Path) -> Result<Box<Path>, ()> {
        if let Some(downloadable) = self.queue.get(index) {
            return downloadable.download(dest_folder);
        }

        Err(())
    }
}
