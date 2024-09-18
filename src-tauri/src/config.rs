use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
    str::FromStr,
};

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer};
use strum_macros::Display;
use tauri::api::path::data_dir;
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

const CONFIG_FILENAME: &str = "config.json";

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

impl Config {
    pub fn load() -> Result<Config> {
        let config = match std::fs::read(config_folder().join(CONFIG_FILENAME)) {
            Ok(bytes) => serde_json::from_slice::<Config>(&bytes)?,
            Err(_) => Config::default(),
        };

        println!("{config:?}");

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_file = File::create(config_folder().join(CONFIG_FILENAME))?;
        let mut serializer =
            Serializer::with_formatter(config_file, PrettyFormatter::with_indent(b"\t"));

        create_dir_all(config_folder())?;
        self.serialize(&mut serializer)
            .expect("Serialization should not fail.");

        Ok(())
    }
}

fn config_folder() -> PathBuf {
    if let Some(config_file_path) = data_dir() {
        config_file_path.join("prawnloader")
    } else {
        PathBuf::from_str("./").unwrap()
    }
}
