use std::process::Stdio;

use color_eyre::eyre::{eyre, Result};
use crossbeam_channel::{unbounded, Sender};
use deezer::DeezerClient;
use deezer_downloader::{
    song::{Album as DownloaderAlbum, Artist as DownloaderArtist},
    Downloader as DeezerDownloader, SongMetadata,
};
use futures::future::join_all;
use tauri::api::path::download_dir;
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{
    config::Config,
    models::music::{Album, Item, Song},
};

use super::{replace_illegal_characters, DeezerId, DownloadRequest, ProgressEvent};

static DOWNLOAD_THREADS: u64 = 4;

struct DeezerRequest(DownloadRequest, Config);

#[derive(Debug)]
pub struct Downloader {
    deezer_client: DeezerClient,
    download_tx: Sender<DeezerRequest>,
}

impl Downloader {
    pub fn new(progress_tx: Sender<ProgressEvent>) -> Self {
        let (download_tx, download_rx) = unbounded::<DeezerRequest>();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                let downloader = DeezerDownloader::new().await.unwrap();
                while let Ok(DeezerRequest(request, ..)) = _download_rx.recv() {
                    let result = match request.item {
                        Item::DeezerAlbum {
                            album,
                            merge_tracks,
                        } => download_album(album, merge_tracks, &downloader, &_progress_tx).await,
                        Item::DeezerTrack { track } => {
                            download_song(track, &downloader, &_progress_tx).await
                        }
                        _ => continue,
                    };

                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(request.request_id),
                        // FIXME: Add download error String
                        Err(_) => ProgressEvent::DownloadError(request.request_id, String::new()),
                    };

                    _progress_tx.send(progress).unwrap();
                }
            });
        }

        Downloader {
            deezer_client: DeezerClient::new(),
            download_tx,
        }
    }

    pub async fn request_download(&self, request: DownloadRequest, config: Config) {
        self.download_tx
            .send(DeezerRequest(request, config))
            .expect("Channel should be open");
    }

    pub async fn get_song(&self, id: DeezerId) -> Option<Song> {
        let maybe_track = self.deezer_client.track(id).await;

        // Check if the song was found AND is readable
        match maybe_track {
            Ok(Some(track)) if track.readable => Some(track.into()),
            _ => None,
        }
    }

    pub async fn get_album(&self, id: DeezerId) -> Option<Album> {
        let maybe_album = self.deezer_client.album(id).await;
        if let Ok(Some(album)) = maybe_album {
            let futures: Vec<_> = album
                .tracks
                .into_iter()
                .map(|album_track| async move {
                    loop {
                        match album_track.get_full().await {
                            Ok(track) => return track.into(),
                            Err(err) => match err {
                                deezer::DeezerError::HttpError(_) => continue,
                            },
                        }
                    }
                })
                .collect();
            let songs = join_all(futures).await;

            return Some(Album {
                title: album.title,
                artist: album.artist.name,
                cover_url: album.cover,
                songs,
            });
        }

        None
    }
}

async fn download_song(
    song: Song,
    downloader: &DeezerDownloader,
    progress_tx: &Sender<ProgressEvent>,
) -> Result<()> {
    let maybe_song =
        deezer_downloader::Song::download_from_metadata(metadata_from_song(song), downloader).await;
    let song = match maybe_song {
        Ok(it) => it,
        Err(_) => return Err(eyre!("Song not found.")),
    };

    write_song_to_file(&song)?;

    let _ = progress_tx.send(ProgressEvent::Finish(todo!(
        "Handle UUID for individual songs."
    )));

    Ok(())
}

async fn download_album(
    album: Album,
    merge_tracks: bool,
    downloader: &DeezerDownloader,
    progress_tx: &Sender<ProgressEvent>,
) -> Result<()> {
    let download_dir = download_dir().ok_or(eyre!("Cannot find download directory."))?;
    let maybe_songs: Vec<_> = album
        .songs
        .into_iter()
        .map(|song| {
            deezer_downloader::Song::download_from_metadata(metadata_from_song(song), downloader)
        })
        .collect();
    let maybe_songs = join_all(maybe_songs).await;

    if merge_tracks {
        // Immediatly return if there is an error on any song
        let songs: Result<Vec<deezer_downloader::Song>, _> = maybe_songs.into_iter().collect();
        let songs = songs.map_err(|err| eyre!(err.to_string()))?;

        let file_name = format_title(&album.artist, &album.title);
        let file_path_str = download_dir
            .join(file_name)
            .into_os_string()
            .into_string()
            .unwrap();

        let args = vec![
            "-y",
            "-f",
            "mp3",
            "-i",
            "pipe:",
            "-c:a",
            "copy",
            &file_path_str,
        ];
        let mut ffmpeg_process = Command::new("ffmpeg")
            .args(args)
            .current_dir(download_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .unwrap();

        let mut ffmpeg_stdin = ffmpeg_process.stdin.take().unwrap();

        for song in songs {
            ffmpeg_stdin.write_all(&song.content).await?;
        }

        drop(ffmpeg_stdin);
        ffmpeg_process.wait().await?;

        progress_tx.send(ProgressEvent::Finish(todo!(
            "Handle UUID for individual songs."
        )));

        Ok(())
    } else {
        for maybe_song in maybe_songs {
            let result: Result<()> = match maybe_song {
                Ok(song) => write_song_to_file(&song),
                Err(err) => Err(eyre!(err)),
            };

            let _ = match result {
                Ok(_) => progress_tx.send(ProgressEvent::Finish(todo!(
                    "Handle UUID for individual songs."
                ))),
                Err(err) => progress_tx.send(ProgressEvent::DownloadError(
                    todo!("Handle UUID for individual songs."),
                    err.to_string(),
                )),
            };
        }

        Ok(())
    }
}

/// Write a [Song] to the download directory.
///
/// TODO: Allow the target directory to be given.
fn write_song_to_file(song: &deezer_downloader::Song) -> Result<()> {
    let Some(download_dir) = download_dir() else {
        return Ok(());
    };

    let song_title = format_song(song);
    song.write_to_file(download_dir.join(song_title))
        .map_err(|_| eyre!("An error occured while writing the file."))?;

    Ok(())
}

fn format_song(song: &deezer_downloader::Song) -> String {
    format_title(
        song.tag.artist().unwrap_or_default(),
        song.tag.title().unwrap_or_default(),
    )
}

fn format_title(artist: &str, title: &str) -> String {
    replace_illegal_characters(&(format!("{} - {}.mp3", artist, title)))
}

fn metadata_from_song(song: Song) -> SongMetadata {
    SongMetadata {
        id: song.id.parse().unwrap_or_default(),
        title: song.title,
        artist: DownloaderArtist {
            // Id is not used in the metadata
            id: Default::default(),
            name: song.artist,
        },
        album: DownloaderAlbum {
            // Id is not used in the metadata
            id: Default::default(),
            title: song.album.title,
            // Only cover_big is used in the metadata
            cover_big: song.album.cover_url,
            cover_medium: Default::default(),
            cover_small: Default::default(),
        },
        release_date: Some(song.release_date),
    }
}
