#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use crossbeam_channel::{unbounded, Receiver, Sender};
use deezer::models::Track;
use prawnloader::{
    downloader::{Downloader, Id, ProgressEvent},
    parsers::{normalize_url, ParsedId},
};
use tauri::{Manager, State};

enum Event {
    ProgressEvent(ProgressEvent),
    AddToQueue(Track),
    RemoveFromQueue(Track),
}

struct AppState {
    downloader: Downloader,
    queue: Mutex<Vec<Track>>,
    event_tx: Sender<Event>,
    event_rx: Receiver<Event>,
}

#[tauri::command]
async fn add_to_queue(url: String, state: State<'_, AppState>) -> Result<(), String> {
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

    let mut queue = state.queue.lock().unwrap();
    for track in tracks {
        state
            .event_tx
            .send(Event::AddToQueue(track.clone()))
            .unwrap();
        queue.push(track);
    }

    Ok(())
}

#[tauri::command]
async fn request_download(track_id: Id, state: State<'_, AppState>) -> Result<(), String> {
    let mut queue = state.queue.lock().unwrap();
    let index = queue
        .iter()
        .position(|x| x.id == track_id)
        .ok_or("Track with id {track_id} not found")?;
    let track = queue.remove(index);

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

            // Event loop
            let (_event_tx, _event_rx) = (event_tx.clone(), event_rx.clone());
            std::thread::spawn(move || {
                // Transfer any download event to the main event loop
                while let Ok(progress_event) = progress_rx.recv() {
                    _event_tx
                        .send(Event::ProgressEvent(progress_event))
                        .unwrap();
                }

                // Transfer events to front-end
                while let Ok(event) = _event_rx.recv() {
                    println!("Add to queue");
                    match event {
                        Event::ProgressEvent(progress_event) => {
                            let event_name = &progress_event.to_string();
                            match progress_event {
                                ProgressEvent::Waiting(track)
                                | ProgressEvent::Start(track)
                                | ProgressEvent::Finish(track)
                                | ProgressEvent::DownloadError(track) => {
                                    handle.emit_all(event_name, track).unwrap()
                                }
                            };
                        }
                        Event::AddToQueue(track) => handle.emit_all("AddToQueue", track).unwrap(),
                        Event::RemoveFromQueue(track) => {
                            handle.emit_all("RemoveFromQueue", track).unwrap()
                        }
                    }
                }
            });

            app.manage(AppState {
                downloader,
                queue: Mutex::default(),
                event_tx,
                event_rx,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_to_queue, request_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
