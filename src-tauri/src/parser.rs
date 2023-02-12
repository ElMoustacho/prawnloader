use crate::{downloader::DownloadableSong, youtube::YoutubeParser};
use tokio::sync::Mutex;

pub type ParserResult = Result<Vec<Box<dyn DownloadableSong>>, ()>;

pub trait SongParser: Send {
    fn parse_url(&self, url: &String) -> ParserResult;
}

pub struct Parser {
    parsers: Mutex<Vec<Box<dyn SongParser>>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            parsers: Mutex::new(vec![Box::new(YoutubeParser {})]),
        }
    }

    /**
    Parses an url and returns a result containing a vector of `DownloadableSong`
    if it matches with a parser.

    # Errors
    This function will return an error if the url passed doesn't match with any parser.
    */
    pub(crate) fn parse_url(&self, url: &String) -> ParserResult {
        let parsers = tokio::task::block_in_place(|| self.parsers.blocking_lock());

        for parser in parsers.iter() {
            if let Ok(downloadable_list) = parser.parse_url(&url) {
                return Ok(downloadable_list);
            }
        }

        Err(())
    }

    pub fn add_parser(&self, parser: Box<dyn SongParser>) {
        tokio::task::block_in_place(|| self.parsers.blocking_lock().push(parser));
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_parse() {
        let parser = Parser::new();

        parser
            .parse_url(&String::from(
                "https://music.youtube.com/watch?v=gAy5WZo9kts",
            ))
            .expect("Url should be parsable to a YT video.");

        parser
            .parse_url(&String::from("https://www.youtube.com/watch?v=ORofRTMg-iY"))
            .expect("Url should be parsable to a YT video, converted from a YT music URL.");
    }

    #[test]
    fn playlist_parse() {
        let parser = Parser::new();

        parser
            .parse_url(&String::from(
                "https://music.youtube.com/playlist?list=OLAK5uy_nSewatBUjTf3IO_DIqqMXn3ps_WbEAyi4",
            ))
            .expect("Url should be parsable to a YT playlist.");

        parser
            .parse_url(&String::from(
                "https://www.youtube.com/playlist?list=PLevurNKwl9HEcxa6K3dUoQ1jSBUUC2UxI",
            ))
            .expect("Url should be parsable to a YT playlist, converted from a YT music URL.");
    }
}
