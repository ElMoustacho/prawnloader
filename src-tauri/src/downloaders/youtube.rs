use std::{
    env::{current_dir, current_exe},
    fs::{create_dir_all, DirBuilder},
    path::{Path, PathBuf},
    process::Stdio,
};

use color_eyre::eyre::{eyre, Result};
use crossbeam_channel::{unbounded, Sender};
use futures::future::join_all;
use tauri::api::path::download_dir;
use tempfile::TempDir;
use tokio::process::Command;
use url::Url;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

use crate::{
    config::Config,
    models::music::{Album, Item, Song},
};

use super::{
    replace_illegal_characters, DownloadRequest, ProgressEvent, YoutubeId, YoutubePlaylistId,
};

static DOWNLOAD_THREADS: u64 = 4;

struct YoutubeRequest(DownloadRequest, Config);

pub struct Downloader {
    download_tx: Sender<YoutubeRequest>,
}

impl Downloader {
    pub fn new(progress_tx: Sender<ProgressEvent>) -> Self {
        let (download_tx, download_rx) = unbounded::<YoutubeRequest>();

        for _ in 0..DOWNLOAD_THREADS {
            let _download_rx = download_rx.clone();
            let _progress_tx = progress_tx.clone();

            tokio::spawn(async move {
                while let Ok(request) = _download_rx.recv() {
                    let request_id = request.0.request_id;
                    let result = match request.0.item {
                        Item::YoutubeVideo { .. } => download_song(request, &_progress_tx).await,
                        Item::YoutubePlaylist { .. } => {
                            download_playlist(request, &_progress_tx).await
                        }

                        _ => continue,
                    };
                    let progress = match result {
                        Ok(_) => ProgressEvent::Finish(request_id),
                        Err(err) => ProgressEvent::DownloadError(request_id, err.to_string()),
                    };

                    _progress_tx.send(progress).unwrap();
                }
            });
        }

        Downloader { download_tx }
    }

    pub async fn request_download(&self, request: DownloadRequest, config: Config) {
        self.download_tx
            .send(YoutubeRequest(request, config))
            .expect("Channel should be open");
    }

    pub async fn get_song(&self, id: YoutubeId) -> Option<Song> {
        let video = match YoutubeDl::new(id).run_async().await.ok()? {
            YoutubeDlOutput::Playlist(playlist) => {
                unreachable!()
            }
            YoutubeDlOutput::SingleVideo(video) => video,
        };

        Some((*video).into())
    }

    pub async fn get_playlist(&self, id: YoutubePlaylistId) -> Option<Album> {
        unimplemented!()
    }
}

async fn download_song(request: YoutubeRequest, progress_tx: &Sender<ProgressEvent>) -> Result<()> {
    let YoutubeRequest(DownloadRequest { item, request_id }, config) = request;
    let Item::YoutubeVideo(song) = item else {
        unreachable!("Item should be YoutubeVideo.");
    };

    progress_tx.send(ProgressEvent::Start(request_id)).unwrap();

    YoutubeDl::new(song.id)
        .extract_audio(true)
        .extra_arg("-t")
        .extra_arg("mp3")
        .output_template("%(title).mp3")
        .download_to_async(config.download_folder)
        .await?;
    Ok(())
}

async fn download_playlist(
    request: YoutubeRequest,
    progress_tx: &Sender<ProgressEvent>,
) -> Result<()> {
    unimplemented!()
}

// // TODO: Use ffmpeg stream to split song
// async fn split_video_by_chapters(
//     video_details: VideoDetails,
//     file_format: String,
//     video_source_path: &Path,
//     dest_folder_path: &Path,
// ) {
//     for (index, chapter) in video_details.chapters.iter().enumerate() {
//         let output_filename = format_filename(&chapter.title, &file_format);
//         let output_path = dest_folder_path.join(output_filename);
//         let start = chapter.start_time.to_string();
//         let end;
//         if index != video_details.chapters.len() - 1 {
//             end = video_details
//                 .chapters
//                 .get(index + 1)
//                 .unwrap()
//                 .start_time
//                 .to_string();
//         } else {
//             end = video_details.length_seconds.clone();
//         }

//         let args = vec![
//             "-i",
//             video_source_path.to_str().unwrap(),
//             "-ss",
//             &start,
//             "-to",
//             &end,
//             "-c:a",
//             "copy",
//             output_path.to_str().unwrap(),
//         ];
//         Command::new("ffmpeg")
//             .args(args)
//             .stdout(Stdio::null())
//             .stderr(Stdio::null())
//             .kill_on_drop(true)
//             .spawn()
//             .unwrap()
//             .wait()
//             .await
//             .unwrap();
//     }
// }

fn format_filename(title: &str, extension: &str) -> String {
    format!("{}.{}", replace_illegal_characters(&title), extension)
}
