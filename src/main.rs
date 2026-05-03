mod clock_base_image;
mod clock_elm;
mod clock_window;
mod options;

use clap::Parser;
use gpui::{Bounds, WindowBounds, WindowOptions, point, prelude::*, px, size};
use log::{error, info};

use crate::clock_window::ClockWindow;

fn main() {
    // コマンドライン引数の処理
    let options = options::Options::parse();

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
        let daemonize = daemonize::Daemonize::new()
            .pid_file("/tmp/haridokei.pid")
            .chown_pid_file(true);
        match daemonize.start() {
            Ok(_) => info!("デーモン化に成功しました。"),
            Err(e) => error!("デーモン化に失敗しました。:{}", e),
        }
    }

    // gpui初期化
    gpui::Application::new().run(|app| {
        // メインウィンドウの生成
        let win_opt = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(300.), px(300.)),
            })),
            ..Default::default()
        };
        app.open_window(win_opt, |win, cx| {
            cx.new(|_cx| {
                let win_size = win.bounds().size;
                ClockWindow::new(win_size)
            })
        })
        .unwrap();
    });
    info!("針時計を終了しました。");
}
