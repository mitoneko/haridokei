use std::f32::consts::PI;

use gpui::{
    App, Font, FontFallbacks, FontFeatures, FontStyle, FontWeight, Path, PathBuilder, Pixels,
    Point, Rgba, Size, TextAlign, TextRun, Window, px, rgba,
};
use log::error;

pub struct ClockBaseImage {
    size: Size<Pixels>,
    center: Point<Pixels>,
    radius: f32,
    clock_background_color: Rgba,
    img_path: Vec<(Path<Pixels>, Rgba)>,
}

impl ClockBaseImage {
    pub fn new(size: Size<Pixels>, clock_background_color: [u8; 4]) -> Self {
        let center = Point::new(size.width / 2.0, size.height / 2.0);
        let radius = size.width.min(size.height).as_f32() / 2.0;
        let color_value = clock_background_color
            .iter()
            .fold(0, |acc, &c| (acc << 8) + c as u32);
        let back_ground_color = gpui::rgba(color_value);
        let img_path = Vec::new();

        let mut obj = Self {
            size,
            center,
            radius,
            img_path,
            clock_background_color: back_ground_color,
        };
        obj.make_clock_base();
        obj
    }

    /// 時計背景のイメージを描画する。
    pub fn paint_clock_base(&self, window: &mut gpui::Window, cx: &mut gpui::App) {
        for (path, color) in self.img_path.iter() {
            window.paint_path(path.clone(), *color);
        }
        self.draw_numbers(window, cx);
    }

    pub fn center(&self) -> Point<Pixels> {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_size(&mut self, size: Size<Pixels>) {
        if self.size != size {
            self.size = size;
            self.center = Point::new(size.width / 2.0, size.height / 2.0);
            self.radius = size.width.min(size.height).as_f32() / 2.0;
            self.make_clock_base();
        }
    }

    /// 時計の背景のイメージを生成する。
    fn make_clock_base(&mut self) {
        self.img_path.clear();
        self.draw_clock_background();
        self.draw_major_scale();
        self.draw_center_pin();
        self.draw_miner_scale();
    }

    fn draw_clock_background(&mut self) {
        let mut path_bld = PathBuilder::fill();
        self.draw_circle(&mut path_bld, self.center, self.radius);
        match path_bld.build() {
            Ok(path) => self.img_path.push((path, self.clock_background_color)),
            Err(e) => error!("時計背景の丸の描画に失敗({e})"),
        }
    }

    /// 大目盛りの描画を行う
    fn draw_major_scale(&mut self) {
        let width = Point::new(px(self.radius / 10.0), px(0.0));
        let height = Point::new(px(0.0), px(self.radius / 30.0));
        let first_point = Point::new(px(self.radius * 0.98), height.y / 2.0);
        for i in 0..12 {
            let mut path_bld = PathBuilder::fill();
            path_bld.move_to(first_point);
            path_bld.line_to(first_point - height);
            path_bld.line_to(first_point - height - width);
            path_bld.line_to(first_point - width);
            path_bld.close();
            let angle = (360.0 / 12.0) * i as f32;
            path_bld.rotate(angle);
            path_bld.translate(self.center);
            match path_bld.build() {
                Ok(path) => self.img_path.push((path, gpui::rgba(0x000000ff))),
                Err(e) => error!("大目盛りの描画に失敗({e})"),
            }
        }
    }

    /// センターピンの描画を行う
    fn draw_center_pin(&mut self) {
        let mut path_bld = PathBuilder::fill();
        self.draw_circle(&mut path_bld, self.center, self.radius / 20.0);
        match path_bld.build() {
            Ok(path) => self.img_path.push((path, gpui::rgba(0x000000ff))),
            Err(e) => error!("センターピンの描画に失敗({e})"),
        }
    }

    /// 小目盛りの描画を行う
    fn draw_miner_scale(&mut self) {
        let first_point = Point::new(px(self.radius * 0.93), px(0.0));
        let scale_radius = self.radius / 60.0;
        let indexs = (0..60).filter(|i| i % 5 != 0);
        for i in indexs {
            let mut path_bld = PathBuilder::fill();
            let angle = (360.0 / 60.0) * i as f32;
            self.draw_circle(&mut path_bld, first_point, scale_radius);
            path_bld.rotate(angle);
            path_bld.translate(self.center);
            match path_bld.build() {
                Ok(path) => self.img_path.push((path, gpui::rgba(0x000000ff))),
                Err(e) => error!("小目盛りの描画に失敗({e})"),
            }
        }
    }

    /// 文字盤の数字を描画する
    fn draw_numbers(&self, window: &mut Window, cx: &mut App) {
        const FONT_SIZE_RATE: f32 = 0.25;
        const NUM_POSITION_RATE: f32 = 0.70;
        let font_size = px(self.radius() * FONT_SIZE_RATE);
        let font = Font {
            family: "Fraunces".into(),
            features: FontFeatures::default(),
            fallbacks: Some(FontFallbacks::from_fonts(vec![".SystemUIFont".into()])),
            weight: FontWeight::default(),
            style: FontStyle::default(),
        };
        let num_strs = [
            "12", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11",
        ];
        let mut text_run = TextRun {
            len: 0,
            font,
            color: rgba(0x000000ff).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        for (i, num_str) in num_strs.iter().enumerate() {
            let angle = 2.0 * PI * i as f32 / 12.0;
            text_run.len = num_str.len();
            let shaped_line = window.text_system().shape_line(
                (*num_str).into(),
                font_size,
                std::slice::from_ref(&text_run),
                None,
            );
            let x = (self.radius() * NUM_POSITION_RATE) * f32::sin(angle);
            let y = -(self.radius() * NUM_POSITION_RATE) * f32::cos(angle);
            let mut origin = Point::new(px(x), px(y)) + self.center;
            origin += Point::new(-shaped_line.width() / 2.0, -font_size / 2.0);
            shaped_line
                .paint(
                    origin,
                    font_size,
                    TextAlign::default(),
                    Some(shaped_line.width()),
                    window,
                    cx,
                )
                .unwrap_or_else(|e| error!("文字盤の数字の描画に失敗しました。:({e})"));
        }
    }

    /// 円を描画する
    fn draw_circle(&mut self, path_bld: &mut PathBuilder, center: Point<Pixels>, radius: f32) {
        let point_up = Point::new(px(0.0), px(radius));
        let point_down = Point::new(px(0.0), px(-radius));
        let radii = Point::new(px(radius), px(radius));
        path_bld.move_to(point_up);
        path_bld.arc_to(radii, px(0.0), true, true, point_down);
        path_bld.arc_to(radii, px(0.0), true, true, point_up);
        path_bld.translate(center);
    }
}
