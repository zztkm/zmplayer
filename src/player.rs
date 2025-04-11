use std::{io::BufReader, path::PathBuf};

use crate::config::Config;
use orfail::{OrFail, Result};
use walkdir::WalkDir;

/// 音楽プレイヤーを管理する
pub struct Player {
    /// このストリームは、音楽を再生するために保持しておく必要がある(なんでかはわかってない...)
    stream: rodio::OutputStream,
    /// rodio Sink
    sink: rodio::Sink,
    /// 読み込むファイルパス
    file_paths: Vec<PathBuf>,
    /// 音楽プレイヤーの設定
    config: Config,
}

impl Player {
    /// 音楽プレイヤーを初期化する
    pub fn new(config: Config) -> Result<Self> {
        let (stream, handle) = rodio::OutputStream::try_default().or_fail()?;
        let sink = rodio::Sink::try_new(&handle).or_fail()?;

        // 音楽フォルダのパスを取得する
        let file_paths = WalkDir::new(&config.music_dir)
            .max_depth(config.search_depth as usize)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.or_fail().ok()?;
                if entry.file_type().is_file() {
                    Some(entry.path().to_path_buf())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(Self {
            stream,
            sink,
            file_paths,
            config,
        })
    }

    /// 音楽プレイヤーを実行する
    pub fn run(&mut self) -> Result<()> {
        for file_path in &self.file_paths {
            let file = std::fs::File::open(file_path).or_fail()?;
            println!("adding: {}", file_path.display());
            if let Ok(source) = rodio::Decoder::new(BufReader::new(file)).or_fail() {
                self.sink.append(source);
            } else {
                println!("Failed to decode: {}", file_path.display());
            }
        }
        self.sink.sleep_until_end();
        Ok(())
    }

    /// 音楽プレイヤーの設定を表示する
    pub fn show_config(&self) {
        println!("Music Directory: {}", self.config.music_dir.display());
        println!("Search Depth: {}", self.config.search_depth);
    }
}
