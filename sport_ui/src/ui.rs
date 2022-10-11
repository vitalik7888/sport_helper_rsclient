pub use std::rc::Rc;

use core_ui::{component::TerminalEvent, layer::{UiLayers, Layer}, render::Render, theme::UiTheme};
use sport_core::controller;
use tui::{widgets, style::{Style, Color}};

use crate::{ui_events::{UiEvents, UiEvent, SharedUiEvents}, main_ui_layer::MainUiLayer};

pub struct Ui {
    need_stop: bool,
    ui_events_manager: SharedUiEvents,
    layers: UiLayers,
}

#[allow(dead_code)]
impl Ui {
    pub fn new(controller: Rc<controller::Controller>) -> Self {
        let ui_events_manager = Rc::new(UiEvents::default());
        let mut s = Self {
            ui_events_manager: ui_events_manager.clone(),
            need_stop: false,
            layers: UiLayers::new(),
        };

        let main_layer = Box::new(MainUiLayer::new(controller, ui_events_manager.clone()));
        s.add_layer(main_layer);
        s
    }

    pub fn stop(&mut self) {
        self.need_stop = true;
    }

    pub fn process(&mut self, render: &Render) {
        render.draw(Box::new(|f| {
            let size = f.size();

            f.render_widget(
                // TODO move to layer
                widgets::Block::default().style(Style::default().bg(Color::Red).fg(Color::Black)),
                size,
                );
            self.layers.get_all().for_each(|l| l.draw(f, size));
        })).expect("Can`t draw ui");

        self.process_events();
        self.clean();
    }

    fn process_events(&mut self) {
        let ui_events = self.ui_events_manager.get_all();
        for ui_event in ui_events {
            match ui_event {
                UiEvent::AddLayer(layer) => self.add_layer(layer),
                // UiEvent::MenuSwitched(_) => { self.send_event(ui_event); },
                _ => {},
            }
        }
    }

    fn clean(&mut self) {
        self.layers.clean();
    }

    pub fn add_layer(&mut self, mut layer: Box<dyn Layer>) {
        layer.focus(true);
        layer.set_visible(true);
        self.layers.push(layer);
    }

    pub fn send_term_event(&mut self, e: &TerminalEvent) -> bool {
        for l in self.layers.get_all_mut().rev() {
            if l.on_focus() && l.is_visible() {
                if l.is_modal() {
                    return l.on_term_event(e);
                }
                if l.on_term_event(e) {
                    return true;
                }
            }
        }
        false
    }

    pub fn apply_theme(&mut self, theme: &UiTheme) {
        self.layers.get_all_mut().for_each(|l| l.apply_theme(theme));
    }
}
