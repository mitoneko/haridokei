use std::sync::{Arc, Mutex};

use chrono::{Local, Timelike};
use gpui::{App, Bounds, PathBuilder, Pixels, Point, Style, Window, fill, prelude::*, px, rgba};

use crate::clock_base_image::ClockBaseImage;

/// 時計を表示するためのElement
pub struct Clock {
    base_image: Arc<Mutex<ClockBaseImage>>,
    center: Point<Pixels>,
    radius: Pixels,
}

impl Clock {
    pub fn new(base_image: Arc<Mutex<ClockBaseImage>>) -> Self {
        let (center, radius) = {
            let base_image = base_image.lock().unwrap();
            (base_image.center(), px(base_image.radius()))
        };
        Self {
            base_image,
            center,
            radius,
        }
    }

    /// 秒針の描画
    fn paint_second_hand(
        &mut self,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        second: f32,
        millis: f32,
    ) {
        let arg = (360. / 60.) * second + (360. / (60. * 1000.)) * millis;
        let mut path = PathBuilder::fill();
        path.move_to(bounds.origin);
        path.line_to(Point::new(-(self.radius * 0.01), -(self.radius * 0.3)));
        path.line_to(Point::new(px(0.0), -(self.radius * 0.9)));
        path.line_to(Point::new(self.radius * 0.01, -(self.radius * 0.3)));
        path.close();
        path.rotate(arg);
        path.translate(self.center);
        let path = path.build().unwrap();
        window.paint_path(path, rgba(0x000000ff));
    }

    /// 長針の描画
    fn paint_long_hand(
        &mut self,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        minute: f32,
        second: f32,
        millis: f32,
    ) {
        let arg = (360. / 60.) * minute
            + ((360. / (60. * 60.)) * second + (360. / (60. * 60. * 1000.)) * millis);
        let mut path = PathBuilder::fill();
        path.move_to(bounds.origin);
        path.line_to(Point::new(-(self.radius * 0.04), -(self.radius * 0.3)));
        path.line_to(Point::new(px(0.0), -(self.radius * 0.9)));
        path.line_to(Point::new(self.radius * 0.04, -(self.radius * 0.3)));
        path.close();
        path.rotate(arg);
        path.translate(self.center);
        let path = path.build().unwrap();
        window.paint_path(path, rgba(0x000000ff));
    }

    /// 短針の描画
    fn paint_short_hand(
        &mut self,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        hour: f32,
        minute: f32,
        second: f32,
    ) {
        let arg = (360. / 12.) * hour
            + ((360. / (12. * 60.)) * minute + (360. / (12. * 60. * 60.)) * second);
        let mut path = PathBuilder::fill();
        path.move_to(bounds.origin);
        path.line_to(Point::new(-(self.radius * 0.06), -(self.radius * 0.2)));
        path.line_to(Point::new(px(0.0), -(self.radius * 0.55)));
        path.line_to(Point::new(self.radius * 0.06, -(self.radius * 0.2)));
        path.close();
        path.rotate(arg);
        path.translate(self.center);
        let path = path.build().unwrap();
        window.paint_path(path, rgba(0x000000ff));
    }
}

impl Element for Clock {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let now = Local::now();
        let (_, hour) = now.hour12();
        let hour = (hour % 12) as f32;
        let minute = now.minute() as f32;
        let second = now.second() as f32;
        let millis = now.timestamp_subsec_millis() as f32;

        // windowのクリア
        let size = bounds.size;
        let bounds = Bounds {
            origin: Point::new(px(0.0), px(0.0)),
            size,
        };
        let rect = fill(bounds, rgba(0x00000000));
        window.paint_quad(rect);
        // 時計のベースイメージを描画
        self.base_image.lock().unwrap().paint_clock_base(window, cx);
        // 針の描画
        self.paint_second_hand(bounds, window, second, millis);
        self.paint_long_hand(bounds, window, minute, second, millis);
        self.paint_short_hand(bounds, window, hour, minute, second);
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let style = Style {
            size: gpui::Size::full(),
            ..Default::default()
        };
        let layoutid = window.request_layout(style, None, cx);
        (layoutid, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: gpui::Bounds<gpui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }
}

impl IntoElement for Clock {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}
