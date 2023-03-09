#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use prawnloader::downloader::{self, Event};
use tauri::{api::path::download_dir, Manager, State};
use tokio::sync::Mutex;

struct AppState {
    downloader: Mutex<downloader::Downloader>,
}

#[tauri::command]
async fn add_to_queue(url: String, state: State<'_, AppState>) -> Result<(), String> {
    let result = state.downloader.lock().await.add_to_queue(url).await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_from_queue(id: usize, state: State<'_, AppState>) -> Result<(), ()> {
    let result = state.downloader.lock().await.remove_from_queue(id);

    result
}

#[tauri::command]
async fn clear_queue(state: State<'_, AppState>) -> Result<(), ()> {
    state.downloader.lock().await.clear_queue();

    Ok(())
}

#[tauri::command]
async fn download(id: usize, state: State<'_, AppState>) -> Result<(), String> {
    let download_dir = &download_dir().unwrap();

    state
        .downloader
        .lock()
        .await
        .start_download(id, download_dir)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn download_queue(state: State<'_, AppState>) -> Result<(), String> {
    let download_dir = &download_dir().unwrap();

    state
        .downloader
        .lock()
        .await
        .start_download_queue(download_dir);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let downloader = downloader::Downloader::new();

            let handle = app.handle();
            let receiver = downloader.get_event_receiver();

            std::thread::spawn(move || {
                while let Ok(event) = receiver.recv() {
                    match event {
                        Event::UpdateQueue(queue) => {
                            handle.emit_all("queue_update", queue).unwrap()
                        }

                        Event::DownloadComplete(file_location) => {
                            handle.emit_all("download_complete", file_location).unwrap()
                        }

                        Event::DownloadStarted(song) => {
                            handle.emit_all("download_started", song).unwrap()
                        }
                    };
                }
            });

            app.manage(AppState {
                downloader: Mutex::new(downloader),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_to_queue,
            remove_from_queue,
            clear_queue,
            download,
            download_queue,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
