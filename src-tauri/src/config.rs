use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Clone, Default, Display)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum YoutubeFormat {
    #[default]
    MP3,
    WEBM,
    WAV,
    OGG,
}

// TODO: Persist and load config
#[derive(TS, Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Config {
    // General
    // TODO: Use this value
    pub group_songs_in_folder: bool,
    // TODO: Use this value
    pub download_folder: PathBuf,

    // Deezer
    pub deezer_merge_tracks: bool,

    // Youtube
    #[ts(inline)]
    pub youtube_format: YoutubeFormat,
    pub youtube_split_chapters: bool,
}
