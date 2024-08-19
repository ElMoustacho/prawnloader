use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Clone, Default, EnumIter)]
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
}
