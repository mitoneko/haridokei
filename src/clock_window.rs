use std::{f32::consts::PI, sync::Arc};

use ab_glyph::FontArc;
use gpui::{AsyncApp, Img, Pixels, RenderImage, Size, div, img, prelude::*};
use image::{Frame, Rgba};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_polygon_mut, draw_text_mut, text_size},
    point::Point,
};

type ImageBuffer = image::ImageBuffer<Rgba<u8>, Vec<u8>>;

/// 時計を表示するコンテキスト
pub struct ClockWindow {
    base_image: Option<ImageBuffer>,
    size: Option<Size<Pixels>>,
}

impl ClockWindow {
    pub fn new() -> Self {
        Self {
            base_image: None,
            size: None,
        }
    }

    /// 時計の画像を生成する。
    pub fn make_clock_img(&mut self, size: Size<Pixels>) -> Img {
        let mut clock_img = if self.is_cached(size) {
            let Some(ref img) = self.base_image else {
                unreachable!("checked done in is_cashed.");
            };
            img.clone()
        } else {
            let img = self.make_clock_base(size);
            self.base_image = Some(img.clone());
            self.size = Some(size);
            img
        };
        let center = Point::new(
            size.width.to_f64() as f32 / 2.0,
            size.height.to_f64() as f32 / 2.0,
        );
        let radius = size.width.min(size.height).to_f64() as f32 / 2.0;
        self.draw_hands(&mut clock_img, center, radius);

        let frame = Frame::new(clock_img);
        let render_image = Arc::new(RenderImage::new([frame]));
        img(render_image)
    }

    /// 時計の背景を生成する。
    fn make_clock_base(&self, size: Size<Pixels>) -> ImageBuffer {
        let center = Point::new(
            size.width.to_f64() as f32 / 2.0,
            size.height.to_f64() as f32 / 2.0,
        );
        let radius = size.width.min(size.height).to_f64() as f32 / 2.0;
        let back_ground_color = Rgba::from([0, 0, 0, 0]);
        let mut clock_img = ImageBuffer::from_pixel(
            size.width.to_f64() as u32,
            size.height.to_f64() as u32,
            back_ground_color,
        );
        self.draw_clock_background(
            &mut clock_img,
            center,
            radius,
            Rgba::from([0, 255, 255, 255]),
        );
        self.draw_major_scale(&mut clock_img, center, radius);
        self.draw_center_pin(&mut clock_img, center, radius);
        self.draw_miner_scale(&mut clock_img, center, radius);
        self.draw_numbers(&mut clock_img, center, radius);
        clock_img
    }

    fn draw_clock_background(
        &self,
        img: &mut ImageBuffer,
        center: Point<f32>,
        radius: f32,
        back_ground_color: Rgba<u8>,
    ) {
        let center = (center.x as i32, center.y as i32);
        draw_filled_circle_mut(img, center, radius as i32, back_ground_color);
    }

