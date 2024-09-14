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

#[derive(TS, Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Config {
    #[ts(inline)]
    pub youtube_format: YoutubeFormat,
    pub split_by_chapters_default: bool,
}
