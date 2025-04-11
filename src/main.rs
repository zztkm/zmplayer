use orfail::OrFail;
use std::path::PathBuf;

use zmplayer::commands::Commands;
use zmplayer::config::Config;
use zmplayer::player::Player;

fn main() -> noargs::Result<()> {
    let cmd = Commands::parse(std::env::args())?;
    match cmd {
        Commands::Run => {
            // 音楽プレイヤーを実行する
            println!("Running music player...");
            let config_path = get_config_path();
            let config = if config_path.exists() {
                Config::parse(&config_path).or_fail()?
            } else {
                // Config が存在しない場合は、デフォルトの Config を作成し、保存する
                let config = Config::default();
                config.write_to_file(&config_path).or_fail()?;
                config
            };
            if !config_path.exists() {
                // Config を自動生成して、保存する
                let config = Config::default();
                config.write_to_file(&config_path).or_fail()?;
                println!("Config file created at: {}", config_path.display());
            }

            let player = Player::new(config)?;
            player.show_config();
            player.run().or_fail()?
        }
        Commands::Init(init) => {
            // プロジェクトを初期化する
            let config = Config {
                music_dir: PathBuf::from(init.dir), // 指定された音楽フォルダのパス
                search_depth: init.search_depth,    // 指定された探索深さ
            };

            let config_path = get_config_path();
            if config_path.exists() && !init.force {
                println!("Config file already exists. Use --force to overwrite.");
                return Ok(());
            }

            if config_path.exists() {
                std::fs::remove_file(&config_path).or_fail()?;
            }

            config.write_to_file(&config_path)?;
            println!("Config file created at: {}", config_path.display());
        }
        Commands::Help(help) => {
            // ヘルプを表示する
            print!("{help}");
        }
    }

    Ok(())
}

fn get_config_path() -> PathBuf {
    // OS ごとのデフォルトの config ファイルのパスを取得する
    if cfg!(target_os = "windows") {
        PathBuf::from(format!(
            "{}\\AppData\\Roaming\\zmplayer\\config.ini",
            std::env::var("USERPROFILE").unwrap_or_default()
        ))
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        PathBuf::from(format!(
            "{}/.config/zmplayer/config.ini",
            std::env::var("HOME").unwrap_or_default()
        ))
    } else {
        PathBuf::from("config.ini")
    }
}
