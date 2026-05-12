use std::sync::{Arc, atomic::AtomicBool};

use gpui::{Bounds, Pixels, point, px, size};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone)]
pub struct GlobalSetting {
    bounds: Bounds<Pixels>,
    clock_background_color: [u8; 4],
    is_terminated: Arc<AtomicBool>,
}

impl GlobalSetting {
    pub fn new(in_file: GlobalSettingFile, is_terminated: Arc<AtomicBool>) -> Self {
        Self {
            bounds: in_file.bounds,
            clock_background_color: in_file.clock_background_color,
            is_terminated: is_terminated.clone(),
        }
    }

    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    pub fn clock_background_color(&self) -> [u8; 4] {
        self.clock_background_color
    }

    pub fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }

    pub fn is_terminated(&self) -> Arc<AtomicBool> {
        self.is_terminated.clone()
    }
}

impl gpui::Global for GlobalSetting {}

/// グローバルセッティングのうち、ファイルに保存するための値
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GlobalSettingFile {
    bounds: Bounds<Pixels>,
    clock_background_color: [u8; 4],
}

impl Default for GlobalSettingFile {
    fn default() -> Self {
        let size = size(px(300.), px(300.));
        let origin = point(px(10.), px(10.));
        Self {
            bounds: Bounds::new(origin, size),
            clock_background_color: [0, 255, 255, 255],
        }
    }
}

impl From<GlobalSetting> for GlobalSettingFile {
    fn from(value: GlobalSetting) -> Self {
        Self {
            bounds: value.bounds,
            clock_background_color: value.clock_background_color,
        }
    }
}
