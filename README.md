# zmplayer

zztkm's music player.

今のところ、 ローカルにある音楽ファイルを再生するためのシンプルなコマンドラインツールです。

## 使い方

### インストール

crate.io では公開していないので、GitHub から直接インストールします。
```bash
cargo install --git https://github.com/zztkm/zmplayer.git
```

または、git clone してインストール。
```bash
git clone https://github.com/zztkm/zmplayer.git
cd zmplayer
cargo install --path .
```

### 設定

```bash
zmplayer init -h
```

## 対応 OS

※現在は、Windows での動作確認のみ行っています。

- Windows
- Linux
- MacOS
