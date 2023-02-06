#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use prawnloader::downloader;
use std::path::Path;
use tauri::api::path::download_dir;

fn main() {
    struct MockOptions<'a> {
        pub dest_folder: &'a Path,
    }

    let mock_options = MockOptions {
        dest_folder: &download_dir().unwrap(),
    };

    let urls = vec![
        // "https://www.deezer.com/fr/track/597403742",
        // "https://deezer.page.link/mZsk7WU6P4r4h3nA8",
        // "https://www.deezer.com/fr/album/345755977",
        // "https://www.deezer.com/fr/playlist/10575085742",
        // "https://music.youtube.com/watch?v=gAy5WZo9kts",
        // "https://music.youtube.com/playlist?list=OLAK5uy_nSewatBUjTf3IO_DIqqMXn3ps_WbEAyi4",
        "https://www.youtube.com/watch?v=ORofRTMg-iY",
        // "https://www.youtube.com/playlist?list=PLevurNKwl9HEcxa6K3dUoQ1jSBUUC2UxI",
    ];

    for url in urls {
        if let Some(t) = downloader::download(url, mock_options.dest_folder) {
            println!("Downloaded {}", t.0.as_ref().get_song().title);
        }
    }

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
