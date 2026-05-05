use gpui::{AsyncApp, Pixels, Size, prelude::*};
use imageproc::point::Point;

use crate::{clock_base_image::ClockBaseImage, clock_elm::Clock};

/// 時計を表示するコンテキスト
pub struct ClockWindow {
    base_image: ClockBaseImage,
}

impl ClockWindow {
    pub fn new(size: Size<Pixels>, back_ground_color: [u8; 4]) -> Self {
        Self {
            base_image: ClockBaseImage::new(size, back_ground_color),
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
        let win_size = window.bounds().size;
        let entity_id = cx.entity_id();
        cx.spawn(async move |_, cx: &mut AsyncApp| {
            gpui::Timer::after(std::time::Duration::from_millis(10)).await;
            cx.update(|cx| {
                cx.notify(entity_id);
            })
            .unwrap();
        })
        .detach();
        self.base_image.set_size(win_size);
        Clock::new(self.base_image.image())
    }
}
