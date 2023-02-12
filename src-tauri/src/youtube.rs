use crate::parser::{ParserResult, SongParser};
use crate::{
    downloader::DownloadableSong,
    music::{Album, Song},
};
use async_trait::async_trait;

pub(crate) struct YoutubeParser {}

#[async_trait]
impl SongParser for YoutubeParser {
    async fn parse_url(&self, url: &String) -> ParserResult {
        parse_video(url).await.or(parse_playlist(url).await)
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

async fn create_song_from_id(id: rustube::IdBuf) -> Result<Song, rustube::Error> {
    let video_details = rustube::Video::from_id(id).await?.video_details();

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

async fn parse_video(url: &String) -> ParserResult {
    let id = rustube::Id::from_raw(url);

    if let Err(_) = id {
        return Err(());
    }

    let id = id.unwrap().as_owned();
    let song = create_song_from_id(id.clone()).await;

    match song {
        Err(_) => Err(()),
        Ok(_) => Ok(vec![Box::new(YoutubeDownloadable {
            song: song.unwrap(),
            id,
        })]),
    }
}

// TODO: Implement playlist parsing
async fn parse_playlist(url: &String) -> ParserResult {
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
        let parser = Parser::new();

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
