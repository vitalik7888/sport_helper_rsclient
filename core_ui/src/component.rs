use tui::layout;

use crate::{theme::UiTheme, event_dispatcher::TermEventDispatcher, render};

pub type TerminalEvent = crossterm::event::Event;

#[allow(unused_variables)]
pub trait Component
{
    fn is_visible(&self) -> bool { true }
    fn set_visible(&mut self, value: bool) {}

    fn draw(&self, f: &mut render::RenderFrame, area: layout::Rect);

    fn apply_theme(&mut self, theme: &UiTheme) {} // Result<(), ThemeError> ?
}

#[allow(unused_variables)]
pub trait EventComponent: Component + TermEventDispatcher {
    fn focus(&mut self, value: bool) {}
    fn on_focus(&self) -> bool { false }

    fn on_term_event(&mut self, event: &TerminalEvent) -> bool { 
        if self.on_focus() {
            return self.dispatch_term_event(event);
        }
        false
    }
}
