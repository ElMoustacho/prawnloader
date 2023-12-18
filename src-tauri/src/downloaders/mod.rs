use serde::Serialize;

use crate::models::music::Song;

pub mod deezer;
pub mod youtube;

pub type DeezerId = u64;
pub type YoutubePlaylistId = String;
pub type YoutubeId = String;

#[derive(Debug, Clone, Serialize, strum_macros::Display)]
pub enum ProgressEvent {
    Waiting(Song),
    Start(Song),
    Finish(Song),
    DownloadError(Song),
}

#[derive(Debug, strum_macros::Display)]
pub enum DownloadStatus {
    Downloading,
    Inactive,
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
