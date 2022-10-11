use std::rc::Rc;

use core_ui::{layer::Layer, event_dispatcher::TermEventDispatcher};
use crossterm::event::Event;
use sport_core::events;

pub enum UiEvent {
    TermEvent(Event),
    AddLayer(Box<dyn Layer>),
    MenuSwitched(usize),
}
pub type UiEvents = events::Events<UiEvent>;
pub type SharedUiEvents = Rc<UiEvents>;

pub struct UiEventSender {
    ui_events_manager: SharedUiEvents,
}

impl UiEventSender {
    pub fn new(ui_events_manager: SharedUiEvents) -> Self {
        Self { ui_events_manager }
    }

    pub fn send_add_layer_event(&mut self, layer: Box<dyn Layer>) {
        self.ui_events_manager.add_event(UiEvent::AddLayer(layer));
    }

    pub fn send_menu_switched_event(&mut self, index: usize) {
        self.ui_events_manager.add_event(UiEvent::MenuSwitched(index));
    }

    pub fn ui_events_manager(&self) -> SharedUiEvents {
        self.ui_events_manager.clone()
    }
}

pub trait UiEventDispatcher: TermEventDispatcher {
    fn dispatch_ui_event(&mut self, event: &UiEvent) -> bool {
        return match event {
            UiEvent::TermEvent(event) => self.dispatch_term_event(event),
            UiEvent::MenuSwitched(index) => self.on_menu_switched(*index),
            _ => false,
        }
    }

    fn on_menu_switched(&mut self, _index: usize) -> bool { false }
}
