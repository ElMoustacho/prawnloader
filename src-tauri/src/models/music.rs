use deezer::models::Track;
use serde::Serialize;
use ts_rs::TS;
use ytextract::{playlist::Video as PlaylistVideo, Video};

#[derive(TS, Debug, Serialize, Clone, Default)]
#[ts(export, export_to = "../src/models/")]
pub struct Album {
    pub title: String,
    pub cover_url: String,
}

#[derive(TS, Debug, Serialize, Clone, Default)]
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
        Self {
            id: video.id().to_string(),
            title: video.title().to_string(),
            album: Album::default(),
            artist: video.channel().name().to_string(),
        }
    }
}

impl From<PlaylistVideo> for Song {
    fn from(video: PlaylistVideo) -> Self {
        Self {
            id: video.id().to_string(),
            title: video.title().to_string(),
            album: Album::default(),
            artist: video.channel().name().to_string(),
        }
    }
}

impl From<rusty_ytdl::VideoDetails> for Song {
    fn from(video_details: rusty_ytdl::VideoDetails) -> Self {
        let artist = video_details
            .author
            .map(|author| author.name)
            .unwrap_or_default();

        Self {
            id: video_details.video_id,
            title: video_details.title,
            album: Album::default(),
            artist,
        }
    }
}
