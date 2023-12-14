use crossbeam_channel::{unbounded, Sender};
use futures::StreamExt;
use rustube::download_best_quality;
use ytextract::Client;

use crate::models::music::Song;

use super::ProgressEvent;

static DOWNLOAD_THREADS: u64 = 4;

pub struct Downloader {
    youtube_client: Client,
    download_tx: Sender<ytextract::video::Video>,
}

impl Downloader {
    pub fn new(progress_tx: Sender<ProgressEvent>) -> Self {
        let (download_tx, download_rx) = unbounded::<ytextract::video::Video>();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                while let Ok(video) = _download_rx.recv() {
                    _progress_tx
                        .send(ProgressEvent::Start(video.clone().into()))
                        .unwrap();

                    let result = download_song_from_video(&video).await;
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(video.into()),
                        Err(_) => ProgressEvent::DownloadError(video.into()),
                    };

                    _progress_tx.send(progress).unwrap();
                }
            });
        }

        Downloader {
            youtube_client: Client::new(),
            download_tx,
        }
    }

    pub fn request_download(&self, track: ytextract::video::Video) {
        self.download_tx
            .send(track)
            .expect("Channel should be open");
    }

    pub async fn get_song(&self, id: ytextract::video::Id) -> Option<Song> {
        self.youtube_client.video(id).await.ok().map(|x| x.into())
    }

    pub async fn get_playlist_songs(&self, id: ytextract::playlist::Id) -> Option<Vec<Song>> {
        let playlist = self.youtube_client.playlist(id).await.ok()?;
        let songs = playlist
            .videos()
            .filter_map(|result| async { result.ok() })
            .map(|video| video.into())
            .collect()
            .await;

        Some(songs)
    }
}

async fn download_song_from_video(
    video: &ytextract::video::Video,
) -> Result<std::path::PathBuf, rustube::Error> {
    download_best_quality(&video.id()[..]).await
}
