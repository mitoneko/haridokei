use gpui::{Bounds, Pixels, point, px, size};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GlobalSetting {
    bounds: Bounds<Pixels>,
    clock_background_color: [u8; 4],
}

impl GlobalSetting {
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    pub fn clock_background_color(&self) -> [u8; 4] {
        self.clock_background_color
    }

    pub fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }
}

impl Default for GlobalSetting {
    fn default() -> Self {
        let size = size(px(300.), px(300.));
        let origin = point(px(10.), px(10.));
        Self {
            bounds: Bounds::new(origin, size),
            clock_background_color: [0, 255, 255, 255],
        }
    }
}

impl gpui::Global for GlobalSetting {}
