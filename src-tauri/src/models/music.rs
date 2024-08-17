use deezer::models::Track;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export)]
pub struct Album {
    pub title: String,
    pub cover_url: String,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SourceDownloader {
    Youtube,
    Deezer,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export)]
pub struct Song {
    #[ts(inline)]
    pub source: SourceDownloader,
    pub id: String,
    pub title: String,
    pub album: Album,
    pub artist: String,
    pub release_date: String,
}

impl From<Track> for Song {
    fn from(track: Track) -> Self {
        Self {
            source: SourceDownloader::Deezer,
            id: track.id.to_string(),
            title: track.title,
            artist: track.artist.name,
            album: Album {
                title: track.album.title,
                cover_url: track.album.cover,
            },
            release_date: track.release_date,
        }
    }
}

impl From<rusty_ytdl::search::Video> for Song {
    fn from(video: rusty_ytdl::search::Video) -> Self {
        let thumbnail = video
            .thumbnails
            .first()
            .map_or_else(String::default, |t| t.url.clone());

        Self {
            source: SourceDownloader::Youtube,
            id: video.id,
            title: video.title,
            album: Album {
                title: String::new(),
                cover_url: thumbnail,
            },
            artist: video.channel.name,
            release_date: video.uploaded_at.unwrap_or_default(),
        }
    }
}

impl From<rusty_ytdl::VideoDetails> for Song {
    fn from(video_details: rusty_ytdl::VideoDetails) -> Self {
        let artist = video_details
            .author
            .map(|author| author.name)
            .unwrap_or_default();
        let album = Album {
            title: String::new(),
            cover_url: video_details
                .thumbnails
                .first()
                .map(|t| t.url.clone())
                .unwrap_or_default(),
        };

        Self {
            source: SourceDownloader::Youtube,
            id: video_details.video_id,
            title: video_details.title,
            album,
            artist,
            release_date: video_details.upload_date,
        }
    }
}
