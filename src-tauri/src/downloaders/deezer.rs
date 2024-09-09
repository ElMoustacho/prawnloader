use color_eyre::eyre::{eyre, Result};
use crossbeam_channel::{unbounded, Sender};
use deezer::{models::Track, DeezerClient};
use deezer_downloader::{
    song::{Album, Artist},
    Downloader as DeezerDownloader, SongMetadata,
};
use futures::future::join_all;
use tauri::api::path::download_dir;

use crate::models::music::Song;

use super::{replace_illegal_characters, DeezerId, ProgressEvent};

static DOWNLOAD_THREADS: u64 = 4;

#[derive(Debug)]
pub struct Downloader {
    deezer_client: DeezerClient,
    download_tx: Sender<Song>,
}

impl Downloader {
    pub fn new(progress_tx: Sender<ProgressEvent>) -> Self {
        let (download_tx, download_rx) = unbounded::<Song>();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                let downloader = DeezerDownloader::new().await.unwrap();
                while let Ok(song) = _download_rx.recv() {
                    _progress_tx
                        .send(ProgressEvent::Start(song.clone()))
                        .unwrap();

                    let result = download_song(song.clone(), &downloader).await;
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(song),
                        // FIXME: Add download error String
                        Err(_) => ProgressEvent::DownloadError(song, String::new()),
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

    pub async fn request_download(&self, song: Song) -> Result<()> {
        self.download_tx.send(song).expect("Channel should be open");

        Ok(())
    }

    pub async fn get_track(&self, id: DeezerId) -> Option<Track> {
        let maybe_track = self.deezer_client.track(id).await;

        // Check if the song was found AND is readable
        match maybe_track {
            Ok(Some(track)) if track.readable => Some(track),
            _ => None,
        }
    }

    pub async fn get_album_tracks(&self, id: DeezerId) -> Option<Vec<Song>> {
        let maybe_album = self.deezer_client.album(id).await;
        if let Ok(Some(album)) = maybe_album {
            let futures: Vec<_> = album
                .tracks
                .into_iter()
                .map(|album_track| {
                    async move {
                        // FIXME: Expect may panic on unsable connections
                        album_track
                            .get_full()
                            .await
                            .expect("Track should always be available.")
                            .into()
                    }
                })
                .collect();

            return Some(join_all(futures).await);
        }

        None
    }
}

async fn download_song(song: Song, downloader: &DeezerDownloader) -> Result<()> {
    let maybe_song =
        deezer_downloader::Song::download_from_metadata(metadata_from_song(song), downloader).await;
    let song = match maybe_song {
        Ok(it) => it,
        Err(_) => return Err(eyre!("Song not found.")),
    };

    write_song_to_file(&song)?;

    Ok(())
}

/// Write a [Song] to the download directory.
///
/// TODO: Allow the target directory to be given.
fn write_song_to_file(song: &deezer_downloader::Song) -> Result<()> {
    let Some(download_dir) = download_dir() else {
        return Ok(());
    };

    let song_title = format!(
        "{} - {}.mp3",
        song.tag.artist().unwrap_or_default(),
        song.tag.title().unwrap_or_default()
    );
    let song_title = replace_illegal_characters(&song_title);

    song.write_to_file(download_dir.join(song_title))
        .map_err(|_| eyre!("An error occured while writing the file."))?;

    Ok(())
}

fn metadata_from_song(song: Song) -> SongMetadata {
    SongMetadata {
        id: song.id.parse().unwrap_or_default(),
        title: song.title,
        artist: Artist {
            // Id is not used in the metadata
            id: Default::default(),
            name: song.artist,
        },
        album: Album {
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
