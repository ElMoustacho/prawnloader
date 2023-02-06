use crate::{music::Song, parser::parse_url};
use std::path::Path;

pub trait DownloadableSong {
    fn download(&self, dest_folder: &Path) -> Result<Box<Path>, ()>;
    fn get_song(&self) -> &Song;
}

pub fn download(
    url: impl Into<String>,
    dest_folder: &Path,
) -> Option<(Box<dyn DownloadableSong>, Box<Path>)> {
    let downloadable = parse_url(&url.into())?;

    if let Ok(dest_file) = downloadable.as_ref().download(dest_folder) {
        return Some((downloadable, dest_file));
    }

    None
}
