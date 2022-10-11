use std::{
    time::{Duration, Instant}, 
    rc::Rc,
};

use crossterm::event::KeyCode;
use core_ui::{render, theme::UiTheme, component::TerminalEvent};
use sport_core::{events, controller::Controller, config::{KeyMap, Config}};
use sport_ui::ui;

// For current moment app working with server only as one instance at time 
pub struct App {
    events_manager: Rc<events::Events<TerminalEvent>>,
    need_quit: bool,
    is_focused: bool,
    ui: ui::Ui,
    keymap: KeyMap,
}

#[allow(dead_code)]
impl App {
    pub fn new(cfg: Config, keymap: KeyMap) -> Self {
        let host = format!("http://{}:{}", cfg.server.host.to_string(), cfg.server.port);
        let events_manager = Rc::new(events::Events::default());
        let controller = Rc::new(Controller::new(&host, cfg, keymap.clone()));
        Self {
            need_quit: false,
            is_focused: true,
            keymap,
            ui: ui::Ui::new(controller),
            events_manager,
        }
    }

    pub fn run(&mut self, tick_rate: Duration) {
        let mut last_tick = Instant::now();

        let render = render::Render::default();
        render.clear().expect("Can`t clear render terminal");

        loop {
            if self.is_focused {
                self.ui.process(&render);
            }

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if let Ok(event) = events::read_system_event(timeout) {
                self.events_manager.add_event(event);
            }

            self.process_events();

            if self.need_quit {
                break;
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn apply_theme(&mut self, theme: &UiTheme) {
        self.ui.apply_theme(theme);
    }

    fn on_key(&mut self, c: char) {
        if c == self.keymap.quit {
            self.need_quit = true;
        }
    }

    async fn on_tick(&mut self) {}

    fn process_events(&mut self) {
        for event in self.events_manager.get_all() {
            match event {
                crossterm::event::Event::FocusGained => self.is_focused = true,
                crossterm::event::Event::FocusLost => self.is_focused = false,
                _ => {},
            }
            if !self.ui.send_term_event(&event) {
                if let crossterm::event::Event::Key(key) = event {
                    match key.code {
                        KeyCode::Char(c) => self.on_key(c),
                        _ => {}
                    }
                }
            }
        }
    }
}

