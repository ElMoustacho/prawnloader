use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Song {
    pub title: String,
    pub track: Option<u16>,
    pub album: Album,
}

#[derive(Serialize, Clone)]
pub struct Album {
    pub name: String,
    pub artist: String,
    pub year: Option<u16>,
    pub cover: Option<Vec<u8>>,
}
