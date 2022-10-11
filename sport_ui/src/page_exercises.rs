use std::rc::Rc;

use core_ui::{components::{Component, EventComponent}, event_dispatcher::{KeyEventDispatcher, TermEventDispatcher}, layer::Layer};
use sport_core::controller;

use crate::{exercises_table::UiExercisesTable, ui_events::SharedUiEvents, menu::Page};

pub struct PageExercises {
    exercises_table: UiExercisesTable,
    is_visible: bool,
    is_focused: bool,
}

impl PageExercises {
}

impl Component for PageExercises {
    fn draw(&self, f: &mut core_ui::render::RenderFrame, area: tui::layout::Rect) {
        if !self.is_visible {
            return;
        }
        self.exercises_table.draw(f, area);
    }

    fn is_visible(&self) -> bool { self.is_visible }

    fn set_visible(&mut self, value: bool) { self.is_visible = value; }
}

impl TermEventDispatcher for PageExercises {}
impl KeyEventDispatcher for PageExercises {}

impl EventComponent for PageExercises {
    fn focus(&mut self, value: bool) { 
        self.is_focused = value;
        self.exercises_table.focus(value);
    }

    fn on_focus(&self) -> bool { self.is_focused }

    fn on_term_event(&mut self, event: &core_ui::component::TerminalEvent) -> bool { 
        if self.on_focus() {
            if self.dispatch_term_event(event) {
                return true;
            }
            return self.exercises_table.dispatch_term_event(event);
        }
        false
    }
}

impl Layer for PageExercises { }

impl Page for PageExercises {
    fn new(controller: Rc<controller::Controller>, ui_events_manager: SharedUiEvents) -> Self {
        Self { 
            exercises_table: UiExercisesTable::new(controller, ui_events_manager),
            is_visible: true,
            is_focused: false,
        } 
    }

    fn commands_help(&self) -> &str {
        if self.exercises_table.on_focus() {
            self.exercises_table.commands_help()
        } else {
            ""
        }
    }
}
