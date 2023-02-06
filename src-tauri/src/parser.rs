use crate::{downloader::DownloadableSong, youtube::YoutubeParser};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub(crate) static PARSERS: Lazy<Mutex<Vec<Box<dyn Parser>>>> =
    Lazy::new(|| Mutex::new(vec![Box::new(YoutubeParser {})]));

pub trait Parser: Send {
    fn parse_url(&self, url: &String) -> Option<Box<dyn DownloadableSong>>;
}

pub(crate) fn parse_url(url: &String) -> Option<Box<dyn DownloadableSong>> {
    for parser in PARSERS.lock().unwrap().iter() {
        if let Some(downloadable) = parser.parse_url(&url) {
            return Some(downloadable);
        }
    }

    None
}

pub fn add_parser(parser: Box<dyn Parser>) -> Result<(), ()> {
    let mutex = PARSERS.lock();

    match mutex {
        Ok(mut vec) => {
            vec.push(parser);
            Ok(())
        }
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_parse() {
        parse_url(&String::from(
            "https://music.youtube.com/watch?v=gAy5WZo9kts",
        ))
        .expect("Url should be parsable to a YT video.");

        parse_url(&String::from("https://www.youtube.com/watch?v=ORofRTMg-iY"))
            .expect("Url should be parsable to a YT video, converted from a YT music URL.");
    }

    #[test]
    fn playlist_parse() {
        parse_url(&String::from(
            "https://music.youtube.com/playlist?list=OLAK5uy_nSewatBUjTf3IO_DIqqMXn3ps_WbEAyi4",
        ))
        .expect("Url should be parsable to a YT playlist.");

        parse_url(&String::from(
            "https://www.youtube.com/playlist?list=PLevurNKwl9HEcxa6K3dUoQ1jSBUUC2UxI",
        ))
        .expect("Url should be parsable to a YT playlist, converted from a YT music URL.");
    }
}
