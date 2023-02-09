#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use prawnloader::downloader;
use std::sync::{Arc, Mutex};
use tauri::State;

struct AppState {
    downloader: Arc<Mutex<downloader::Downloader>>,
}

#[tauri::command]
async fn add_to_queue(url: String, state: State<'_, AppState>) -> Result<(), ()> {
    state.downloader.lock().unwrap().add_to_queue(url)
}

#[tauri::command]
async fn remove_from_queue(id: usize, state: State<'_, AppState>) -> Result<(), ()> {
    state.downloader.lock().unwrap().remove_from_queue(id)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            downloader: Arc::new(Mutex::new(downloader::Downloader::new())),
        })
        .invoke_handler(tauri::generate_handler![add_to_queue, remove_from_queue])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_parsers() {
        let mut downloader = downloader::Downloader::new();

        let urls = [
            "https://www.deezer.com/fr/track/597403742",
            "https://deezer.page.link/mZsk7WU6P4r4h3nA8",
            "https://www.deezer.com/fr/album/345755977",
            "https://www.deezer.com/fr/playlist/10575085742",
            "https://music.youtube.com/watch?v=gAy5WZo9kts",
            "https://music.youtube.com/playlist?list=OLAK5uy_nSewatBUjTf3IO_DIqqMXn3ps_WbEAyi4",
            "https://www.youtube.com/watch?v=ORofRTMg-iY",
            "https://www.youtube.com/playlist?list=PLevurNKwl9HEcxa6K3dUoQ1jSBUUC2UxI",
        ];

        for url in urls {
            if let Ok(_) = downloader.add_to_queue(url) {
                println!("Added \"{}\" to queue", url);
            } else {
                panic!("URL `{}` should be parsable.", url);
            }
        }
    }
}
