use crate::downloader::DownloadableSong;
use crate::music::Album;
use crate::music::Song;
use crate::parser::Parser;

pub(crate) struct YoutubeParser {}

impl Parser for YoutubeParser {
    fn parse_url(&self, url: &String) -> Option<Box<dyn DownloadableSong>> {
        if let Ok(id) = rustube::Id::from_raw(url) {
            let id = id.as_owned();
            let song = create_song_from_id(&id);

            if let Err(_) = song {
                return None;
            }

            return Some(Box::new(YoutubeDownloadable {
                song: song.unwrap(),
                id,
            }));
        }

        None
    }
}

pub(crate) struct YoutubeDownloadable {
    song: Song,
    id: rustube::IdBuf,
}

impl DownloadableSong for YoutubeDownloadable {
    fn download(&self, dest_folder: &std::path::Path) -> Result<Box<std::path::Path>, ()> {
        let path =
            rustube::blocking::download_worst_quality(&self.id.watch_url().to_string()).unwrap();

        Ok(Box::from(path))
    }

    fn get_song(&self) -> &Song {
        &self.song
    }
}

fn create_song_from_id(id: &rustube::Id) -> Result<Song, rustube::Error> {
    let album = Album {
        name: "ok".to_string(),
        artist: "author".to_string(),
        year: None,
        cover: None,
    };

    Ok(Song {
        title: "title".to_string(),
        track: None,
        album,
    })
}
