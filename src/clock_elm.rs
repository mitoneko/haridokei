use std::sync::Arc;

use gpui::{Corners, Element, IntoElement, RenderImage, Style};

/// 時計を表示するためのElement
pub struct Clock {
    base_image: Arc<RenderImage>,
}

impl Clock {
    pub fn new(base_image: Arc<RenderImage>) -> Self {
        Self { base_image }
    }
}

impl Element for Clock {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
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
            .unwrap();
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
        window: &mut gpui::Window,
        cx: &mut gpui::App,
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
        _window: &mut gpui::Window,
        _cx: &mut gpui::App,
    ) -> Self::PrepaintState {
    }
}

impl IntoElement for Clock {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}
