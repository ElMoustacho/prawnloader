use std::{
    env::{current_dir, current_exe},
    fs::{create_dir_all, DirBuilder},
    path::{Path, PathBuf},
    process::Stdio,
};

use color_eyre::eyre::{eyre, OptionExt, Result};
use crossbeam_channel::{unbounded, Sender};
use deezer_downloader::song;
use tauri::api::path::download_dir;
use tempfile::TempDir;
use tokio::process::Command;
use url::Url;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

use crate::{
    config::Config,
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
                while let Ok(request) = _download_rx.recv() {
                    let request_id = request.0.request_id;
                    let result = match request.0.item {
                        Item::YoutubeVideo { .. } => download_song(request, &_progress_tx).await,
                        Item::YoutubePlaylist { .. } => {
                            download_playlist(request, &_progress_tx).await
                        }

                        _ => continue,
                    };
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(request_id),
                        Err(err) => ProgressEvent::DownloadError(request_id, err.to_string()),
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

    pub async fn get_song(&self, id: YoutubeId) -> Result<Song> {
        let video = match YoutubeDl::new(id).run_async().await? {
            YoutubeDlOutput::Playlist(playlist) => {
                unreachable!()
            }
            YoutubeDlOutput::SingleVideo(video) => video,
        };

        Ok((*video).into())
    }

    pub async fn get_playlist(&self, id: YoutubePlaylistId) -> Result<Album> {
        let result = YoutubeDl::new(id)
            .extra_arg("--compat-options")
            .extra_arg("no-youtube-unavailable-videos")
            .flat_playlist(true)
            .run_async()
            .await?
            .into_playlist()
            .ok_or_eyre("URL is not a playlist")?
            .into();

        Ok(result)
    }
}

async fn download_song(request: YoutubeRequest, progress_tx: &Sender<ProgressEvent>) -> Result<()> {
    let YoutubeRequest(DownloadRequest { item, request_id }, config) = request;
    let Item::YoutubeVideo(song) = item else {
        unreachable!("Item should be YoutubeVideo.");
    };

    progress_tx.send(ProgressEvent::Start(request_id)).unwrap();

    let split_chapters_str = if config.youtube_split_chapters {
        "--split-chapters"
    } else {
        ""
    };

    let download_folder = if config.youtube_split_chapters && config.group_songs_in_folder {
        config
            .download_folder
            .join(replace_illegal_characters(&song.title))
    } else {
        config.download_folder.clone()
    };

    let y = YoutubeDl::new(song.id)
        .extract_audio(true)
        .extra_arg("-t")
        .extra_arg("mp3")
        .extra_arg(split_chapters_str)
        .extra_arg("--embed-metadata")
        .output_template("%(title)s.%(ext)s")
        .download_to_async(config.download_folder)
        .await?;
    Ok(())
}

async fn download_playlist(
    request: YoutubeRequest,
    progress_tx: &Sender<ProgressEvent>,
) -> Result<()> {
    let YoutubeRequest(DownloadRequest { item, request_id }, config) = request;
    let Item::YoutubePlaylist(playlist) = item else {
        unreachable!("Item should be YoutubePlaylist.");
    };

    let _ = progress_tx.send(ProgressEvent::Start(request_id));

    for song in playlist.songs {
        let download_folder = if config.group_songs_in_folder {
            config
                .download_folder
                .join(replace_illegal_characters(&playlist.title))
        } else {
            config.download_folder.clone()
        };

        let split_chapters_str = if config.youtube_split_chapters {
            "--split-chapters"
        } else {
            ""
        };

        let youtube_dl = YoutubeDl::new(song.id)
            .extra_arg(split_chapters_str)
            .extra_arg("--embed-metadata")
            .extract_audio(true)
            .extra_arg("-t")
            .extra_arg("mp3")
            .output_template("%(title)s.%(ext)s")
            .download_to_async(download_folder)
            .await?;
    }

    todo!()
}

fn format_filename(title: &str, extension: &str) -> String {
    format!("{}.{}", replace_illegal_characters(&title), extension)
}
