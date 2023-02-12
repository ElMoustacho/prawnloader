use crate::parser::{ParserResult, SongParser};
use crate::{
    downloader::DownloadableSong,
    music::{Album, Song},
};
use async_trait::async_trait;

pub(crate) struct YoutubeParser {}

impl SongParser for YoutubeParser {
    fn parse_url(&self, url: &String) -> ParserResult {
        match parse_video(url) {
            Err(_) => parse_playlist(url),
            result => result,
        }
    }
}

pub(crate) struct YoutubeDownloadable {
    song: Song,
    id: rustube::IdBuf,
}

#[async_trait]
impl DownloadableSong for YoutubeDownloadable {
    // TODO: Implement downloading
    async fn download(&self, dest_folder: &std::path::Path) -> Result<Box<std::path::Path>, ()> {
        todo!()
    }

    fn get_song(&self) -> &Song {
        &self.song
    }
}

// FIXME: Remove blocking operation
fn create_song_from_id(id: &rustube::Id) -> Result<Song, rustube::Error> {
    let video_details =
        tokio::task::block_in_place(|| rustube::blocking::Video::from_id(id.as_owned()))?
            .video_details();

    let album = Album {
        name: "album".to_string(),
        artist: video_details.author.to_string(),
        year: None,
        cover: None,
    };

    Ok(Song {
        title: video_details.title.to_string(),
        track: None,
        album,
    })
}

fn parse_video(url: &String) -> ParserResult {
    let id = rustube::Id::from_raw(url);

    if let Err(_) = id {
        return Err(());
    }

    let id = id.unwrap().as_owned();
    let song = create_song_from_id(&id);

    match song {
        Err(_) => Err(()),
        Ok(_) => Ok(vec![Box::new(YoutubeDownloadable {
            song: song.unwrap(),
            id,
        })]),
    }
}

// TODO: Implement playlist parsing
fn parse_playlist(url: &String) -> ParserResult {
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[tokio::test]
    async fn video_parse() {
        let parser = Parser::from(vec![Box::new(YoutubeParser {})]);

        parser
            .parse_url(&String::from("https://www.youtube.com/watch?v=ORofRTMg-iY"))
            .expect("Url should be parsable to a YT video.");

        parser
            .parse_url(&String::from(
                "https://music.youtube.com/watch?v=gAy5WZo9kts",
            ))
            .expect("Url should be parsable to a YT music song.");
    }

    #[tokio::test]
    async fn playlist_parse() {
        let parser = Parser::new();

        parser
            .parse_url(&String::from(
                "https://www.youtube.com/playlist?list=PLevurNKwl9HEcxa6K3dUoQ1jSBUUC2UxI",
            ))
            .expect("Url should be parsable to a YT playlist.");

        parser
            .parse_url(&String::from(
                "https://music.youtube.com/playlist?list=OLAK5uy_nSewatBUjTf3IO_DIqqMXn3ps_WbEAyi4",
            ))
            .expect("Url should be parsable to a YT music playlist.");
    }
}
