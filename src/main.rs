mod clock_window;

use gpui::{Bounds, WindowBounds, WindowOptions, point, prelude::*, px, size};
use log::info;

use crate::clock_window::ClockWindow;

fn main() {
    // ロギング機構初期化
    systemd_journal_logger::JournalLog::new()
        .unwrap()
        .with_syslog_identifier("haridokei".to_string())
        .install()
        .unwrap();
    log::set_max_level(log::LevelFilter::Info);

    // gpui初期化
    info!("starting application");
    gpui::Application::new().run(|app| {
        // メインウィンドウの生成
        let win_opt = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(300.), px(300.)),
            })),
            ..Default::default()
        };
        app.open_window(win_opt, |_win, cx| cx.new(|_cx| ClockWindow::new()))
            .unwrap();
    });
    info!("application exited");
}
