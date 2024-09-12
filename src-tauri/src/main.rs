#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use crossbeam_channel::unbounded;
use prawnloader::{
    config::Config,
    downloaders::{
        deezer::Downloader as DeezerDownloader, youtube::Downloader as YoutubeDownloader,
        DownloadRequest,
    },
    events::Event,
    models::music::Item,
    parsers::{parse_id, ParsedId},
};
use tauri::{Manager, State};

struct DownloadersState {
    deezer_downloader: DeezerDownloader,
    youtube_downloader: YoutubeDownloader,
}

struct ConfigState {
    config: Config,
}

#[tauri::command]
async fn get_item(url: String, state: State<'_, DownloadersState>) -> Result<Item, String> {
    let parsed_id = parse_id(&url)
        .await
        .map_err(|_| format!("Unable to parse URL\"{url}\""))?;
    let item: Item = match parsed_id {
        ParsedId::DeezerAlbum(id) => Item::DeezerAlbum {
            album: state
                .deezer_downloader
                .get_album(id)
                .await
                .ok_or(format!("Invalid album id {id}"))?,
            merge_tracks: false,
        },
        ParsedId::DeezerTrack(id) => Item::DeezerTrack {
            track: state
                .deezer_downloader
                .get_song(id)
                .await
                .ok_or(format!("Invalid track id {id}"))?,
        },
        ParsedId::YoutubeVideo(id) => Item::YoutubeVideo {
            video: state
                .youtube_downloader
                .get_song(id)
                .await
                .ok_or(format!("Invalid video id"))?,
            split_by_chapters: false,
        },
        ParsedId::YoutubePlaylist(id) => Item::YoutubePlaylist {
            playlist: state
                .youtube_downloader
                .get_playlist(id)
                .await
                .ok_or(format!("Invalid playlist id"))?,
        },
    };

    Ok(item)
}

#[tauri::command]
async fn request_download(
    request: DownloadRequest,
    state: State<'_, DownloadersState>,
    config_state: State<'_, Mutex<ConfigState>>,
) -> Result<(), String> {
    let config = config_state.lock().unwrap().config.clone();
    match request.item {
        Item::DeezerAlbum { .. } | Item::DeezerTrack { .. } => {
            state
                .deezer_downloader
                .request_download(request, config)
                .await;
        }
        Item::YoutubeVideo { .. } | Item::YoutubePlaylist { .. } => {
            state
                .youtube_downloader
                .request_download(request, config)
                .await;
        }
    }

    Ok(())
}

#[tauri::command]
fn get_config(state: State<'_, Mutex<ConfigState>>) -> Result<Config, ()> {
    Ok(state.lock().unwrap().config.clone())
}

#[tauri::command]
fn update_config(config: Config, state: State<'_, Mutex<ConfigState>>) -> Result<Config, String> {
    state.lock().as_mut().unwrap().config = config;

    // Return the modified config in case we need to do additional checks later
    Ok(state.lock().unwrap().config.to_owned())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let (progress_tx, progress_rx) = unbounded();
            let (event_tx, event_rx) = unbounded();

            let deezer_downloader = DeezerDownloader::new(progress_tx.clone());
            let youtube_downloader = YoutubeDownloader::new(progress_tx.clone());

            let handle = app.handle();

            // Transfer any download event to the main event loop
            std::thread::spawn(move || {
                while let Ok(progress_event) = progress_rx.recv() {
                    event_tx.send(Event::from(progress_event)).unwrap();
                }
            });

            // Transfer events to front-end
            std::thread::spawn(move || {
                while let Ok(event) = event_rx.recv() {
                    let event_name = &event.to_string()[..];
                    match event {
                        Event::Waiting(track) => handle.emit_all(event_name, track).unwrap(),
                        Event::Start(track) => handle.emit_all(event_name, track).unwrap(),
                        Event::Finish(track) => handle.emit_all(event_name, track).unwrap(),
                        Event::DownloadError(track, err_msg) => {
                            handle.emit_all(event_name, (track, err_msg)).unwrap()
                        }
                    }
                }
            });

            app.manage(DownloadersState {
                deezer_downloader,
                youtube_downloader,
            });

            app.manage(Mutex::new(ConfigState {
                config: Config::default(),
            }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_item,
            request_download,
            get_config,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
