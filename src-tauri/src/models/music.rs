use deezer::models::Track;
use serde::Serialize;
use ts_rs::TS;
use ytextract::Video;

#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../src/models/")]
pub struct Album {
    pub title: String,
    pub cover_url: String,
}

#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../src/models/")]
pub struct Song {
    pub id: String,
    pub title: String,
    pub album: Album,
    pub artist: String,
}

impl From<Track> for Song {
    fn from(track: Track) -> Self {
        Self {
            id: track.id.to_string(),
            title: track.title,
            artist: track.artist.name,
            album: Album {
                title: track.album.title,
                cover_url: track.album.cover,
            },
        }
    }
}

impl From<Video> for Song {
    fn from(video: Video) -> Self {
        todo!("YouTube not implemented")
    }
}