    /// 大目盛りの描画を行う
    fn draw_major_scale(&self, img: &mut ImageBuffer, center: Point<f32>, radius: f32) {
        let width = Point::new(radius / 10.0, 0.0);
        let height = Point::new(0.0, radius / 30.0);
        let first_point = center + Point::new(radius * 0.98, height.y / 2.0);
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
                .rotate_points(points, center, angle)
                .iter()
                .map(|p| Point::new(p.x as i32, p.y as i32))
                .collect();
            draw_polygon_mut(img, &rotated_points, Rgba::from([0, 0, 0, 255]));
        }
    }

    /// センターピンの描画を行う
    fn draw_center_pin(&self, img: &mut ImageBuffer, center: Point<f32>, radius: f32) {
        let center = (center.x as i32, center.y as i32);
        draw_filled_circle_mut(
            img,
            center,
            (radius / 20.0) as i32,
            Rgba::from([0, 0, 0, 255]),
        );
    }

    /// 小目盛りの描画を行う
    fn draw_miner_scale(&self, img: &mut ImageBuffer, center: Point<f32>, radius: f32) {
        let first_point = center + Point::new(radius * 0.93, 0.0);
        let scale_radius = radius / 60.0;
        let indexs = (0..60).filter(|i| i % 5 != 0);
        for i in indexs {
            let angle = (2.0 * PI) / 60.0 * i as f32;
            let scale_center = self.rotate_points([first_point].into_iter(), center, angle)[0];
            draw_filled_circle_mut(
                img,
                (scale_center.x as i32, scale_center.y as i32),
                scale_radius as i32,
                Rgba::from([0, 0, 0, 255]),
            );
        }
    }

    /// 文字盤の数字を描画する
    fn draw_numbers(&self, img: &mut ImageBuffer, center: Point<f32>, radius: f32) {
        let font = FontArc::try_from_slice(include_bytes!("../resource/Fraunces.ttf")).unwrap();
        let font_size_base = radius / 3.0;
        let first_point_center = center + Point::new(radius * 0.7, 0.0);
        for i in 0..12 {
            let text = format!("{}", (i + 2) % 12 + 1);
            let font_size = if text.len() == 1 {
                font_size_base
            } else {
                font_size_base * 0.9
            };
            let angle = (2.0 * PI) / 12.0 * i as f32;
            let point_center =
                self.rotate_points([first_point_center].into_iter(), center, angle)[0];
            let text_size = text_size(font_size, &font, &text);
            let text_point =
                point_center - Point::new(text_size.0 as f32 / 2.0, text_size.1 as f32 / 2.0);
            draw_text_mut(
                img,
                Rgba::from([0, 0, 0, 255]),
                text_point.x as i32,
                text_point.y as i32,
                font_size,
                &font,
                &text,
            );
        }
    }

    /// 時計の針の描画を行う
    fn draw_hands(&self, img: &mut ImageBuffer, center: Point<f32>, radius: f32) {
        let (hour, minute, second, millis) = self.get_current_time();
        self.draw_long_hand(img, center, radius, minute, second);
        self.draw_short_hand(img, center, radius, hour, minute);
        self.draw_second_hand(img, center, radius, second, millis);
    }

    /// 長針を描画する
    fn draw_long_hand(
        &self,
        img: &mut ImageBuffer,
        center: Point<f32>,
        radius: f32,
        minute: u32,
        second: u32,
    ) {
        let origin_point = Point::new(center.x + radius * 0.9, center.y);
        let second_point = center + Point::new(radius * 0.3, radius * 0.03);
        let force_point = center + Point::new(radius * 0.3, radius * -0.03);
        let origin_points = [origin_point, second_point, center, force_point];
        let second_angle = second as f32 * (2.0 * PI) / (60.0 * 60.0);
        let minute_angle = minute as f32 * (2.0 * PI) / 60.0;
        let angle_from_12 = minute_angle + second_angle;
        let mut angle = angle_from_12 - ((2.0 * PI) / 4.0);
        if angle < 0.0 {
            angle = (2.0 * PI) + angle;
        }
        let rotated: Vec<Point<i32>> = self
            .rotate_points(origin_points.into_iter(), center, angle)
            .into_iter()
            .map(|p| Point::new(p.x as i32, p.y as i32))
            .collect();
        draw_polygon_mut(img, &rotated, Rgba::from([0, 0, 0, 255]));
    }

    /// 短針を描画する
    fn draw_short_hand(
        &self,
        img: &mut ImageBuffer,
        center: Point<f32>,
        radius: f32,
        hour: u32,
        minute: u32,
    ) {
        let origin_point = Point::new(center.x + radius * 0.6, center.y);
        let second_point = center + Point::new(radius * 0.2, radius * 0.04);
        let force_point = center + Point::new(radius * 0.2, radius * -0.04);
        let origin_points = [origin_point, second_point, center, force_point];
        let minute_angle = minute as f32 * (2.0 * PI) / (60.0 * 12.0);
        let hour = if hour > 12 { hour - 12 } else { hour };
        let hour_angle = hour as f32 * (2.0 * PI) / 12.0;
        let angle_from_12 = hour_angle + minute_angle;
        let mut angle = angle_from_12 - ((2.0 * PI) / 4.0);
        if angle < 0.0 {
            angle = (2.0 * PI) + angle
        }
        let rotated: Vec<Point<i32>> = self
            .rotate_points(origin_points.into_iter(), center, angle)
            .into_iter()
            .map(|p| Point::new(p.x as i32, p.y as i32))
            .collect();
        draw_polygon_mut(img, &rotated, Rgba::from([0, 0, 0, 255]));
    }

    /// 秒針を描画する
    fn draw_second_hand(
        &self,
        img: &mut ImageBuffer,
        center: Point<f32>,
        radius: f32,
        second: u32,
        millis: u32,
    ) {
        let origin_point = Point::new(center.x + radius * 0.95, center.y);
        let second_point = center + Point::new(radius * 0.2, radius * 0.01);
        let force_point = center + Point::new(radius * 0.2, radius * -0.01);
        let origin_points = [origin_point, second_point, center, force_point];
        let second_angle = second as f32 * (2.0 * PI) / 60.0;
        let millis_angle = millis as f32 * (2.0 * PI) / (60.0 * 1000.0);
        let angle_from_12 = second_angle + millis_angle;
        let mut angle = angle_from_12 - ((2.0 * PI) / 4.0);
        if angle < 0.0 {
            angle += 2.0 * PI;
        }
        let rotated: Vec<Point<i32>> = self
            .rotate_points(origin_points.into_iter(), center, angle)
            .into_iter()
            .map(|p| Point::new(p.x as i32, p.y as i32))
            .collect();
        draw_polygon_mut(img, &rotated, Rgba::from([0, 0, 0, 255]));
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

    fn get_current_time(&self) -> (u32, u32, u32, u32) {
        use chrono::Timelike;
        let now = chrono::Local::now();
        (
            now.hour(),
            now.minute(),
            now.second(),
            now.timestamp_subsec_millis(),
        )
    }

    fn is_cached(&self, size: Size<Pixels>) -> bool {
        if self.base_image.is_none() {
            return false;
        };
        if let Some(size_cached) = self.size {
            if size_cached == size {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

impl Render for ClockWindow {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let win_size = window.bounds().size;
        cx.spawn(async |_, cx: &mut AsyncApp| {
            std::thread::sleep(std::time::Duration::from_millis(40));
            cx.refresh().unwrap();
        })
        .detach();
        div().child(self.make_clock_img(win_size))
    }
}
