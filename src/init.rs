//! アプリケーションの初期化関連手続き

use crate::options::Options;

use anyhow::{Context, Result};
use log::{error, info, warn};
use std::{
    fs::{File, OpenOptions},
    io::{Seek, Write},
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};
use systemd_journal_logger::JournalLog;

/// ロギング機構の初期化を行う。
pub fn init_logging(options: &Options) {
    JournalLog::new()
        .unwrap()
        .with_syslog_identifier("haridokei".to_string())
        .install()
        .unwrap();
    if options.debug {
        log::set_max_level(log::LevelFilter::Debug);
    } else if options.info {
        log::set_max_level(log::LevelFilter::Info)
    } else {
        log::set_max_level(log::LevelFilter::Warn);
    }
}

/// 指定されていればデーモン化する
pub fn do_daemonize(pid_file: &mut PidFile) -> Result<()> {
    let daemonize = daemonize::Daemonize::new().chown_pid_file(true);
    daemonize.start().context("デーモン化に失敗しました。")?;
    info!("デーモン化に成功しました。");
    pid_file
        .update_pid()
        .context("PIDファイルの更新に失敗しました。")?;
    Ok(())
}

/// シグナルハンドラの登録を行う。
/// 終了シグナル受信用のフラグを返す。
pub fn register_signal_handler() -> Arc<AtomicBool> {
    let is_terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, is_terminate.clone())
        .map(|_id| ())
        .unwrap_or_else(|e| error!("シグナルハンドラの登録に失敗しました。(SIGINT):{}", e));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, is_terminate.clone())
        .map(|_id| ())
        .unwrap_or_else(|e| error!("シグナルハンドラの登録に失敗しました。(SIGTERM):{}", e));
    is_terminate
}

/// systemdに起動完了を通知する。
pub fn notify_systemd_ready() {
    let window_initialized_message = "ウィンドウの初期化を終了しました。";
    info!("{}", window_initialized_message);
    sd_notify::notify(&[
        sd_notify::NotifyState::Ready,
        sd_notify::NotifyState::Status(window_initialized_message),
    ])
    .unwrap_or_else(|e| {
        error!("systemdへの通知に失敗しました。:{}", e);
    });
}

/// PIDファイルの維持管理を行う
pub struct PidFile {
    file: File,
    file_path: PathBuf,
    is_enable: bool,
}

impl PidFile {
    const PID_FILE_NAME: &str = "haridokei.pid";

    pub fn new() -> Result<Self> {
        let path = Self::get_pid_file_path().context("PIDファイルのパスの取得に失敗しました。")?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false) // ロック取得の確定後、明示的に、set_len(0)で消すため、ここでは、falseとする。
            .open(&path)
            .context("PIDファイルの生成に失敗しました。")?;
        let mut ret = Self {
            file,
            file_path: path.to_path_buf(),
            is_enable: false,
        };
        ret.update_pid()
            .context("PIDファイルのPID登録に失敗しました。")?;
        ret.is_enable = true;
        Ok(ret)
    }

    /// PIDファイルのパスを取得する。
    /// ファイルの生成場所は`$XDG_RUNTIME_DIR/`。環境変数のない場合、`/tmp`。
    /// もし、既存のファイルが存在する場合、ロックの可否を確認し、ロックが取れないからエラーを返す。
    fn get_pid_file_path() -> Result<PathBuf> {
        let pid_path = match std::env::var("XDG_RUNTIME_DIR") {
            Ok(runtime_dir) if !runtime_dir.trim().is_empty() => {
                let mut path = PathBuf::from(runtime_dir);
                path.push(Self::PID_FILE_NAME);
                path
            }
            Ok(_) | Err(_) => PathBuf::from(format!("/tmp/{}", Self::PID_FILE_NAME)),
        };
        Ok(pid_path)
    }

    /// PIDファイルに登録のPIDを現在のものに更新する。
    pub fn update_pid(&mut self) -> Result<()> {
        self.file
            .try_lock()
            .context("PIDファイルのロックに失敗しました。二重起動の可能性があります。")?;
        self.file.set_len(0)?;
        self.file.rewind()?;
        write!(self.file, "{}", std::process::id())?;
        self.file.flush()?;
        self.file.try_lock_shared()?;
        Ok(())
    }
}

impl Drop for PidFile {
    fn drop(&mut self) {
        if self.is_enable {
            if let Err(e) = std::fs::remove_file(&self.file_path) {
                error!("PIDファイルの削除に失敗しました。:{}", e);
            }
            info!("PIDファイルを削除しました。");
        } else {
            warn!("PIDファイルは有効化されていないため、削除は行いませんでした。");
        };
    }
}
