use std::path::PathBuf;

use sanitize_filename::sanitize;
use tokio::process::Command;

use crate::{
    config::{Config, YoutubeFormat},
    models::music::Song,
};

pub(super) struct YtDlpCommandBuilder<'a> {
    pub command: Command,
    song: &'a Song,
    config: &'a Config,
}

impl<'a> YtDlpCommandBuilder<'a> {
    pub fn new(song: &'a Song, config: &'a Config) -> Self {
        let mut builder = Self {
            command: Command::new("yt-dlp"),
            song,
            config,
        };

        let params = builder.generate_params();
        builder.build_cmd_args(params);

        log::debug!(
            "Built command with args: {:#?}",
            builder
                .command
                .as_std()
                .get_args()
                .map(|arg| arg.to_string_lossy().to_string())
                .collect::<Vec<_>>()
        );

        builder
    }

    fn generate_params(&self) -> YtDlpCommandParams {
        let config = self.config;

        let has_chapters = self
            .song
            .chapters
            .as_ref()
            .is_some_and(|chapters| chapters.len() > 0);
        // Whether we should actually split the chapters, if the song has some
        let split_chapters = config.youtube_split_chapters && has_chapters;

        let output_folder = if config.group_songs_in_folder && split_chapters {
            config
                .download_folder
                .join(sanitize(self.song.title.clone()))
        } else {
            config.download_folder.clone()
        };

        YtDlpCommandParams {
            output_folder,
            split_chapters,
            format: config.youtube_format.clone(),
        }
    }

    fn build_cmd_args(&mut self, params: YtDlpCommandParams) {
        // Default args
        self.command.args(vec![
            &self.song.id,
            "--extract-audio",
            "--embed-metadata",
            "-o",
            "%(title)s.%(ext)s",
        ]);

        // If output format is mp3, add special preset
        match params.format {
            YoutubeFormat::MP3 => {
                self.command.args(vec!["-t", "mp3"]);
            }
            YoutubeFormat::WEBM => {}
            YoutubeFormat::WAV => {}
            YoutubeFormat::OGG => {}
        }

        // Split chapters
        if params.split_chapters {
            self.command.arg("--split-chapters");

            self.command.arg("--paths");
            self.command
                .arg(format!("home:{}", params.output_folder.to_str().unwrap()));

            self.command.arg("--paths");
            self.command.arg(format!(
                "temp:{}",
                params.output_folder.join("temp").to_str().unwrap()
            ));
        } else {
            // Add the output
            let file_name = sanitize(format!("{}.{}", self.song.title, params.format.to_string()));
            let file_path = params.output_folder.join(file_name);
            self.command.arg(file_path);
        }
    }
}

struct YtDlpCommandParams {
    output_folder: PathBuf,
    split_chapters: bool,
    format: YoutubeFormat,
}

#[cfg(test)]
mod tests {
    use tauri::api::path::download_dir;

    use crate::models::music::{Chapter, SongAlbum};

    use super::*;

    #[test]
    fn yt_dlp_command_builder() {
        env_logger::init();

        let song = Song {
            title: "Test Song".to_string(),
            id: String::new(),
            album: SongAlbum {
                title: String::new(),
                cover_url: String::new(),
            },
            artist: String::new(),
            release_date: String::new(),
            chapters: Some(vec![Chapter {
                title: "Chapter 1".to_string(),
                start_time: 0.0,
                end_time: 60.0,
            }]),
        };
        let config = Config {
            group_songs_in_folder: true,
            download_folder: download_dir().unwrap(),
            youtube_format: YoutubeFormat::MP3,
            youtube_split_chapters: true,
            ..Default::default()
        };
        let mut builder = YtDlpCommandBuilder::new(&song, &config);

        builder.build_cmd_args(builder.generate_params());
        let command = builder.command;

        let command_args = command
            .as_std()
            .get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        log::debug!("Generated args: {command_args:?}");
        assert!(command_args.contains("--extract-audio"));
        assert!(command_args.contains("--embed-metadata"));
        assert!(command_args.contains("-t"));
        assert!(command_args.contains("mp3"));
        assert!(command_args.contains("--split-chapters"));
        assert!(command_args.contains("--paths"));
        assert!(command_args.contains("temp"));
        assert!(command_args.contains("Test Song.mp3"));
    }
}
