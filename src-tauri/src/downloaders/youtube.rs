use std::{
    fs::{create_dir_all, DirBuilder},
    path::Path,
    process::Stdio,
};

use color_eyre::eyre::{eyre, Result};
use crossbeam_channel::{unbounded, Sender};
use futures::future::join_all;
use rusty_ytdl::{
    search::{Playlist, PlaylistSearchOptions},
    FFmpegArgs, Video, VideoDetails,
};
use tempfile::TempDir;
use tokio::process::Command;

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

async fn download_song(request: YoutubeRequest, progress_tx: &Sender<ProgressEvent>) -> Result<()> {
    let YoutubeRequest(DownloadRequest { item, request_id }, config) = request;
    let Item::YoutubeVideo(song) = item else {
        unreachable!("Item should be YoutubeVideo.");
    };

    let file_format: String = config.youtube_format.to_string();
    let video = Video::new(song.id.clone())?;

    let file_name = format_filename(&song.title, &file_format);

    let args = FFmpegArgs {
        format: Some(file_format.clone()),
        audio_filter: None,
        video_filter: None,
    };
    progress_tx.send(ProgressEvent::Start(request_id)).unwrap();
    if config.youtube_split_chapters {
        let video_details = video.get_info().await?.video_details;

        let temp_dir = TempDir::new()?;
        let temp_dir_path = temp_dir.path();

        let temp_vid_filename = "temp";
        let temp_vid_path = temp_dir_path.join(temp_vid_filename);

        let dest_folder_path = config
            .download_folder
            .join(replace_illegal_characters(&song.title));

        video
            .download_with_ffmpeg(&temp_vid_path, Some(args))
            .await?;
        DirBuilder::new()
            .recursive(true)
            .create(&dest_folder_path)?;
        split_video_by_chapters(
            video_details,
            file_format,
            &temp_vid_path,
            &dest_folder_path,
        )
        .await;

        drop(temp_dir);
    } else {
        let video_path = config.download_folder.join(file_name);
        DirBuilder::new()
            .recursive(true)
            .create(config.download_folder.clone())?;
        video.download_with_ffmpeg(video_path, Some(args)).await?;
    }

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

    let download_dir = if config.group_songs_in_folder {
        config
            .download_folder
            .join(replace_illegal_characters(&playlist.title.clone()))
    } else {
        config.download_folder
    };
    let futures: Vec<_> = playlist
        .songs
        .into_iter()
        .map(|song| {
            let file_format = config.youtube_format.to_string();
            let download_dir = download_dir.clone();
            async move {
                let file_format = file_format.clone();
                let file_name = format_filename(&song.title, &file_format);
                let file_path = &download_dir.join(file_name);
                let args = FFmpegArgs {
                    format: Some(file_format.to_owned()),
                    audio_filter: None,
                    video_filter: None,
                };

                if let Ok(video) = Video::new(song.id.clone()) {
                    create_dir_all(&download_dir)?;
                    video.download_with_ffmpeg(file_path, Some(args)).await?;

                    return Ok(());
                };

                Err(eyre!("Error while downloading video with id {}", song.id))
            }
        })
        .collect();

    let maybe_songs = join_all(futures).await;

    for (i, maybe_song) in maybe_songs.into_iter().enumerate() {
        let _ = match maybe_song {
            Ok(_) => progress_tx.send(ProgressEvent::AlbumTrackComplete(request_id, i)),
            Err(err) => progress_tx.send(ProgressEvent::AlbumTrackError(
                request_id,
                i,
                err.to_string(),
            )),
        };
    }

    progress_tx.send(ProgressEvent::Start(request_id)).unwrap();

    Ok(())
}

// TODO: Use ffmpeg stream to split song
async fn split_video_by_chapters(
    video_details: VideoDetails,
    file_format: String,
    video_source_path: &Path,
    dest_folder_path: &Path,
) {
    for (index, chapter) in video_details.chapters.iter().enumerate() {
        let output_filename = format_filename(&chapter.title, &file_format);
        let output_path = dest_folder_path.join(output_filename);
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
            video_source_path.to_str().unwrap(),
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
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();
    }
}

fn format_filename(title: &str, extension: &str) -> String {
    format!("{}.{}", replace_illegal_characters(&title), extension)
}
