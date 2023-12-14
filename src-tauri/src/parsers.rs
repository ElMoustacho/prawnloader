use std::{num::ParseIntError, str::FromStr};

use url::Url;

use crate::downloaders::{DeezerId, YoutubeId, YoutubePlaylistId};

type ParseResult = std::result::Result<ParsedId, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid URL {0}")]
    InvalidURL(String),
    #[error("invalid ID {0}")]
    InvalidId(#[from] ParseIntError),
    #[error("no parse found for URL {0}")]
    NoParser(String),
    #[error("song with id {0} not found")]
    SongNotFound(DeezerId),
    #[error("album with id {0} not found")]
    AlbumNotFound(DeezerId),
    #[error("unable to parse url {0}")]
    UnparsableUrl(#[from] url::ParseError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedId {
    DeezerAlbum(DeezerId),
    DeezerTrack(DeezerId),
    YoutubeVideo(YoutubeId),
    YoutubePlaylist(YoutubePlaylistId),
}

pub async fn parse_id(url: &str) -> ParseResult {
    static PARSERS: [fn(url: &Url) -> ParseResult; 2] = [parse_deezer, parse_youtube];

    let url = normalize_url(url).await?;

    for parser in PARSERS {
        if let Ok(id) = parser(&url) {
            return Ok(id);
        }
    }

    Err(Error::NoParser(url.to_string()))
}

/// .Standardizes URLs to be understood by parsers.
///
/// # Errors
///
/// This function will return an error if the string is not a valid URL.
async fn normalize_url(url: &str) -> std::result::Result<Url, url::ParseError> {
    let mut url = Url::parse(url)?;

    match url.domain() {
        Some("music.youtube.com") | Some("m.youtube.com") => {
            let _ = url.set_host(Some("www.youtube.com"));
        }
        Some("deezer.page.link") => {
            url = follow_redirects(url).await;
        }
        _ => {}
    };

    Ok(url)
}

fn parse_youtube(url: &Url) -> ParseResult {
    let url = &url[..];

    if let Ok(id) = ytextract::video::Id::from_str(url) {
        return Ok(ParsedId::YoutubeVideo(id));
    }

    if let Ok(id) = ytextract::playlist::Id::from_str(url) {
        return Ok(ParsedId::YoutubePlaylist(id));
    }

    return Err(Error::InvalidURL("URL is not valid.".to_string()));
}

fn parse_deezer(url: &Url) -> ParseResult {
    let paths = url
        .path_segments()
        .ok_or(Error::InvalidURL("URL cannot be a base.".to_string()))?;
    let last_two: Vec<_> = paths.rev().take(2).collect();

    if last_two.len() < 2 {
        return Err(Error::InvalidURL(
            "Expected at least 2 path segments.".to_string(),
        ));
    }

    let id = last_two[0].parse::<DeezerId>()?;
    let track_album = last_two[1];

    match track_album {
        "track" => Ok(ParsedId::DeezerTrack(id)),
        "album" => Ok(ParsedId::DeezerAlbum(id)),
        _ => Err(Error::InvalidURL("Invalid {track_album}".to_string())),
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
            parse_id(DEEZER_ALBUM_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::DeezerAlbum(63318982)
        );
        assert_eq!(
            parse_id(DEEZER_TRACK_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::DeezerTrack(498467242)
        );
        assert_eq!(
            parse_id(DEEZER_PAGE_LINK_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::DeezerTrack(498467242)
        );
        assert_eq!(
            parse_id(YOUTUBE_VIDEO_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::YoutubeVideo("dQw4w9WgXcQ".parse().unwrap())
        );
        assert_eq!(
            parse_id(YOUTUBE_PLAYLIST_URL)
                .await
                .expect("URL should be valid"),
            ParsedId::YoutubePlaylist("PLv3TTBr1W_9tppikBxAE_G6qjWdBljBHJ".parse().unwrap())
        );
    }

    #[test]
    fn parses_deezer_album() {
        let url = Url::parse(DEEZER_ALBUM_URL).expect("URL should be valid");
        let parsed_id = parse_deezer(&url).expect("URL should be valid");
        let expected_id: u64 = 63318982;

        assert_eq!(parsed_id, ParsedId::DeezerAlbum(expected_id));
    }

    #[test]
    fn parses_deezer_track() {
        let url = Url::parse(DEEZER_TRACK_URL).expect("URL should be valid");
        let parsed_id = parse_deezer(&url).expect("URL should be valid");
        let expected_id: u64 = 498467242;

        assert_eq!(parsed_id, ParsedId::DeezerTrack(expected_id));
    }

    #[test]
    fn parses_youtube_video() {
        let url = Url::parse(YOUTUBE_VIDEO_URL).expect("URL should be valid");
        let parsed_id = parse_youtube(&url).expect("URL should be valid");
        let expected_id = "dQw4w9WgXcQ".parse().unwrap();

        assert_eq!(parsed_id, ParsedId::YoutubeVideo(expected_id));
    }

    #[test]
    fn parses_youtube_playlist() {
        let url = Url::parse(YOUTUBE_PLAYLIST_URL).expect("URL should be valid");
        let parsed_id = parse_youtube(&url).expect("URL should be valid");
        let expected_id = "PLv3TTBr1W_9tppikBxAE_G6qjWdBljBHJ".parse().unwrap();

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
