// Disable these modules for now
// mod deezer;
// mod youtube;

use std::{collections::HashMap, num::ParseIntError};

use color_eyre::eyre::eyre;
use url::Url;

use crate::downloader::DeezerId;

type Result = std::result::Result<ParsedId, Error>;
type YoutubeId = String;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid URL {0}")]
    InvalidURL(#[from] color_eyre::eyre::Error),
    #[error("invalid ID {0}")]
    InvalidId(#[from] ParseIntError),
    #[error("no parse found for URL {0}")]
    NoParser(String),
    #[error("song with id {0} not found")]
    SongNotFoundError(DeezerId),
    #[error("album with id {0} not found")]
    AlbumNotFoundError(DeezerId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedId {
    DeezerAlbum(DeezerId),
    DeezerTrack(DeezerId),
    YoutubeVideo(YoutubeId),
    YoutubePlaylist(YoutubeId),
}

pub async fn normalize_url(url: &str) -> Result {
    let url = Url::parse(url).map_err(|err| color_eyre::eyre::Error::from(err))?;

    match url.domain() {
        Some("www.youtube.com") => Ok(parse_youtube(url)?),
        Some("www.deezer.com") => Ok(parse_deezer(url)?),
        Some("deezer.page.link") => {
            let url = follow_redirects(url).await;
            Ok(parse_deezer(url)?)
        }
        _ => Err(Error::NoParser(url.to_string())),
    }
}

fn parse_youtube(url: Url) -> Result {
    let query_pairs: HashMap<_, _> = url.query_pairs().collect();

    let id =  match url.path() {
        "/watch" if query_pairs.contains_key("v") => ParsedId::YoutubeVideo(query_pairs.get("v").unwrap().to_string()),
        "/playlist" if query_pairs.contains_key("list") => ParsedId::YoutubePlaylist(query_pairs.get("list").unwrap().to_string()),
        _ => {
            return Err(Error::InvalidURL(eyre!(
                "URL must be in the format \"www.youtube.com/watch?v=abcdefg\" or \"www.youtube.com/playlist?list=abcdefg\""
            )))
        }
    };

    Ok(id)
}

fn parse_deezer(url: Url) -> Result {
    let paths = url.path_segments().ok_or(eyre!("Invalid segments"))?;
    let last_two: Vec<_> = paths.rev().take(2).collect();

    if last_two.len() < 2 {
        return Err(Error::InvalidURL(eyre!(
            "Expected at least 2 path segments."
        )));
    }

    let id = last_two[0].parse::<DeezerId>()?;
    let track_album = last_two[1];

    match track_album {
        "track" => Ok(ParsedId::DeezerTrack(id)),
        "album" => Ok(ParsedId::DeezerAlbum(id)),
        _ => Err(Error::InvalidURL(eyre!("Invalid {track_album}"))),
    }
}

pub async fn follow_redirects(url: Url) -> Url {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();
    response.url().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    static YOUTUBE_VIDEO_URL: &str = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    static YOUTUBE_PLAYLIST_URL: &str =
        "https://www.youtube.com/playlist?list=PLv3TTBr1W_9tppikBxAE_G6qjWdBljBHJ";
    static DEEZER_ALBUM_URL: &str = "https://www.deezer.com/fr/album/63318982";
    static DEEZER_TRACK_URL: &str = "https://www.deezer.com/fr/track/498467242";
    static DEEZER_PAGE_LINK_URL: &str = "https://deezer.page.link/CWiy1BS7UeZqAnt56";

    #[tokio::test]
    async fn finds_correct_parser() {
        assert_eq!(
            normalize_url(DEEZER_ALBUM_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::DeezerAlbum(63318982)
        );
        assert_eq!(
            normalize_url(DEEZER_TRACK_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::DeezerTrack(498467242)
        );
        assert_eq!(
            normalize_url(YOUTUBE_VIDEO_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::YoutubeVideo("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn parses_deezer_album() {
        let url = Url::parse(DEEZER_ALBUM_URL).expect("URL should be valid");
        let parsed_id = parse_deezer(url).expect("URL should be valid");
        let expected_id: u64 = 63318982;

        assert_eq!(parsed_id, ParsedId::DeezerAlbum(expected_id));
    }

    #[test]
    fn parses_deezer_track() {
        let url = Url::parse(DEEZER_TRACK_URL).expect("URL should be valid");
        let parsed_id = parse_deezer(url).expect("URL should be valid");
        let expected_id: u64 = 498467242;

        assert_eq!(parsed_id, ParsedId::DeezerTrack(expected_id));
    }

    #[test]
    fn parses_youtube_video() {
        let url = Url::parse(YOUTUBE_VIDEO_URL).expect("URL should be valid");
        let parsed_id = parse_youtube(url).expect("URL should be valid");
        let expected_id = "dQw4w9WgXcQ".to_string();

        assert_eq!(parsed_id, ParsedId::YoutubeVideo(expected_id));
    }

    #[test]
    fn parses_youtube_playlist() {
        let url = Url::parse(YOUTUBE_PLAYLIST_URL).expect("URL should be valid");
        let parsed_id = parse_youtube(url).expect("URL should be valid");
        let expected_id = "PLv3TTBr1W_9tppikBxAE_G6qjWdBljBHJ".to_string();

        assert_eq!(parsed_id, ParsedId::YoutubePlaylist(expected_id));
    }

    #[tokio::test]
    async fn follows_redirects() {
        let new_url =
            &follow_redirects(Url::parse(DEEZER_PAGE_LINK_URL).expect("URL should be valid")).await;
        let target_url =
            &follow_redirects(Url::parse(DEEZER_TRACK_URL).expect("URL should be valid")).await;

        assert_eq!(new_url.domain(), target_url.domain());
        assert_eq!(new_url.path(), target_url.path());
    }
}
