use anyhow::{bail, Result};
use async_trait::async_trait;

use crate::downloader::queue::Queue;

pub type ParserResult = Result<Queue>;

/// A parser used to transform URLs into downloadable songs with metadata.
#[async_trait]
pub trait SongParser: Send + Sync {
    async fn parse_url(&self, url: &String) -> ParserResult;
}

/// Manages multiple parsers and returns the first successful parse, else `Err`.
pub struct Parser {
    pub parsers: Vec<Box<dyn SongParser>>,
}

impl Parser {
    /// Create a new `Parser`.
    pub fn new(parsers: Vec<Box<dyn SongParser>>) -> Parser {
        Parser { parsers }
    }

    /// Parses an url and returns a result containing a vector of `QueueSong`
    /// if it matches with a parser.
    ///
    /// # Errors
    /// This function will return an error if the url passed doesn't match with any parser.
    pub(crate) async fn parse_url(&self, url: &String) -> ParserResult {
        for parser in self.parsers.iter() {
            if let Ok(downloadable_list) = parser.parse_url(&url).await {
                return Ok(downloadable_list);
            }
        }

        bail!("")
    }
}
