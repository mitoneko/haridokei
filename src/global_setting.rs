use std::sync::{Arc, Mutex, atomic::AtomicBool};

use gpui::{Bounds, Pixels, point, px, size};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone)]
pub struct GlobalSetting {
    save_item: Arc<Mutex<GlobalSettingFile>>,
    is_terminated: Arc<AtomicBool>,
}

impl GlobalSetting {
    pub fn new(in_file: Arc<Mutex<GlobalSettingFile>>, is_terminated: Arc<AtomicBool>) -> Self {
        Self {
            save_item: in_file,
            is_terminated: is_terminated.clone(),
        }
    }

    pub fn bounds(&self) -> Bounds<Pixels> {
        self.save_item.lock().unwrap().bounds
    }

    pub fn clock_background_color(&self) -> [u8; 4] {
        self.save_item.lock().unwrap().clock_background_color
    }

    pub fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.save_item.lock().unwrap().bounds = bounds;
    }

    pub fn is_terminated(&self) -> Arc<AtomicBool> {
        self.is_terminated.clone()
    }

    pub fn font_family(&self) -> String {
        self.save_item.lock().unwrap().font_family.clone()
    }
}

impl gpui::Global for GlobalSetting {}

/// グローバルセッティングのうち、ファイルに保存するための値
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GlobalSettingFile {
    bounds: Bounds<Pixels>,
    clock_background_color: [u8; 4],
    font_family: String,
}

impl Default for GlobalSettingFile {
    fn default() -> Self {
        let size = size(px(300.), px(300.));
        let origin = point(px(10.), px(10.));
        Self {
            bounds: Bounds::new(origin, size),
            clock_background_color: [0, 255, 255, 255],
            font_family: ".SystemUIFont".into(),
        }
    }
}

impl From<GlobalSetting> for GlobalSettingFile {
    fn from(value: GlobalSetting) -> Self {
        value.save_item.lock().unwrap().clone()
    }
}
