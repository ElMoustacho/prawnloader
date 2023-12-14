use crossbeam_channel::{unbounded, Sender};
use futures::StreamExt;
use rusty_ytdl::{VideoOptions, VideoQuality, VideoSearchOptions};
use tauri::api::path::download_dir;
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
        let video = rusty_ytdl::Video::new(id.to_string()).ok()?;
        let video_details = video.get_basic_info().await.ok()?.video_details;

        Some(video_details.into())
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
) -> Result<(), rusty_ytdl::VideoError> {
    let id = &video.id()[..];

    // TODO: Enable quality configuration
    let options = VideoOptions {
        quality: VideoQuality::Lowest,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let video = rusty_ytdl::Video::new_with_options(id, options).unwrap();
    let video_details = video.get_basic_info().await?.video_details;

    // TODO: Allow the target directory to be given.
    let video_path = download_dir().unwrap().join(video_details.title);
    video.download(video_path).await.unwrap();

    Ok(())
}
