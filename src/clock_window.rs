use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use gpui::{
    App, AsyncApp, Pixels, Size, TitlebarOptions, Window, WindowBackgroundAppearance, WindowBounds,
    WindowOptions, prelude::*, px, size,
};
use log::info;

use crate::{clock_base_image::ClockBaseImage, clock_elm::Clock, global_setting::GlobalSetting};

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
        window_background: WindowBackgroundAppearance::Transparent,
        ..Default::default()
    };
    app.open_window(win_opt, |win, cx| {
        cx.new(|cx| {
            let win_size = win.bounds().size;
            let global_setting: &GlobalSetting = cx.global();
            let clock_background_color = global_setting.clock_background_color();
            ClockWindow::new(
                win_size,
                clock_background_color,
                global_setting.font_family(),
                global_setting.is_terminated(),
                win,
            )
        })
    })
    .unwrap();
}

/// 時計を表示するコンテキスト
pub struct ClockWindow {
    base_image: Arc<Mutex<ClockBaseImage>>,
    is_terminate: Arc<AtomicBool>,
    timer_started: bool,
}

impl ClockWindow {
    pub fn new(
        size: Size<Pixels>,
        background_color: [u8; 4],
        font_family: String,
        is_terminate: Arc<AtomicBool>,
        window: &mut Window,
    ) -> Self {
        Self {
            base_image: Arc::new(Mutex::new(ClockBaseImage::new(
                size,
                background_color,
                font_family,
                window,
            ))),
            is_terminate,
            timer_started: false,
        }
    }
}

impl Render for ClockWindow {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_terminate.load(std::sync::atomic::Ordering::Relaxed) {
            crate::init::notify_systemd_stopping();
            info!("終了シグナルを受信しました。終了処理に入ります。");
            cx.quit();
        }
        let win_size = window.bounds().size;
        let entity_id = cx.entity_id();
        if !self.timer_started {
            self.timer_started = true;
            let is_terminate = self.is_terminate.clone();
            cx.spawn(async move |_, cx: &mut AsyncApp| {
                loop {
                    if is_terminate.load(Ordering::Relaxed) {
                        break;
                    }
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(75))
                        .await;
                    cx.update(|cx| {
                        cx.notify(entity_id);
                    });
                }
            })
            .detach();
        }
        self.base_image.lock().unwrap().set_size(win_size, window);
        cx.global_mut::<GlobalSetting>().set_bounds(window.bounds());
        Clock::new(self.base_image.clone())
    }
}
