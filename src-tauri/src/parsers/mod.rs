// Disable these modules for now
// mod deezer;
// mod youtube;

use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use url::Url;

use crate::downloader::Id;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedId {
    DeezerAlbum(Id),
    DeezerTrack(Id),
    YoutubeTrack(String),
}

pub async fn normalize_url(url: &str) -> Result<ParsedId> {
    let url = Url::parse(url)?;

    match url.domain() {
        Some("www.youtube.com") => Ok(parse_youtube(url)?),
        Some("www.deezer.com") => Ok(parse_deezer(url)?),
        Some("deezer.page.link") => {
            let url = follow_redirects(url).await;
            Ok(parse_deezer(url)?)
        }
        _ => Err(eyre!("No parser matches this URL")),
    }
}

fn parse_youtube(url: Url) -> Result<ParsedId> {
    todo!()
}

fn parse_deezer(url: Url) -> Result<ParsedId> {
    let paths = url.path_segments().ok_or(eyre!("Invalid segments"))?;
    let last_two: Vec<_> = paths.rev().take(2).collect();

    if last_two.len() < 2 {
        bail!("Expected at least 2 path segments.")
    }

    let id = last_two[0].parse::<Id>()?;
    let track_album = last_two[1];

    match track_album {
        "track" => Ok(ParsedId::DeezerTrack(id)),
        "album" => Ok(ParsedId::DeezerAlbum(id)),
        _ => Err(eyre!("Invalid {track_album}")),
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
            ParsedId::YoutubeTrack("dQw4w9WgXcQ".to_string())
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
