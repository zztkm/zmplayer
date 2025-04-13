use ini::Ini;
use orfail::{Failure, OrFail, Result};
use std::path::{Path, PathBuf};

// / 音楽フォルダを検索する深さのデフォルト値
pub const DEFAULT_SEARCH_DEPTH_STR: &str = "4";

pub struct Config {
    /// 音楽フォルダのパス
    pub music_dir: PathBuf,
    /// 音楽ディレクトリを探索する深さ
    pub search_depth: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            music_dir: PathBuf::from(get_default_music_folder()),
            search_depth: DEFAULT_SEARCH_DEPTH_STR.parse().unwrap(),
        }
    }
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
            Some(dir) => {
                println!("Music Directory dir: {}", dir);
                PathBuf::from(dir)
            }
            None => PathBuf::from(get_default_music_folder()),
        };

        let search_depth = match root.get("search_depth") {
            Some(depth) => depth.parse::<u32>().or_fail()?,
            None => DEFAULT_SEARCH_DEPTH_STR.parse().unwrap(),
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
            .with_section(None::<String>)
            .set("music_dir", self.music_dir.to_string_lossy().to_string());
        config
            .with_section(None::<String>)
            .set("search_depth", self.search_depth.to_string());

        // フォルダ、ファイルが存在しない場合は作成する
        if !file_path.exists() {
            std::fs::create_dir_all(file_path.parent().unwrap()).or_fail()?;
        }
        config.write_to_file(file_path).or_fail()
    }
}

/// 音楽フォルダのデフォルトパスを取得する関数
pub fn get_default_music_folder() -> String {
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
