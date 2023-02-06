pub struct Song {
    pub title: String,
    pub track: Option<u16>,
    pub album: Album,
}

pub struct Album {
    pub name: String,
    pub artist: String,
    pub year: Option<u16>,
    pub cover: Option<Vec<u8>>,
}
