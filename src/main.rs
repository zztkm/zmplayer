use zmplayer::commands::Commands;

fn main() -> noargs::Result<()> {
    let cmd = Commands::parse(std::env::args())?;
    match cmd {
        Commands::Run => {
            // 音楽プレイヤーを実行する
            println!("Running music player...");
            // ここに音楽プレイヤーを実行するコードを追加
        }
        Commands::Init(init) => {
            // プロジェクトを初期化する
            println!("Initializing project...");
        }
        Commands::Help(help) => {
            // ヘルプを表示する
            print!("{help}");
        }
    }

    Ok(())
}

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
