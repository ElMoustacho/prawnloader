#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use prawnloader::{
    downloader::{Downloader, Id},
    events::Event,
    models::music::Song,
    parsers::{normalize_url, ParsedId},
};
use tauri::{Manager, State};

struct AppState {
    downloader: Downloader,
    event_tx: Sender<Event>,
    event_rx: Receiver<Event>,
}

#[tauri::command]
async fn get_songs(url: String, state: State<'_, AppState>) -> Result<Vec<Song>, String> {
    let parsed_id = normalize_url(&url)
        .await
        .map_err(|_| format!("Unable to parse URL\"{url}\""))?;
    let tracks = match parsed_id {
        ParsedId::DeezerAlbum(id) => state
            .downloader
            .get_album_tracks(id)
            .await
            .ok_or(format!("Invalid album id {id}"))?,
        ParsedId::DeezerTrack(id) => state
            .downloader
            .get_track(id)
            .await
            .map(|x| vec![x])
            .ok_or(format!("Invalid track id {id}"))?,
        ParsedId::YoutubeTrack(id) => todo!("YouTube not implemented yet."),
    };

    let songs = tracks.into_iter().map(|track| Song::from(track)).collect();

    Ok(songs)
}

#[tauri::command]
async fn request_download(track_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let track_id: Id = track_id
        .parse()
        .map_err(|_| "Id could not be converter to integer")?;

    let track = state
        .downloader
        .get_track(track_id)
        .await
        .ok_or(format!("Unable to find track with id {track_id}"))?;

    state.downloader.request_download(track);

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let downloader = Downloader::new();
            let (event_tx, event_rx) = unbounded();

            let handle = app.handle();
            let progress_rx = downloader.get_progress_rx();

            let (_event_tx, _event_rx) = (event_tx.clone(), event_rx.clone());

            // Transfer any download event to the main event loop
            std::thread::spawn(move || {
                while let Ok(progress_event) = progress_rx.recv() {
                    _event_tx.send(Event::from(progress_event)).unwrap();
                }
            });

            // Transfer events to front-end
            std::thread::spawn(move || {
                while let Ok(event) = _event_rx.recv() {
                    let event_name = &event.to_string()[..];
                    match event {
                        Event::Waiting(track) => {
                            handle.emit_all(event_name, Song::from(track)).unwrap()
                        }
                        Event::Start(track) => {
                            handle.emit_all(event_name, Song::from(track)).unwrap()
                        }
                        Event::Finish(track) => {
                            handle.emit_all(event_name, Song::from(track)).unwrap()
                        }
                        Event::DownloadError(track) => {
                            handle.emit_all(event_name, Song::from(track)).unwrap()
                        }
                        Event::RemoveFromQueue(track) => {
                            handle.emit_all(event_name, track).unwrap()
                        }
                    }
                }
            });

            app.manage(AppState {
                downloader,
                event_tx,
                event_rx,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_songs, request_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
