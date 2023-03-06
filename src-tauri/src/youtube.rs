use crate::parser::{ParserResult, SongParser};
use crate::{
    downloader::DownloadableSong,
    music::{Album, Song},
};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::FuturesOrdered;
use futures::StreamExt;
use regex::Regex;
use rustube::Video;
use std::path::PathBuf;

pub(crate) struct YoutubeParser {
    ytextract_client: ytextract::Client,
}

impl YoutubeParser {
    pub fn new() -> Self {
        YoutubeParser {
            ytextract_client: ytextract::Client::new(),
        }
    }

    async fn parse_video(&self, id: ytextract::video::Id) -> Result<YoutubeDownloadable> {
        let video = self.ytextract_client.video(id).await?;
        let song = build_song_from_video(video).await?;

        let id = rustube::Id::from_raw(&id)?.as_owned();

        Ok(YoutubeDownloadable { song, id })
    }

    async fn parse_video_url(&self, url: &String) -> ParserResult {
        let id = url.parse::<ytextract::video::Id>()?;
        let video = self.parse_video(id).await?;

        Ok(vec![Box::new(video)])
    }

    async fn parse_playlist_url(&self, url: &String) -> ParserResult {
        let videos = self
            .ytextract_client
            .playlist(url.to_string().parse()?)
            .await?
            .videos();

        let mut futures = FuturesOrdered::new();

        futures::pin_mut!(videos);

        while let Some(video) = videos.next().await {
            let Ok(video) = video else { continue };

            futures.push_back(self.parse_video(video.id()));
        }

        let filtered_parses: Vec<Box<dyn DownloadableSong>> = futures
            .filter_map(|result| async {
                if let Ok(result) = result {
                    Some(Box::new(result) as _)
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

pub(crate) struct YoutubeDownloadable {
    song: Song,
    id: rustube::IdBuf,
}

#[async_trait]
impl DownloadableSong for YoutubeDownloadable {
    async fn download(&self, dest_folder: PathBuf) -> Result<PathBuf> {
        let video = Video::from_id(self.id.to_owned()).await?;

        video
            .best_audio()
            .unwrap()
            .download_to_dir(dest_folder.clone())
            .await?;

        Ok(dest_folder)
    }

    fn get_song(&self) -> &Song {
        &self.song
    }
}

// TODO: Add album name & year & track
async fn build_song_from_video(video: ytextract::Video) -> Result<Song> {
    let thumbnail = get_thumbnail(&video).await;

    let album = Album {
        name: "album".to_string(),
        artist: video.channel().name().to_string(),
        year: None,
        cover: thumbnail,
    };

    Ok(Song {
        title: video.title().to_string(),
        track: None,
        album,
    })
}

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
