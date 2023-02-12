use async_trait::async_trait;

use crate::{downloader::DownloadableSong, youtube::YoutubeParser};

pub type ParserResult = Result<Vec<Box<dyn DownloadableSong>>, ()>;

/**
A parser used to transform URLs into downloadable songs with metadata.
 */
#[async_trait]
pub trait SongParser: Send + Sync {
    async fn parse_url(&self, url: &String) -> ParserResult;
}

/**
Manages multiple parsers and returns the first successful parse, else `Err`.
 */
pub struct Parser {
    pub parsers: Vec<Box<dyn SongParser>>,
}

impl Parser {
    /**
    Create a new `Parser` with a default list of parsers.
    */
    pub fn new() -> Parser {
        Parser {
            parsers: vec![Box::new(YoutubeParser {})],
        }
    }

    /**
    Create a new `Parser` with a custom list of parsers.
    */
    pub fn from(parsers: Vec<Box<dyn SongParser>>) -> Parser {
        Parser {
            parsers: Vec::from(parsers),
        }
    }

    /**
    Parses an url and returns a result containing a vector of `DownloadableSong`
    if it matches with a parser.

    # Errors
    This function will return an error if the url passed doesn't match with any parser.
    */
    pub(crate) async fn parse_url(&self, url: &String) -> ParserResult {
        for parser in self.parsers.iter() {
            if let Ok(downloadable_list) = parser.parse_url(&url).await {
                return Ok(downloadable_list);
            }
        }

        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_parsers() {
        let parser = Parser::new();

        let urls = [
            // Deezer
            "https://www.deezer.com/fr/track/597403742",
            "https://deezer.page.link/mZsk7WU6P4r4h3nA8",
            "https://www.deezer.com/fr/album/345755977",
            "https://www.deezer.com/fr/playlist/10575085742",
            // YouTube Music
            "https://music.youtube.com/watch?v=gAy5WZo9kts",
            "https://music.youtube.com/playlist?list=OLAK5uy_nSewatBUjTf3IO_DIqqMXn3ps_WbEAyi4",
            // YouTube
            "https://www.youtube.com/watch?v=ORofRTMg-iY",
            "https://www.youtube.com/playlist?list=PLevurNKwl9HEcxa6K3dUoQ1jSBUUC2UxI",
        ];

        for url in urls {
            parser
                .parse_url(&url.to_string())
                .await
                .expect(&format!("Url '{}' should be parsable.", url)[..]);
        }
    }
}
