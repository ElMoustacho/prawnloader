use anyhow::Context;
use anyhow::Result;
use crossbeam::channel::{unbounded, Receiver, Sender};
use futures::executor::ThreadPool;
use std::path::{Path, PathBuf};

use crate::music::Song;

use super::queue::DownloadState;
use super::queue::{Id, Queue, QueueSong};

pub enum Event {
    UpdateQueue(Queue),
    DownloadStarted(Song),
    DownloadComplete(PathBuf),
}

pub struct Downloader {
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
    download_sender: Sender<String>,
    download_receiver: Receiver<String>,
    queue: Queue,
    threadpool: ThreadPool,
}

impl Downloader {
    pub fn new() -> Downloader {
        let (event_sender, event_receiver) = unbounded();
        let (download_sender, download_receiver) = unbounded();

        Downloader {
            event_sender,
            event_receiver,
            download_sender,
            download_receiver,
            queue: Vec::new(),
            threadpool: ThreadPool::new().expect("Error while building ThreadPool."),
        }
    }

    pub fn get_event_receiver(&self) -> Receiver<Event> {
        self.event_receiver.clone()
    }

    pub async fn add_to_queue(&mut self, songs: &mut Vec<QueueSong>) {
        self.queue.append(songs);

        self.emit_queue_update();
    }

    pub fn remove_from_queue(&mut self, id: Id) {
        self.queue.retain(|song| song.id != id);

        self.emit_queue_update();
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();

        self.emit_queue_update();
    }

    pub fn start_download(&mut self, id: Id, dest_folder: &Path) -> Result<()> {
        let queue_song = self
            .queue
            .iter_mut()
            .find(|song| song.id == id)
            .context("Song not found.")?;

        download_song(queue_song, dest_folder.clone(), &self.threadpool);

        Ok(())
    }

    pub fn start_download_queue(&mut self, dest_folder: &Path) {
        for queue_song in self.queue.iter_mut() {
            download_song(queue_song, dest_folder.clone(), &self.threadpool);
        }
    }

    fn emit_queue_update(&self) {
        self.event_sender
            .send(Event::UpdateQueue(self.queue.clone()))
            .expect("Channel should be connected.");
    }
}

fn download_song(queue_song: &mut QueueSong, dest_folder: &Path, threadpool: &ThreadPool) {
    let dl_fun = queue_song.download_fun;
    let url = queue_song.url.clone();
    let dest_folder = dest_folder.to_owned();

    queue_song.download_state = DownloadState::Downloading;

    threadpool.spawn_ok(async move {
        (dl_fun)(&url, dest_folder.to_path_buf()).await;
    })
}
