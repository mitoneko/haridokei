use std::sync::{Arc, atomic::AtomicBool};

use gpui::{
    App, AsyncApp, Pixels, Size, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size,
};
use imageproc::point::Point;
use log::{error, info};

use crate::{
    clock_base_image::ClockBaseImage,
    clock_elm::Clock,
    global_setting::{self, GlobalSetting, GlobalSettingFile},
};

/// タイトルバーの表示名
const TITLE_NAME: &str = "Haridokei";
/// アプリケーションID(WM_CLASSに使用される)
const APP_ID: &str = "jp.laki.haridokei";

/// メインウィンドウをオープンする
pub fn open_main_window(app: &mut App) {
    let global_setting: &GlobalSetting = app.global();
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
    app.open_window(win_opt, |win, cx| {
        cx.on_app_quit(|app| {
            let setting: &mut GlobalSetting = app.global_mut();
            let setting_file: GlobalSettingFile = setting.clone().into();
            match confy::store(global_setting::APP_NAME, None, setting_file) {
                Ok(_) => info!("設定ファイルを保存しました。"),
                Err(e) => error!("設定ファイルの保存に失敗しました。:{e}"),
            };
            async {}
        })
        .detach();
        cx.new(|cx| {
            let win_size = win.bounds().size;
            let global_setting: &GlobalSetting = cx.global();
            let clock_background_color = global_setting.clock_background_color();
            ClockWindow::new(
                win_size,
                clock_background_color,
                global_setting.is_terminated(),
            )
        })
    })
    .unwrap();
}

/// 時計を表示するコンテキスト
pub struct ClockWindow {
    base_image: ClockBaseImage,
    is_terminate: Arc<AtomicBool>,
}

impl ClockWindow {
    pub fn new(
        size: Size<Pixels>,
        background_color: [u8; 4],
        is_terminate: Arc<AtomicBool>,
    ) -> Self {
        Self {
            base_image: ClockBaseImage::new(size, background_color),
            is_terminate,
        }
    }

    /// 点の集合を回転させる
    #[allow(dead_code)]
    fn rotate_points<I: Iterator<Item = Point<f32>>>(
        &self,
        points: I,
        origin: Point<f32>,
        angle: f32,
    ) -> Vec<Point<f32>> {
        points
            .map(|p| {
                let translated = p - origin;
                let rotated = Point::new(
                    translated.x * angle.cos() - translated.y * angle.sin(),
                    translated.x * angle.sin() + translated.y * angle.cos(),
                );
                rotated + origin
            })
            .collect()
    }
}

impl Render for ClockWindow {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_terminate.load(std::sync::atomic::Ordering::Relaxed) {
            info!("終了シグナルを受信しました。終了処理に入ります。");
            crate::init::notify_systemd_stopping();
            cx.quit();
        }
        let win_size = window.bounds().size;
        let entity_id = cx.entity_id();
        cx.spawn(async move |_, cx: &mut AsyncApp| {
            gpui::Timer::after(std::time::Duration::from_millis(33)).await;
            cx.update(|cx| {
                cx.notify(entity_id);
            })
            .unwrap();
        })
        .detach();
        self.base_image.set_size(win_size);
        cx.global_mut::<GlobalSetting>().set_bounds(window.bounds());
        Clock::new(self.base_image.image())
    }
}
