use tui::{layout, widgets, buffer::Buffer, style::Style};

use crate::{components, render};


#[derive(Default, Clone, Copy)]
pub struct UiLabel<'a> {
    text: &'a str,
}

impl<'a> components::Component for UiLabel<'a> {
    fn draw(&self, f: &mut render::RenderFrame, area: layout::Rect) {
         f.render_widget(self.clone(), area); // FIXME clone
    }
}

impl<'a> widgets::Widget for UiLabel<'a> {
    fn render(self, area: layout::Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, Style::default());
    }
}

#[allow(dead_code)]
impl<'a> UiLabel<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn text(mut self, text: &'a str) -> UiLabel<'a> {
        self.text = text;
        self
    }
}


