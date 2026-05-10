mod clock_base_image;
mod clock_elm;
mod clock_window;
mod global_setting;
mod init;
mod options;

use anyhow::Result;
use clap::Parser;
use gpui::{TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size};
use log::{error, info};

use crate::{clock_window::ClockWindow, global_setting::GlobalSetting};

/// アプリケーションID(WM_CLASSに使用される)
const APP_ID: &str = "jp.laki.haridokei";
/// タイトルバーの表示名
const TITLE_NAME: &str = "Haridokei";

fn main() -> Result<()> {
    let options = options::Options::parse(); // コマンドライン引数の処理
    init::init_logging(&options); // ロギング機構初期化

    // 設定ファイルの読み込み
    let global_setting: GlobalSetting =
        confy::load(global_setting::APP_NAME, None).unwrap_or_else(|e| {
            error!("設定ファイルの読み込みに失敗。デフォルト値を使用します。:{e}");
            GlobalSetting::default()
        });

    let mut pid_file = init::PidFile::new().inspect_err(|e| {
        error!("PIDファイルの生成に失敗しました。:{e:?}");
    })?;
    if options.daemon {
        init::do_daemonize(&mut pid_file).inspect_err(|e| {
            error!("デーモン化に失敗しました。:{e:?}");
        })?;
    };

    let is_terminate = init::register_signal_handler();

    info!("針時計を開始しました。");

    // gpui初期化
    gpui::Application::new().run(move |app| {
        // メインウィンドウの生成
        let titlebar = TitlebarOptions {
            title: Some(TITLE_NAME.into()),
            ..Default::default()
        };
        let win_opt = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(global_setting.bounds())),
            window_min_size: Some(size(px(50.), px(50.))),
            titlebar: Some(titlebar),
            app_id: Some(APP_ID.into()),
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
    Ok(())
}
