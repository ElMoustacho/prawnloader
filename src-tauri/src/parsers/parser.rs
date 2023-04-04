use anyhow::{bail, Result};
use async_trait::async_trait;
use once_cell::sync::Lazy;

use crate::downloader::queue::Queue;

use super::deezer::DeezerParser;
use super::youtube::YoutubeParser;

pub type ParserResult = Result<Queue>;

static PARSERS: Lazy<[Box<dyn SongParser>; 2]> = Lazy::new(|| {
    [
        Box::new(YoutubeParser::new()),
        Box::new(DeezerParser::new()),
    ]
});

/// A parser used to transform URLs into downloadable songs with metadata.
#[async_trait]
pub trait SongParser: Send + Sync {
    async fn parse_url(&self, url: &String) -> ParserResult;
}

/// Parses an url and returns a result containing a vector of `QueueSong`
/// if it matches with a parser.
///
/// # Errors
/// This function will return an error if the url passed doesn't match with any parser.
pub async fn parse_url(url: &String) -> ParserResult {
    for parser in PARSERS.iter() {
        if let Ok(downloadable_list) = parser.parse_url(&url).await {
            return Ok(downloadable_list);
        }
    }

    bail!("")
}
