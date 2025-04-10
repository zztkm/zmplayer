use ini::Ini;
use orfail::{Failure, OrFail, Result};
use std::path::Path;

const DEFAULT_SEARCH_DEPTH: u32 = 4;

pub struct Config {
    /// 音楽フォルダのパス
    pub music_dir: String,
    /// 音楽フォルダを探索する深さ
    pub search_depth: u32,
}

impl Config {
    /// config ファイルから設定を読み込む関数
    pub fn parse(file_path: &Path) -> Result<Self> {
        let config = Ini::load_from_file(file_path).or_fail()?;

        // root = None::<String> でセクションを取得する、取得できなかった場合はエラーを返す
        let root = match config.section(None::<String>) {
            Some(s) => s,
            None => return Err(Failure::new("Failed to load config file".to_string())),
        };

        let music_dir = match root.get("music_dir") {
            Some(dir) => dir.to_string(),
            None => get_default_music_folder(),
        };

        let search_depth = match root.get("search_depth") {
            Some(depth) => depth.parse::<u32>().or_fail()?,
            None => DEFAULT_SEARCH_DEPTH,
        };

        Ok(Self {
            music_dir,
            search_depth,
        })
    }

    /// config ファイルを生成する関数
    pub fn write_to_file(&self, file_path: &Path) -> Result<()> {
        let mut config = Ini::new();
        config
            .with_section(Some("zmplayer"))
            .set("music_dir", self.music_dir.clone());
        config
            .with_section(Some("zmplayer"))
            .set("search_depth", self.search_depth.to_string());

        // TODO: write_to_file の返り値をそのまま返せばよくね？
        config.write_to_file(file_path).or_fail()
    }
}

/// 音楽フォルダのデフォルトパスを取得する関数
fn get_default_music_folder() -> String {
    if cfg!(target_os = "windows") {
        format!(
            "{}\\Music",
            std::env::var("USERPROFILE").unwrap_or_default()
        )
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        format!("{}/Music", std::env::var("HOME").unwrap_or_default())
    } else {
        String::from("./music") // フォールバック
    }
}
