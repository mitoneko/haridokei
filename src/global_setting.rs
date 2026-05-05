use gpui::{Pixels, Size, px, size};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalSetting {
    size: Size<Pixels>,
    clock_background_color: [u8; 4],
}

impl GlobalSetting {
    pub fn size(&self) -> Size<Pixels> {
        self.size
    }

    pub fn clock_background_color(&self) -> [u8; 4] {
        self.clock_background_color
    }

    pub fn set_size(&mut self, size: Size<Pixels>) {
        self.size = size;
    }
}

impl Default for GlobalSetting {
    fn default() -> Self {
        Self {
            size: size(px(300.), px(300.)),
            clock_background_color: [0, 255, 255, 255],
        }
    }
}

impl gpui::Global for GlobalSetting {}
