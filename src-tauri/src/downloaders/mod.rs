use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::music::Item;

pub mod deezer;
pub mod youtube;

pub type DeezerId = u64;
pub type YoutubePlaylistId = String;
pub type YoutubeId = String;

#[derive(Debug, Clone, Serialize, strum_macros::Display)]
pub enum ProgressEvent {
    Waiting(Uuid),
    Start(Uuid),
    Finish(Uuid),
    DownloadError(Uuid, String),
}

#[derive(TS, Serialize, Deserialize)]
#[serde(rename = "QueueItem")]
#[ts(export)]
pub struct DownloadRequest {
    #[ts(type = "string")]
    pub request_id: Uuid,
    pub item: Item,
}

/// Replaces illegal characters for a Windows file.
fn replace_illegal_characters(str: &str) -> String {
    static ILLEGAL_CHARACTERS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

    str.chars()
        .filter(|char| !ILLEGAL_CHARACTERS.contains(char))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_invalid_chars() {
        let file_name = "AC/DC - Thunderstruck.mp3";

        assert_eq!(
            "ACDC - Thunderstruck.mp3",
            replace_illegal_characters(file_name)
        );

        let file_name = "<>:\"/\\|?* - Test.mp3";

        assert_eq!(" - Test.mp3", replace_illegal_characters(file_name));
    }
}
