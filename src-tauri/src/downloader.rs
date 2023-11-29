use color_eyre::eyre::{eyre, Result};
use crossbeam_channel::{unbounded, Receiver, Sender};
use deezer::{models::Track, DeezerClient};
use deezer_downloader::{
    song::{Album, Artist},
    Downloader as DeezerDownloader, Song, SongMetadata,
};
use futures::future::join_all;
use serde::Serialize;
use tauri::api::path::download_dir;

static DOWNLOAD_THREADS: u64 = 4;

pub type Id = u64;

#[derive(Debug, Clone, Serialize, strum_macros::Display)]
pub enum ProgressEvent {
    Waiting(Track),
    Start(Track),
    Finish(Track),
    DownloadError(Track),
}

#[derive(Debug, strum_macros::Display)]
pub enum DownloadStatus {
    Downloading,
    Inactive,
}

#[derive(Debug)]
pub struct Downloader {
    deezer_client: DeezerClient,
    progress_rx: Receiver<ProgressEvent>,
    download_tx: Sender<Track>,
}

impl Downloader {
    pub fn new() -> Self {
        let (download_tx, download_rx) = unbounded::<Track>();
        let (progress_tx, progress_rx) = unbounded();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                let downloader = DeezerDownloader::new().await.unwrap();
                while let Ok(track) = _download_rx.recv() {
                    _progress_tx
                        .send(ProgressEvent::Start(track.clone()))
                        .unwrap();

                    let result = download_song_from_track(track.clone(), &downloader).await;
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(track),
                        Err(_) => ProgressEvent::DownloadError(track),
                    };

                    _progress_tx.send(progress).unwrap();
                }
            });
        }

        Downloader {
            deezer_client: DeezerClient::new(),
            download_tx,
            progress_rx,
        }
    }

    pub fn get_progress_rx(&self) -> Receiver<ProgressEvent> {
        self.progress_rx.clone()
    }

    pub fn request_download(&self, track: Track) {
        self.download_tx
            .send(track)
            .expect("Channel should be open");
    }

    pub async fn get_track(&self, id: u64) -> Option<Track> {
        let maybe_track = self.deezer_client.track(id).await;

        // Check if the song was found AND is readable
        match maybe_track {
            Ok(Some(track)) if track.readable => Some(track),
            _ => None,
        }
    }

    pub async fn get_album_tracks(&self, id: u64) -> Option<Vec<Track>> {
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

/// Replaces illegal characters for a Windows file.
fn replace_illegal_characters(str: &str) -> String {
    static ILLEGAL_CHARACTERS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

    str.chars()
        .filter(|char| !ILLEGAL_CHARACTERS.contains(char))
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_invalid_chars() {
        let file_name = "AC/DC - Thunderstruck.mp3";

        assert_eq!(
            "ACDC - Thunderstruck.mp3",
            replace_illegal_characters(file_name)
        );

        let file_name = "<>:\"/\\|?* - Test.mp3";

        assert_eq!(" - Test.mp3", replace_illegal_characters(file_name));
    }
}
