#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use prawnloader::{
    downloader::{DownloadRequest, Downloader, ProgressEvent},
    parsers::{normalize_url, ParsedId},
};
use tauri::{Manager, State};

struct AppState {
    downloader: Downloader,
}

#[tauri::command]
async fn request_download(url: String, state: State<'_, AppState>) -> Result<(), String> {
    let parsed_id = normalize_url(&url)
        .await
        .map_err(|_| format!("Unable to parse URL\"{url}\""))?;
    let download_request = match parsed_id {
        ParsedId::DeezerAlbum(id) => DownloadRequest::Album(id),
        ParsedId::DeezerTrack(id) => DownloadRequest::Song(id),
        ParsedId::YoutubeTrack(id) => todo!("YouTube not implemented yet."),
    };

    state.downloader.request_download(download_request);

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let downloader = Downloader::new();

            let handle = app.handle();
            let receiver = downloader.get_progress_rx();

            // Transfer any download event to the front-end
            std::thread::spawn(move || {
                while let Ok(event) = receiver.recv() {
                    let event_name = &event.to_string();
                    match event {
                        ProgressEvent::Queue(track)
                        | ProgressEvent::Start(track)
                        | ProgressEvent::Finish(track)
                        | ProgressEvent::DownloadError(track) => {
                            handle.emit_all(event_name, track).unwrap()
                        }
                        ProgressEvent::SongNotFoundError(id)
                        | ProgressEvent::AlbumNotFoundError(id) => {
                            handle.emit_all(event_name, id).unwrap()
                        }
                    };
                }
            });

            app.manage(AppState {
                downloader: downloader,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![request_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
