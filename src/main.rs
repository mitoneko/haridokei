mod clock_base_image;
mod clock_elm;
mod clock_window;
mod global_setting;
mod init;
mod options;

use anyhow::Result;
use clap::Parser;
use log::{error, info};

use crate::{
    clock_window::open_main_window,
    global_setting::{GlobalSetting, GlobalSettingFile},
    init::notify_systemd_ready,
};

fn main() -> Result<()> {
    let options = options::Options::parse(); // コマンドライン引数の処理
    init::init_logging(&options); // ロギング機構初期化

    // 設定ファイルの読み込み
    let global_setting: GlobalSettingFile = confy::load(global_setting::APP_NAME, None)
        .unwrap_or_else(|e| {
            error!("設定ファイルの読み込みに失敗。デフォルト値を使用します。:{e}");
            GlobalSettingFile::default()
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
    let global_setting = GlobalSetting::new(global_setting, is_terminate);

    info!("針時計を開始しました。");

    // gpui初期化
    gpui::Application::new().run(move |app| {
        app.set_global(global_setting);
        open_main_window(app);

        notify_systemd_ready();
    });
    init::notify_systemd_stopping();
    Ok(())
}
