use tui::{widgets::{self, Wrap, Paragraph}, layout::{Layout, Direction, Constraint, self}, style::{Color, Style}};

use crate::{
    component::{Component, EventComponent},
    event_dispatcher::{KeyEventDispatcher, TermEventDispatcher},
    layer::Layer,
    render::RenderFrame,
    utils,
};


pub struct UiMessageBox
{
    title: String,
    text: String,
    message_type: UiMessageBoxType,
    is_focused: bool,
    is_visible: bool,
    is_modal: bool,
    is_removing_needed: bool,
    on_accept: Option<Box<dyn FnOnce()>>
}

impl UiMessageBox {
    pub fn new(title: &str, text: String, message_type: UiMessageBoxType) -> Self { 
        Self { 
            text,
            title: title.to_owned(),
            message_type,
            is_focused: true, 
            is_visible: true,
            is_modal: true,
            is_removing_needed: false,
            on_accept: None,
        } 
    }

    pub fn info(title: &str, text: String) -> Self {
        Self::new(title, text, UiMessageBoxType::Info)
    }

    pub fn warn(title: &str, text: String) -> Self {
        Self::new(title, text, UiMessageBoxType::Warn)
    }

    pub fn err(title: &str, text: String) -> Self {
        Self::new(title, text, UiMessageBoxType::Error)
    }

    fn close(&mut self) {
        self.is_removing_needed = true;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_on_accept(&mut self, f: Box<dyn FnOnce()>) {
        self.on_accept = Some(f);
    }

    fn accept(&mut self) {
        if let Some(f) = self.on_accept.take() {
            f();
        }
    }
}

impl TermEventDispatcher for UiMessageBox {}

impl KeyEventDispatcher for UiMessageBox { 
    fn on_esc(&mut self) -> bool {
        self.close();
        true
    }

    fn on_enter(&mut self) -> bool {
        self.accept();
        self.close();
        true
    }

    fn on_char(&mut self, c: &char) -> bool {
        if *c == 'q' {
            self.close()
        }
        true
    }
}

impl Component for UiMessageBox {
    fn draw(&self, f: &mut RenderFrame, area: layout::Rect) {
        let (color, title) = match self.message_type {
            UiMessageBoxType::Info => (Color::White, "Info"),
            UiMessageBoxType::Warn => (Color::Yellow, "Warning"),
            UiMessageBoxType::Error => (Color::Red, "Error"),
        };
        let area = utils::centered_rect(50, 50, area);
        let block = widgets::Block::default()
            .title(format!("{} message: `{}`; -> Commands(Close: [q] | [ESC] | [ENTER])", title, self.title))
            .border_style(Style::default().fg(color))
            .borders(widgets::Borders::ALL);
        f.render_widget(tui::widgets::Clear, area);
        f.render_widget(block, area);

        let l = Layout::default()
            .direction(Direction::Vertical)
            .constraints([ Constraint::Length(1), Constraint::Min(2)].as_ref())
            .split(area);
        let l = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)].as_ref())
            .split(l[1]);

        let p = Paragraph::new(self.text.clone()).wrap(Wrap { trim: true });
        f.render_widget(p, l[1]);
    }

    fn is_visible(&self) -> bool { self.is_visible }

    fn set_visible(&mut self, value: bool) { self.is_visible = value; }
}

impl EventComponent for UiMessageBox {
    fn focus(&mut self, value: bool) { self.is_focused = value; }
    fn on_focus(&self) -> bool { self.is_focused }
}

impl Layer for UiMessageBox {
    fn is_modal(&self) -> bool { self.is_modal }
    fn set_modal(&mut self, is_modal: bool) { self.is_modal = is_modal; }

    fn is_remove_requested(&self) -> bool { self.is_removing_needed }
}

pub enum UiMessageBoxType {
    Info,
    Warn,
    Error,
}
