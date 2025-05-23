use crate::config::{DEFAULT_SEARCH_DEPTH_STR, get_default_music_folder};

pub struct Init {
    // 強制的に初期化するかどうか
    pub force: bool,
    // 音楽フォルダのパス
    // デフォルトはシステムの Music フォルダを使用する
    pub dir: String,
    // 音楽フォルダを探索する深さ
    pub search_depth: u32,
}

pub enum Commands {
    // 音楽プレイヤーの実行
    Run,
    // プロジェクトを初期化するためのコマンド
    Init(Init),
    // コマンドが指定されていない場合は help を表示させる
    Help(String),
}

impl Commands {
    pub fn parse<I>(args: I) -> noargs::Result<Self>
    where
        I: Iterator<Item = String>,
    {
        let mut args = noargs::RawArgs::new(args);

        args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
        args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

        if noargs::VERSION_FLAG.take(&mut args).is_present() {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            return Ok(Commands::Help("Version displayed".to_string()));
        }
        noargs::HELP_FLAG.take_help(&mut args);

        // コマンドの解析
        let cmd: Option<Commands> = if noargs::cmd("run").take(&mut args).present().is_some() {
            Some(Commands::Run)
        } else if noargs::cmd("init").take(&mut args).present().is_some() {
            let force = noargs::flag("force")
                .doc("Force initialization")
                .take(&mut args)
                .is_present();
            // デフォルト値を&'static str に変換するために Box::leak を使用
            // メモリを意図的にリークすることで 'static なライフタイムを持つ文字列を作成することができるというテクニック (らしい)
            // refs: https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
            let dir = noargs::opt("dir")
                .doc("音楽ディレクトリのパス")
                .default(Box::leak(get_default_music_folder().into_boxed_str()))
                .take(&mut args)
                .then(|opt| opt.value().parse())?;
            let search_depth = noargs::opt("search_depth")
                .doc("音楽ディレクトリを探索する深さ")
                .default(DEFAULT_SEARCH_DEPTH_STR)
                .take(&mut args)
                .then(|opt| opt.value().parse())?;

            Some(Commands::Init(Init {
                force,
                dir,
                search_depth,
            }))
        } else {
            args.metadata_mut().help_mode = true;
            None
        };

        if let Some(help) = args.finish()? {
            Ok(Self::Help(help))
        } else if let Some(cmd) = cmd {
            Ok(cmd)
        } else {
            Ok(Commands::Help("No command specified".to_string()))
        }
    }
}
