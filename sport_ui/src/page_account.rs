use std::rc::Rc;

use core_ui::{
    components::{Component, EventComponent},
    event_dispatcher::{KeyEventDispatcher, TermEventDispatcher},
    layer::Layer
};
use sport_core::controller;

use crate::{ui_events::SharedUiEvents, menu::Page};

pub struct PageAccount {
    is_visible: bool,
    is_focused: bool,
}

impl Component for PageAccount {
    fn draw(&self, _f: &mut core_ui::render::RenderFrame, _area: tui::layout::Rect) {
        if !self.is_visible {
            return;
        }
    }

    fn is_visible(&self) -> bool { self.is_visible }

    fn set_visible(&mut self, value: bool) { self.is_visible = value; }
}

impl TermEventDispatcher for PageAccount {}
impl KeyEventDispatcher for PageAccount {}

impl EventComponent for PageAccount {
    fn focus(&mut self, value: bool) { 
        self.is_focused = value;
    }

    fn on_focus(&self) -> bool { self.is_focused }

    fn on_term_event(&mut self, event: &core_ui::component::TerminalEvent) -> bool { 
        if self.on_focus() {
            return self.dispatch_term_event(event);
        }
        false
    }
}

impl Layer for PageAccount { }

impl Page for PageAccount {
    fn new(_controller: Rc<controller::Controller>, _ui_events_manager: SharedUiEvents) -> Self {
        Self { 
            is_visible: true,
            is_focused: false,
        } 
    }

    fn commands_help(&self) -> &str {
        ""
    }
}
