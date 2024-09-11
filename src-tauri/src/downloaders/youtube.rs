use std::path::Path;

use color_eyre::eyre::{eyre, Result};
use crossbeam_channel::{unbounded, Sender};
use rusty_ytdl::{
    search::{Playlist, PlaylistSearchOptions},
    FFmpegArgs, Video, VideoDetails,
};
use tauri::api::path::download_dir;
use tokio::process::Command;

use crate::{
    config::{Config, YoutubeFormat},
    models::music::{Album, Item, Song},
};

use super::{
    replace_illegal_characters, DownloadRequest, ProgressEvent, YoutubeId, YoutubePlaylistId,
};

static DOWNLOAD_THREADS: u64 = 4;

struct YoutubeRequest(DownloadRequest, Config);

pub struct Downloader {
    download_tx: Sender<YoutubeRequest>,
}

impl Downloader {
    pub fn new(progress_tx: Sender<ProgressEvent>) -> Self {
        let (download_tx, download_rx) = unbounded::<YoutubeRequest>();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                while let Ok(YoutubeRequest(request, Config { youtube_format, .. })) =
                    _download_rx.recv()
                {
                    let result = match request.item {
                        Item::YoutubeVideo {
                            video,
                            split_by_chapters,
                        } => {
                            _progress_tx
                                .send(ProgressEvent::Start(request.request_id))
                                .unwrap();
                            download_song(video, split_by_chapters, &youtube_format).await
                        }
                        Item::YoutubePlaylist { playlist } => {
                            _progress_tx
                                .send(ProgressEvent::Start(request.request_id))
                                .unwrap();
                            download_album(playlist, &youtube_format).await
                        }
                        _ => continue,
                    };
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(request.request_id),
                        Err(err) => {
                            ProgressEvent::DownloadError(request.request_id, err.to_string())
                        }
                    };

                    _progress_tx.send(progress).unwrap();
                }
            });
        }

        Downloader { download_tx }
    }

    pub async fn request_download(&self, request: DownloadRequest, config: Config) {
        self.download_tx
            .send(YoutubeRequest(request, config))
            .expect("Channel should be open");
    }

    pub async fn get_song(&self, id: YoutubeId) -> Option<Song> {
        let video = Video::new(id.to_string()).ok()?;
        let video_details = video.get_basic_info().await.ok()?.video_details;

        Some(video_details.into())
    }

    pub async fn get_playlist(&self, id: YoutubePlaylistId) -> Option<Album> {
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

        Some(Album {
            title: playlist.name,
            artist: playlist.channel.name,
            // FIXME: Add cover
            cover_url: String::new(),
            songs,
        })
    }
}

async fn download_song(song: Song, split_by_chapters: bool, format: &YoutubeFormat) -> Result<()> {
    let file_format: String = format.to_string();
    let video = Video::new(song.id.clone())?;

    // TODO: Allow the target directory to be given.
    let title = format_title(&song.title, &file_format);
    let video_path = download_dir().unwrap().join(title);
    let args = FFmpegArgs {
        format: Some(file_format),
        audio_filter: None,
        video_filter: None,
    };
    video.download_with_ffmpeg(video_path, Some(args)).await?;

    Ok(())
}

async fn download_album(playlist: Album, format: &YoutubeFormat) -> Result<()> {
    Err(eyre!("Not implemented"))
}

async fn split_video_by_chapters(
    video_details: VideoDetails,
    file_format: String,
    video_path: &Path,
) {
    for (index, chapter) in video_details.chapters.iter().enumerate() {
        let output_filename = format_title(&chapter.title, &file_format);
        let output_path = download_dir().unwrap().join(output_filename);
        let start = chapter.start_time.to_string();
        let end;
        if index != video_details.chapters.len() - 1 {
            end = video_details
                .chapters
                .get(index + 1)
                .unwrap()
                .start_time
                .to_string();
        } else {
            end = video_details.length_seconds.clone();
        }

        let args = vec![
            "-i",
            video_path.to_str().unwrap(),
            "-ss",
            &start,
            "-to",
            &end,
            "-c:a",
            "copy",
            output_path.to_str().unwrap(),
        ];
        Command::new("ffmpeg")
            .args(args)
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();
    }
}

fn format_title(title: &str, extension: &str) -> String {
    format!("{}.{}", replace_illegal_characters(&title), extension)
}
