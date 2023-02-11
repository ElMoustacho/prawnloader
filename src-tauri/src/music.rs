use serde::Serialize;

#[derive(Serialize)]
pub struct Song {
    pub title: String,
    pub track: Option<u16>,
    pub album: Album,
}

#[derive(Serialize)]
pub struct Album {
    pub name: String,
    pub artist: String,
    pub year: Option<u16>,
    pub cover: Option<Vec<u8>>,
}
