use core_ui::{components::Component, label::UiLabel};
use tui::{widgets, layout::{Layout, Direction, Constraint}};


pub struct Footer {
    pub content: String,
}

impl Default for Footer {
    fn default() -> Self {
        Self { content: "".to_owned() }
    }
}

impl Component for Footer {
    fn draw(&self, f: &mut core_ui::render::RenderFrame, area: tui::layout::Rect) {
        f.render_widget(
            widgets::Block::default()
            .title("Help")
            .borders(widgets::Borders::ALL),
            area,
            );
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Length(1)].as_ref())
            .split(area);
        UiLabel::new(&self.content).draw(f, chunks[0]);
    }
}

