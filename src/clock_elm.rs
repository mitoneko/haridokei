use std::sync::Arc;

use chrono::{Local, Timelike};
use gpui::{
    App, Bounds, Corners, PathBuilder, Pixels, Point, RenderImage, Style, Window, prelude::*, px,
    rgba,
};
use log::error;

/// 時計を表示するためのElement
pub struct Clock {
    base_image: Arc<RenderImage>,
    center: Point<Pixels>,
    radius: Pixels,
}

impl Clock {
    pub fn new(base_image: Arc<RenderImage>) -> Self {
        Self {
            base_image,
            center: Point::default(),
            radius: Pixels::default(),
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
    ) {
        let arg = (360. / 60.) * minute + (360. / (60. * 60.)) * second;
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
    ) {
        let arg = (360. / 12.) * hour + (360. / (12. * 60.)) * minute;
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
        _cx: &mut App,
    ) {
        // 時計のベースイメージを描画
        window
            .paint_image(
                bounds,
                Corners::default(),
                self.base_image.clone(),
                0,
                false,
            )
            .unwrap_or_else(|e| error!("背景の描画に失敗:{}", e));
        self.center = Point::new(bounds.size.width, bounds.size.height) / 2.0;
        self.radius = if bounds.size.width < bounds.size.height {
            bounds.size.width / 2.0
        } else {
            bounds.size.height / 2.0
        };
        let now = Local::now();
        let (_, hour) = now.hour12();
        let hour = (hour % 12) as f32;
        let minute = now.minute() as f32;
        let second = now.second() as f32;
        let millis = now.timestamp_subsec_millis() as f32;
        self.paint_second_hand(bounds, window, second, millis);
        self.paint_long_hand(bounds, window, minute, second);
        self.paint_short_hand(bounds, window, hour, minute);
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
