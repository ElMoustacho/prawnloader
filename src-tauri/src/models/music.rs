use deezer::models::Track;
use serde::Serialize;
use ytextract::Video;

#[derive(Serialize, Clone)]
pub struct Album {
    pub title: String,
    pub cover_url: String,
}

#[derive(Serialize, Clone)]
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
