mod clock_base_image;
mod clock_elm;
mod clock_window;
mod global_setting;
mod options;

use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};

use clap::Parser;
use gpui::{WindowBounds, WindowOptions, prelude::*, px, size};
use log::{error, info};

use crate::{clock_window::ClockWindow, global_setting::GlobalSetting};

fn main() {
    // コマンドライン引数の処理
    let options = options::Options::parse();

    // 設定ファイルの読み込み
    let global_setting: GlobalSetting =
        confy::load(global_setting::APP_NAME, None).unwrap_or_else(|e| {
            error!("設定ファイルの読み込みに失敗。デフォルト値を使用します。:{e}");
            GlobalSetting::default()
        });

    // ロギング機構初期化
    systemd_journal_logger::JournalLog::new()
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
    info!("針時計を開始しました。");

    // 指定されていればデーモン化する
    if options.daemon {
        let pid_path = match std::env::var("XDG_RUNTIME_DIR") {
            Ok(runtime_dir) if !runtime_dir.trim().is_empty() => {
                let mut path = PathBuf::from(runtime_dir);
                path.push("haridokei.pid");
                path
            }
            Ok(_) | Err(_) => PathBuf::from("/tmp/haridokei.pid"),
        };
        let daemonize = daemonize::Daemonize::new()
            .pid_file(pid_path)
            .chown_pid_file(true);
        match daemonize.start() {
            Ok(_) => info!("デーモン化に成功しました。"),
            Err(e) => error!("デーモン化に失敗しました。:{}", e),
        }
    }

    // シグナル受信ハンドラの登録
    let is_terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, is_terminate.clone())
        .map(|_id| ())
        .unwrap_or_else(|e| error!("シグナルハンドラの登録に失敗しました。(SIGINT):{}", e));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, is_terminate.clone())
        .map(|_id| ())
        .unwrap_or_else(|e| error!("シグナルハンドラの登録に失敗しました。(SIGTERM):{}", e));

    // gpui初期化
    gpui::Application::new().run(move |app| {
        // メインウィンドウの生成
        let win_opt = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(global_setting.bounds())),
            window_min_size: Some(size(px(50.), px(50.))),
            ..Default::default()
        };
        app.set_global(global_setting);
        app.open_window(win_opt, |win, cx| {
            win.on_window_should_close(cx, |win, app| {
                let setting: &mut GlobalSetting = app.global_mut();
                setting.set_bounds(win.bounds());
                confy::store(global_setting::APP_NAME, None, setting).unwrap_or_else(|e| {
                    error!("設定ファイルの保存に失敗しました。:{e}");
                });
                true
            });
            cx.new(|cx| {
                let win_size = win.bounds().size;
                let clock_background_color = cx.global::<GlobalSetting>().clock_background_color();
                ClockWindow::new(win_size, clock_background_color, is_terminate.clone())
            })
        })
        .unwrap();
        let window_initialized_message = "ウィンドウの初期化を終了しました。";
        info!("{}", window_initialized_message);
        sd_notify::notify(&[
            sd_notify::NotifyState::Ready,
            sd_notify::NotifyState::Status(window_initialized_message),
        ])
        .unwrap_or_else(|e| {
            error!("systemdへの通知に失敗しました。:{}", e);
        });
    });
    info!("針時計を終了しました。");
}
