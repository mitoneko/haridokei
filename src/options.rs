use clap::Parser;

/// 針時計を表示するアプリ
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    /// アプリケーションをデーモン化して実行する
    #[arg(short, long)]
    pub daemon: bool,
    /// infoレベルログの出力を行う。(--debugを優先する)
    #[arg(long)]
    pub info: bool,
    /// debugレベルのログの出力を行う。
    #[arg(long)]
    pub debug: bool,
}
