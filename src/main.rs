use std::path::Path;

use zmplayer::commands::Commands;
use zmplayer::config::Config;

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
            let config = Config {
                music_dir: init.dir,
                search_depth: init.search_depth, // 指定された探索深さ
            };
            // config ファイルを生成する
            config.write_to_file(Path::new("config.init"))?;
        }
        Commands::Help(help) => {
            // ヘルプを表示する
            print!("{help}");
        }
    }

    Ok(())
}
