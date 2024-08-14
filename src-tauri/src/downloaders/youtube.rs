use crossbeam_channel::{unbounded, Sender};
use rusty_ytdl::{
    search::{Playlist, PlaylistSearchOptions},
    FFmpegArgs, Video, VideoError,
};
use tauri::api::path::download_dir;

use crate::models::music::Song;

use super::{replace_illegal_characters, ProgressEvent, YoutubeId, YoutubePlaylistId};

static DOWNLOAD_THREADS: u64 = 4;

pub struct Downloader {
    download_tx: Sender<Song>,
}

impl Downloader {
    pub fn new(progress_tx: Sender<ProgressEvent>) -> Self {
        let (download_tx, download_rx) = unbounded::<Song>();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                while let Ok(song) = _download_rx.recv() {
                    _progress_tx
                        .send(ProgressEvent::Start(song.clone()))
                        .unwrap();

                    let result = download_song(&song).await;
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(song),
                        Err(err) => ProgressEvent::DownloadError(song, err.to_string()),
                    };

                    _progress_tx.send(progress).unwrap();
                }
            });
        }

        Downloader { download_tx }
    }

    pub async fn request_download(&self, song: Song) -> Result<(), VideoError> {
        self.download_tx.send(song).expect("Channel should be open");

        Ok(())
    }

    pub async fn get_song(&self, id: YoutubeId) -> Option<Song> {
        let video = Video::new(id.to_string()).ok()?;
        let video_details = video.get_basic_info().await.ok()?.video_details;

        Some(video_details.into())
    }

    pub async fn get_playlist_songs(&self, id: YoutubePlaylistId) -> Option<Vec<Song>> {
        let options = PlaylistSearchOptions {
            fetch_all: true,
            ..Default::default()
        };
        let playlist = Playlist::get(id.to_string(), Some(&options)).await.ok()?;
        let songs = playlist
            .videos
            .into_iter()
            .map(|video| video.into())
            .collect();

        Some(songs)
    }
}

async fn download_song(song: &Song) -> Result<(), VideoError> {
    // TODO: Allow to choose file format
    let file_format: String = String::from("mp3");
    let video = Video::new(song.id.clone())?;

    // TODO: Allow the target directory to be given.
    let title = format!(
        "{}.{}",
        replace_illegal_characters(&song.title),
        file_format
    );
    let video_path = download_dir().unwrap().join(title);
    let args = FFmpegArgs {
        format: Some(file_format),
        audio_filter: None,
        video_filter: None,
    };
    video.download_with_ffmpeg(video_path, Some(args)).await?;

    Ok(())
}
