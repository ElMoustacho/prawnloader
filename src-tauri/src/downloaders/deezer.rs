use color_eyre::eyre::{eyre, Result};
use crossbeam_channel::{unbounded, Sender};
use deezer::{models::Track, DeezerClient};
use deezer_downloader::{
    song::{Album, Artist},
    Downloader as DeezerDownloader, Song, SongMetadata,
};
use futures::future::join_all;
use tauri::api::path::download_dir;

use super::{replace_illegal_characters, DeezerId, ProgressEvent};

static DOWNLOAD_THREADS: u64 = 4;

#[derive(Debug)]
pub struct Downloader {
    deezer_client: DeezerClient,
    download_tx: Sender<Track>,
}

impl Downloader {
    pub fn new(progress_tx: Sender<ProgressEvent>) -> Self {
        let (download_tx, download_rx) = unbounded::<Track>();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                let downloader = DeezerDownloader::new().await.unwrap();
                while let Ok(track) = _download_rx.recv() {
                    _progress_tx
                        .send(ProgressEvent::Start(track.clone().into()))
                        .unwrap();

                    let result = download_song_from_track(track.clone(), &downloader).await;
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(track.into()),
                        Err(_) => ProgressEvent::DownloadError(track.into()),
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

    pub async fn request_download(&self, track_id: DeezerId) -> Result<()> {
        let track = self
            .get_track(track_id)
            .await
            .ok_or(eyre!("Unable to find track with id {track_id}"))?;

        self.download_tx
            .send(track)
            .expect("Channel should be open");

        Ok(())
    }

    pub async fn get_track(&self, id: u64) -> Option<Track> {
        let maybe_track = self.deezer_client.track(id).await;

        // Check if the song was found AND is readable
        match maybe_track {
            Ok(Some(track)) if track.readable => Some(track),
            _ => None,
        }
    }

    pub async fn get_album_tracks(&self, id: u64) -> Option<Vec<crate::models::music::Song>> {
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

async fn download_song_from_track(track: Track, downloader: &DeezerDownloader) -> Result<()> {
    let id = track.id;
    let song = match Song::download_from_metadata(metadata_from_track(track), downloader).await {
        Ok(it) => it,
        Err(_) => return Err(eyre!(format!("Song with id {} not found.", id))),
    };

    write_song_to_file(song)?;

    Ok(())
}

/// Write a [Song] to the download directory.
///
/// TODO: Allow the target directory to be given.
fn write_song_to_file(song: Song) -> Result<()> {
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

fn metadata_from_track(track: Track) -> SongMetadata {
    SongMetadata {
        id: track.id,
        title: track.title,
        artist: Artist {
            id: track.artist.id,
            name: track.artist.name,
        },
        album: Album {
            id: track.album.id,
            title: track.album.title,
            cover_small: track.album.cover_small,
            cover_medium: track.album.cover_medium,
            cover_big: track.album.cover_big,
        },
        release_date: Some(track.release_date),
    }
}
