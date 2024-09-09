use deezer::models::Track;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export)]
pub struct Album {
    pub title: String,
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
    #[serde(rename_all = "camelCase")]
    DeezerAlbum { album: Album, merge_tracks: bool },
    #[serde(rename_all = "camelCase")]
    DeezerTrack { track: Song },
    #[serde(rename_all = "camelCase")]
    YoutubeVideo {
        video: Song,
        split_by_chapters: bool,
    },
    #[serde(rename_all = "camelCase")]
    YoutubePlaylist { playlist: Album },
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

impl From<rusty_ytdl::search::Video> for Song {
    fn from(video: rusty_ytdl::search::Video) -> Self {
        let thumbnail = video
            .thumbnails
            .first()
            .map_or_else(String::default, |t| t.url.clone());

        Self {
            id: video.id,
            title: video.title,
            album: SongAlbum {
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
        let album = SongAlbum {
            title: String::new(),
            cover_url: video_details
                .thumbnails
                .first()
                .map(|t| t.url.clone())
                .unwrap_or_default(),
        };

        Self {
            id: video_details.video_id,
            title: video_details.title,
            album,
            artist,
            release_date: video_details.upload_date,
        }
    }
}
