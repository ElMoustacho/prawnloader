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
    },
    events::Event,
    models::music::{Song, SourceDownloader},
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
async fn get_songs(url: String, state: State<'_, DownloadersState>) -> Result<Vec<Song>, String> {
    let parsed_id = parse_id(&url)
        .await
        .map_err(|_| format!("Unable to parse URL\"{url}\""))?;
    let songs: Vec<Song> = match parsed_id {
        ParsedId::DeezerAlbum(id) => state
            .deezer_downloader
            .get_album_tracks(id)
            .await
            .ok_or(format!("Invalid album id {id}"))?,
        ParsedId::DeezerTrack(id) => state
            .deezer_downloader
            .get_track(id)
            .await
            .map(|x| vec![x.into()])
            .ok_or(format!("Invalid track id {id}"))?,
        ParsedId::YoutubeVideo(id) => state
            .youtube_downloader
            .get_song(id)
            .await
            .map(|x| vec![x])
            .ok_or(format!("Invalid video id"))?,
        ParsedId::YoutubePlaylist(id) => state
            .youtube_downloader
            .get_playlist_songs(id)
            .await
            .ok_or(format!("Invalid playlist id"))?,
    };

    let songs = songs.into_iter().map(|track| Song::from(track)).collect();

    Ok(songs)
}

#[tauri::command]
async fn request_download(song: Song, state: State<'_, DownloadersState>) -> Result<(), String> {
    match song.source {
        SourceDownloader::Youtube => state
            .youtube_downloader
            .request_download(song)
            .await
            .map_err(|err| err.to_string()),
        SourceDownloader::Deezer => state
            .deezer_downloader
            .request_download(song)
            .await
            .map_err(|err| err.to_string()),
    }
}

#[tauri::command]
fn get_config(state: State<'_, Mutex<ConfigState>>) -> Result<Config, ()> {
    println!("getting config");
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
                        Event::RemoveFromQueue(track) => {
                            handle.emit_all(event_name, track).unwrap()
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
            get_songs,
            request_download,
            get_config,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
