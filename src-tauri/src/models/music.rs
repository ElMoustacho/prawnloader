use deezer::models::Track;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use youtube_dl::SingleVideo;

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export)]
pub struct Album {
    pub title: String,
    pub artist: String,
    pub cover_url: String,
    pub songs: Vec<Song>,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
pub struct SongAlbum {
    pub title: String,
    pub cover_url: String,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[ts(export)]
pub enum Item {
    DeezerAlbum(Album),
    DeezerTrack(Song),
    YoutubeVideo(Song),
    YoutubePlaylist(Album),
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export)]
pub struct Song {
    pub id: String,
    pub title: String,
    #[ts(inline)]
    pub album: SongAlbum,
    pub artist: String,
    pub release_date: String,
}

impl From<Track> for Song {
    fn from(track: Track) -> Self {
        Self {
            id: track.id.to_string(),
            title: track.title,
            artist: track.artist.name,
            album: SongAlbum {
                title: track.album.title,
                cover_url: track.album.cover,
            },
            release_date: track.release_date,
        }
    }
}

impl From<SingleVideo> for Song {
    fn from(video: SingleVideo) -> Self {
        Self {
            id: video.id,
            title: video.title.expect("Video should have a title"),
            // TODO
            album: SongAlbum {
                title: String::new(),
                cover_url: video.thumbnail.expect("Video should have a thumbnail"),
            },
            artist: video.uploader.expect("Video should have an artist"),
            release_date: video.upload_date.expect("Video should have an upload date"),
        }
    }
}
