use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use ab_glyph::FontArc;
use gpui::{Pixels, RenderImage, Size};
use image::{Frame, Rgba};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_polygon_mut, draw_text_mut, text_size},
    point::Point,
};

type ImageBuffer = image::ImageBuffer<Rgba<u8>, Vec<u8>>;

pub struct ClockBaseImage {
    size: Size<Pixels>,
    center: Point<f32>,
    radius: f32,
    img: ImageBuffer,
    render_image: Mutex<Option<Arc<RenderImage>>>,
}

impl ClockBaseImage {
    pub fn new(size: Size<Pixels>) -> Self {
        let center = Point::new(
            size.width.to_f64() as f32 / 2.0,
            size.height.to_f64() as f32 / 2.0,
        );
        let radius = size.width.min(size.height).to_f64() as f32 / 2.0;
        let back_ground_color = Rgba::from([0, 0, 0, 0]);
        let img = ImageBuffer::from_pixel(
            size.width.to_f64() as u32,
            size.height.to_f64() as u32,
            back_ground_color,
        );

        let mut obj = Self {
            size,
            center,
            radius,
            img,
            render_image: Mutex::new(None),
        };
        obj.make_clock_base();
        obj
    }

    pub fn image(&self) -> Arc<RenderImage> {
        let mut render_image = self.render_image.lock().unwrap();
        if (*render_image).is_none() {
            let frame = Frame::new(self.img.clone());
            let new_render_image = Arc::new(RenderImage::new([frame]));
            *render_image = Some(new_render_image);
        }
        (*render_image).as_ref().unwrap().clone()
    }

    pub fn set_size(&mut self, size: Size<Pixels>) {
        if self.size != size {
            self.size = size;
            self.center = Point::new(
                size.width.to_f64() as f32 / 2.0,
                size.height.to_f64() as f32 / 2.0,
            );
            self.radius = size.width.min(size.height).to_f64() as f32 / 2.0;
            let back_ground_color = Rgba::from([0, 0, 0, 0]);
            self.img = ImageBuffer::from_pixel(
                size.width.to_f64() as u32,
                size.height.to_f64() as u32,
                back_ground_color,
            );
            self.make_clock_base();
            let mut render_image = self.render_image.lock().unwrap();
            *render_image = None;
        }
    }

    /// 時計の背景を生成する。
    fn make_clock_base(&mut self) {
        self.draw_clock_background(Rgba::from([0, 255, 255, 255]));
        self.draw_major_scale();
        self.draw_center_pin();
        self.draw_miner_scale();
        self.draw_numbers();
    }

    fn draw_clock_background(&mut self, back_ground_color: Rgba<u8>) {
        let center = (self.center.x as i32, self.center.y as i32);
        draw_filled_circle_mut(&mut self.img, center, self.radius as i32, back_ground_color);
    }

    /// 大目盛りの描画を行う
    fn draw_major_scale(&mut self) {
        let width = Point::new(self.radius / 10.0, 0.0);
        let height = Point::new(0.0, self.radius / 30.0);
        let first_point = self.center + Point::new(self.radius * 0.98, height.y / 2.0);
        let rectangle_points = [
            first_point,
            first_point - height,
            first_point - height - width,
            first_point - width,
        ];

        for i in 0..12 {
            let angle = (2.0 * PI) / 12.0 * i as f32;
            let points = rectangle_points.into_iter();
            let rotated_points: Vec<Point<i32>> = self
                .rotate_points(points, self.center, angle)
                .iter()
                .map(|p| Point::new(p.x as i32, p.y as i32))
                .collect();
            draw_polygon_mut(&mut self.img, &rotated_points, Rgba::from([0, 0, 0, 255]));
        }
    }

    /// センターピンの描画を行う
    fn draw_center_pin(&mut self) {
        let center = (self.center.x as i32, self.center.y as i32);
        draw_filled_circle_mut(
            &mut self.img,
            center,
            (self.radius / 20.0) as i32,
            Rgba::from([0, 0, 0, 255]),
        );
    }

    /// 小目盛りの描画を行う
    fn draw_miner_scale(&mut self) {
        let first_point = self.center + Point::new(self.radius * 0.93, 0.0);
        let scale_radius = self.radius / 60.0;
        let indexs = (0..60).filter(|i| i % 5 != 0);
        for i in indexs {
            let angle = (2.0 * PI) / 60.0 * i as f32;
            let scale_center = self.rotate_points([first_point].into_iter(), self.center, angle)[0];
            draw_filled_circle_mut(
                &mut self.img,
                (scale_center.x as i32, scale_center.y as i32),
                scale_radius as i32,
                Rgba::from([0, 0, 0, 255]),
            );
        }
    }

    /// 文字盤の数字を描画する
    fn draw_numbers(&mut self) {
        let font = FontArc::try_from_slice(include_bytes!("../resource/Fraunces.ttf")).unwrap();
        let font_size_base = self.radius / 3.0;
        let first_point_center = self.center + Point::new(self.radius * 0.7, 0.0);
        for i in 0..12 {
            let text = format!("{}", (i + 2) % 12 + 1);
            let font_size = if text.len() == 1 {
                font_size_base
            } else {
                font_size_base * 0.9
            };
            let angle = (2.0 * PI) / 12.0 * i as f32;
            let point_center =
                self.rotate_points([first_point_center].into_iter(), self.center, angle)[0];
            let text_size = text_size(font_size, &font, &text);
            let text_point =
                point_center - Point::new(text_size.0 as f32 / 2.0, text_size.1 as f32 / 2.0);
            draw_text_mut(
                &mut self.img,
                Rgba::from([0, 0, 0, 255]),
                text_point.x as i32,
                text_point.y as i32,
                font_size,
                &font,
                &text,
            );
        }
    }

    /// 点の集合を回転させる
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

impl From<ClockBaseImage> for Arc<RenderImage> {
    fn from(value: ClockBaseImage) -> Self {
        value.image()
    }
}
