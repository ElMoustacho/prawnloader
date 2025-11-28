use std::path::PathBuf;

use color_eyre::eyre::Result;
use tauri::api::path::download_dir;
use yt_dlp::{
    client::deps::{Libraries, LibraryInstaller},
    download::{AudioCodec, PostProcessConfig},
    Youtube,
};

use crate::downloaders::ytdlp;

#[cfg(test)]
mod tests {
    use tracing::Level;

    use super::*;

    async fn download() -> Result<()> {
        let libraries_dir = PathBuf::from("libs");
        let output_dir = download_dir().unwrap();

        let youtube = Youtube::with_new_binaries(libraries_dir, output_dir)
            .await
            .expect("Failed to create a new Youtube instance");
        let result = youtube
            .download_audio_stream_with_quality(
                "bbcPLei01Ls",
                "testfile.mp3",
                yt_dlp::model::AudioQuality::Best,
                yt_dlp::model::AudioCodecPreference::MP3,
            )
            .await
            .inspect(|_| println!("Downloaded successfully"))?;

        let r = result.with_file_name("prout.mp3");

        let post_process_result = youtube
            .postprocess_video(
                result,
                r.to_str().unwrap(),
                PostProcessConfig {
                    video_codec: None,
                    audio_codec: Some(AudioCodec::MP3),
                    video_bitrate: None,
                    audio_bitrate: None,
                    resolution: None,
                    framerate: None,
                    preset: None,
                    filters: Vec::new(),
                },
            )
            .await;

        tracing::info!("Post process result: {post_process_result:#?}");

        Ok(())
    }

    #[tokio::test]
    async fn ytdlp() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        let result = download().await.unwrap();
    }
}
