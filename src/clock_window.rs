use gpui::{div, prelude::*, rgb};

/// 時計を表示するコンテキスト
pub struct ClockWindow;
impl Render for ClockWindow {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().text_color(rgb(0xdddddd)).child("hello haridokei")
    }
}
