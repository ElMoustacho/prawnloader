use anyhow::Result;
use async_trait::async_trait;
use futures::stream::FuturesOrdered;
use futures::{Future, StreamExt};
use regex::Regex;
use reqwest::Url;
use rustube::Video;
use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;

use crate::downloader::queue::{Queue, QueueSong};
use crate::music::{Album, Song};
use crate::parser::{ParserResult, SongParser};

pub(crate) struct YoutubeParser {
    ytextract_client: ytextract::Client,
}

impl YoutubeParser {
    pub fn new() -> Self {
        YoutubeParser {
            ytextract_client: ytextract::Client::new(),
        }
    }

    async fn parse_video(&self, id: ytextract::video::Id, track: Option<u16>) -> Result<Song> {
        let video = self.ytextract_client.video(id).await?;
        let song = build_song_from_video(video, track).await;

        Ok(song)
    }

    async fn parse_video_url(&self, url: &String) -> ParserResult {
        let id = url.parse::<ytextract::video::Id>()?;
        let song = self.parse_video(id, None).await?;

        Ok(vec![QueueSong::new(song, url.to_string(), download)])
    }

    async fn parse_playlist_url(&self, url: &String) -> ParserResult {
        let videos = self
            .ytextract_client
            .playlist(url.to_string().parse()?)
            .await?
            .videos();

        let mut track_counter: u16 = 1;
        let mut futures = FuturesOrdered::new();

        futures::pin_mut!(videos);

        while let Some(video) = videos.next().await {
            let Ok(video) = video else { continue };

            futures.push_back(self.parse_video(video.id(), Some(track_counter)));

            track_counter += 1;
        }

        let filtered_parses: Queue = futures
            .filter_map(|result| async {
                if let Ok(song) = result {
                    Some(QueueSong::new(song, url.to_string(), download))
                } else {
                    None
                }
            })
            .collect()
            .await;

        Ok(filtered_parses)
    }
}

#[async_trait]
impl SongParser for YoutubeParser {
    async fn parse_url(&self, url: &String) -> ParserResult {
        let url = &parse_yt_music_to_yt(url);

        self.parse_video_url(url)
            .await
            .or(self.parse_playlist_url(url).await)
    }
}

fn download(
    url: &str,
    dest_folder: PathBuf,
) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> {
    Box::pin(async {
        let url = Url::from_str(url)?;
        let video = Video::from_url(&url).await?;

        video
            .best_audio()
            .unwrap()
            .download_to_dir(dest_folder.clone())
            .await?;

        Ok(dest_folder)
    })
}

/// Creates a new `Song` instance from a `ytextract::Video`.
// TODO: Add album name
async fn build_song_from_video(video: ytextract::Video, track: Option<u16>) -> Song {
    let album = Album {
        name: "album".to_string(),
        artist: video.channel().name().to_string(),
        year: video.date().to_string()[0..4].parse().ok(),
        cover: get_thumbnail(&video).await,
    };

    Song {
        title: video.title().to_string(),
        track,
        album,
    }
}

/// Gets a thumbnail from a video as a vector of bytes.
async fn get_thumbnail(video: &ytextract::Video) -> Option<Vec<u8>> {
    let thumbnails = video.thumbnails();

    if video.thumbnails().len() < 1 {
        return None;
    }

    let thumbnail = &thumbnails.first().unwrap().url.to_string();
    if let Ok(response) = reqwest::get(thumbnail).await {
        match response.bytes().await {
            Ok(bytes) => Some(bytes.to_vec()),
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Changes a music.youtube.com url to a www.youtube.com url.
fn parse_yt_music_to_yt(url: &str) -> String {
    let regex = Regex::new(r"(?P<before>https?://)?music(?P<after>\.youtube\.com.*)").unwrap();
    regex.replace_all(url, "${before}www${after}").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[tokio::test]
    async fn video_parse() {
        let parser = Parser::new(vec![Box::new(YoutubeParser::new())]);

        parser
            .parse_url(&String::from("https://www.youtube.com/watch?v=ORofRTMg-iY"))
            .await
            .expect("Url should be parsable to a YT video.");

        parser
            .parse_url(&String::from(
                "https://music.youtube.com/watch?v=gAy5WZo9kts",
            ))
            .await
            .expect("Url should be parsable to a YT music song.");
    }

    #[tokio::test]
    async fn playlist_parse() {
        let parser = Parser::new(vec![Box::new(YoutubeParser::new())]);

        parser
            .parse_url(&String::from(
                "https://www.youtube.com/playlist?list=PLevurNKwl9HEcxa6K3dUoQ1jSBUUC2UxI",
            ))
            .await
            .expect("Url should be parsable to a YT playlist.");

        parser
            .parse_url(&String::from(
                "https://music.youtube.com/playlist?list=OLAK5uy_nSewatBUjTf3IO_DIqqMXn3ps_WbEAyi4",
            ))
            .await
            .expect("Url should be parsable to a YT music playlist.");
    }
}
