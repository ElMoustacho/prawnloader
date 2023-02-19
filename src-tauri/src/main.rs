#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use prawnloader::downloader::{self, Event};
use std::sync::Arc;
use tauri::{api::path::download_dir, Manager, State};
use tokio::sync::Mutex;

struct AppState {
    downloader: Arc<Mutex<downloader::Downloader>>,
}

#[tauri::command]
async fn add_to_queue(url: String, state: State<'_, AppState>) -> Result<(), ()> {
    let result = state.downloader.lock().await.add_to_queue(url).await;

    result
}

#[tauri::command]
async fn remove_from_queue(id: usize, state: State<'_, AppState>) -> Result<(), ()> {
    let result = state.downloader.lock().await.remove_from_queue(id);

    result
}

#[tauri::command]
async fn download(index: usize, state: State<'_, AppState>) -> Result<(), ()> {
    let download_dir = &download_dir().unwrap();

    let result = state.downloader.lock().await.download(index, download_dir);

    result
}

#[tauri::command]
async fn download_queue(state: State<'_, AppState>) -> Result<(), ()> {
    let download_dir = &download_dir().unwrap();

    state.downloader.lock().await.download_queue(download_dir);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let downloader = downloader::Downloader::new();

            let handle = app.handle();

            downloader
                .event_manager
                .lock()
                .unwrap()
                .add_callback(move |event| match event {
                    Event::AddToQueue(queue) | Event::RemoveFromQueue(queue) => {
                        handle.emit_all("queue_update", queue).unwrap()
                    }

                    Event::DownloadComplete(file_location) => {
                        handle.emit_all("download_complete", file_location).unwrap()
                    }

                    Event::DownloadStarted(song) => {
                        handle.emit_all("download_started", song).unwrap()
                    }
                });

            app.manage(AppState {
                downloader: Arc::new(Mutex::new(downloader)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_to_queue,
            remove_from_queue,
            download,
            download_queue
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
