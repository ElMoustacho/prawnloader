use std::{
    env::{current_dir, current_exe},
    fs::{create_dir_all, DirBuilder},
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

use color_eyre::eyre::{eyre, OptionExt, Result};
use crossbeam_channel::{unbounded, Sender};
use deezer_downloader::song;
use sanitize_filename::sanitize;
use tauri::api::path::download_dir;
use tempfile::{tempfile, TempDir};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{ChildStdout, Command},
};
use url::Url;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

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

    let download_folder = if config.youtube_split_chapters && config.group_songs_in_folder {
        config
            .download_folder
            .join(replace_illegal_characters(&song.title))
    } else {
        config.download_folder.clone()
    };

    log::debug!("Download folder is {download_folder:#?}");

    let mut yt_dlp_child = Command::new("yt-dlp")
        .args(vec![
            "--extract-audio",
            "--audio-format",
            "opus",
            "-o",
            "-", // -o - allows for piping the output to our program
            "--embed-metadata",
            &song.id,
        ])
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start yt-dlp");

    let mut stdout = yt_dlp_child
        .stdout
        .take()
        .expect("Process did not have a stdout");

    let stderr = yt_dlp_child
        .stderr
        .take()
        .expect("Process did not have a stderr");
    let mut stderr_reader = BufReader::new(stderr).lines();

    let id = song.id.clone();
    tokio::spawn(async move {
        while let Ok(Some(line)) = stderr_reader.next_line().await {
            log::info!("yt-dlp {}: {line}", id);
        }
    });

    if !matches!(config.youtube_format, YoutubeFormat::WEBM) || config.youtube_split_chapters {
        run_ffmpeg(stdout, &config, download_folder, song).await
    } else {
        let file_path = download_folder
            .join(sanitize(song.title))
            .with_extension(config.youtube_format.to_string());
        log::debug!("Creating file {:#?}", &file_path);
        let mut file = File::create(file_path).await?;

        write_to_file(&mut stdout, &mut file).await
    }
}

async fn run_ffmpeg(
    mut pipe: ChildStdout,
    config: &Config,
    download_folder: PathBuf,
    song: Song,
) -> Result<()> {
    let format = &config.youtube_format.to_string();
    let args = vec![
        "-loglevel",
        "error",
        "-f",
        "webm", // Input format for the data, should be webm
        "-i",
        "pipe:",
        "-f",
        format,
        "pipe:",
    ];

    log::debug!("Running ffmpeg with args {args:?}");

    let mut ffmpeg_child = Command::new("ffmpeg")
        .args(args)
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ffmpeg");

    let mut stdin = ffmpeg_child
        .stdin
        .take()
        .expect("Process did not have a stdin");

    let mut stdout = ffmpeg_child
        .stdout
        .take()
        .expect("Process did not have a stdout");

    let stderr = ffmpeg_child
        .stderr
        .take()
        .expect("Process did not have a stderr");
    let mut stderr_reader = BufReader::new(stderr).lines();

    tokio::spawn(async move {
        while let Ok(Some(line)) = stderr_reader.next_line().await {
            log::error!("Error when transforming song with ffmpeg: {line}");
        }
    });

    tokio::spawn(async move {
        let mut buffer = vec![0; 1024];
        loop {
            let bytes_read = pipe
                .read(&mut buffer)
                .await
                .expect("Couldn't read from pipe");

            if bytes_read == 0 {
                break;
            }

            println!("Writing {bytes_read} to ffmpeg");

            stdin
                .write_all(&buffer[..bytes_read])
                .await
                .expect("Couldn't write to ffmpeg stdin");
        }
    });

    let file_path = download_folder
        .join(sanitize(song.title))
        .with_extension(config.youtube_format.to_string());
    log::debug!("Creating file on path {file_path:#?}");
    let mut file = File::create(file_path).await?;

    write_to_file(&mut stdout, &mut file).await
}

async fn write_to_file(reader: &mut ChildStdout, file: &mut File) -> Result<()> {
    log::debug!("Writing to file");
    let mut stdout_buffer = vec![0; 1024];
    loop {
        let bytes_read = reader.read(&mut stdout_buffer).await?;

        if bytes_read == 0 {
            return Ok(());
        }

        println!("Writing {bytes_read} to file");

        file.write_all(&stdout_buffer[..bytes_read]).await?;
    }
}

async fn download_playlist(
    request: YoutubeRequest,
    progress_tx: &Sender<ProgressEvent>,
) -> Result<()> {
    todo!()
}

fn format_filename(title: &str, extension: &str) -> String {
    format!("{}.{}", replace_illegal_characters(&title), extension)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::models::music::SongAlbum;

    use super::*;

    #[tokio::test]
    async fn download_song() {
        env_logger::init();

        static VIDEO_ID: &str = "bbcPLei01Ls";

        let album = SongAlbum {
            title: String::from("title"),
            cover_url: String::from("cover_url"),
        };
        let song = Song {
            id: VIDEO_ID.to_string(),
            title: String::from("title"),
            album,
            artist: String::from("artist"),
            release_date: String::from("release_date"),
        };

        let download_request = DownloadRequest {
            item: Item::YoutubeVideo(song),
            request_id: Uuid::default(),
        };
        let config = Config {
            download_folder: download_dir().expect("Didn't find a download directory"),
            youtube_format: YoutubeFormat::MP3,
            youtube_split_chapters: false,
            ..Default::default()
        };
        let request = YoutubeRequest(download_request, config);

        let (progress_tx, progress_rx) = crossbeam_channel::unbounded();
        super::download_song(request, &progress_tx)
            .await
            .expect("Error while downloading the song");
    }
}
