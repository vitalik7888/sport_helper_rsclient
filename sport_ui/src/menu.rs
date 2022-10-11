use std::rc::Rc;

use core_ui::{
    event_dispatcher::{TermEventDispatcher, KeyEventDispatcher},
    components::{EventComponent, Component},
    render::RenderFrame, layer::Layer,
};
use crossterm::event::Event;
use sport_core::controller;
use tui::layout::Rect;

use crate::{tabs::{UiTabs, UiTab}, ui_events::SharedUiEvents};


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MenuItem {
    Exercises,
    Account,
}

impl From<MenuItem> for usize {
    fn from(i: MenuItem) -> usize {
        match i {
            MenuItem::Exercises => 0,
            MenuItem::Account => 1,
        }
    }
}

impl From<usize> for MenuItem {
    fn from(i: usize) -> MenuItem {
        match i {
            0 => MenuItem::Exercises,
            1 => MenuItem::Account,
            _ => unreachable!("Can`t convert index to MenuTab"),
        }
    }
}

pub struct UiMenu {
    tabs: UiTabs,
    is_focused: bool,
}

#[allow(dead_code)]
impl UiMenu {
    pub fn new() -> Self {
        Self {
            tabs: UiTabs::new(
                      vec![
                      UiTab::new(String::from("Exercise")),
                      UiTab::new(String::from("Account")),
                      ]),
                      is_focused: false,
        }
    }

    pub fn tabs(&self) -> &UiTabs {
        &self.tabs
    }

    pub fn set_current_index(&mut self, value: MenuItem) {
        self.tabs.set_current_tab(value.into())
    }

    pub fn current_index(&self) -> MenuItem {
        self.tabs.current_index().into()
    }

    pub fn items_count(&self) -> usize {
        self.tabs.tabs_count()
    }
}

impl TermEventDispatcher for UiMenu {}
impl KeyEventDispatcher for UiMenu {}

impl EventComponent for UiMenu {
    fn focus(&mut self, value: bool) {
        self.is_focused = value;
        self.tabs.focus(value);
    }

    fn on_focus(&self) -> bool {
        self.is_focused
    }

    fn on_term_event(&mut self, e: &Event) -> bool {
        if self.on_focus() {
            return self.tabs.on_term_event(e);
        }
        false
    }
}

impl Component for UiMenu {
    fn draw(&self, f: &mut RenderFrame, area: Rect) {
        self.tabs.draw(f, area);
    }
}

pub trait Page: Layer { // TODO Move from here
    fn new(controller: Rc<controller::Controller>, ui_events_manager: SharedUiEvents) -> Self where Self: Sized;
    // fn set_contoller(controller: Rc<controller::Controller>) {}

    fn commands_help(&self) -> &str { "" }
}

