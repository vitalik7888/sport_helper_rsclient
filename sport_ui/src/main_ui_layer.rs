use std::rc::Rc;

use core_ui::{
    components::{Component, EventComponent},
    render::RenderFrame,
    event_dispatcher::{TermEventDispatcher, KeyEventDispatcher},
    component::TerminalEvent,
    layer::Layer
};
use sport_core::controller;
use tui::layout::{Rect, Layout, Direction, Constraint};

use crate::{
    menu::{UiMenu, MenuItem, Page},
    ui_events::SharedUiEvents,
    footer::Footer,
    page_exercises::PageExercises,
    page_account::PageAccount
};

pub struct MainUiLayer {
    is_focused: bool,
    menu: UiMenu,
    page_exercises: PageExercises,
    page_account: PageAccount,
    footer: Footer,
}

impl MainUiLayer {
    pub fn new(controller: Rc<controller::Controller>, ui_events_manager: SharedUiEvents) -> Self {
        let mut s = Self {
            is_focused: false,
            menu: UiMenu::new(),
            page_exercises: PageExercises::new(controller.clone(), ui_events_manager.clone()),
            page_account: PageAccount::new(controller, ui_events_manager.clone()),
            footer: Footer::default(),
        };
        s.switch_menu(MenuItem::Exercises);
        s
    }

    fn current_page_mut(&mut self) -> &mut dyn Page {
        return match self.menu.current_index() {
            MenuItem::Exercises => &mut self.page_exercises,
            MenuItem::Account => &mut self.page_account,
        }
    }

    fn current_page(&self) -> &dyn Page {
        return match self.menu.current_index() {
            MenuItem::Exercises => &self.page_exercises,
            MenuItem::Account => &self.page_account,
        }
    }

    fn pages_mut(&mut self) -> [&mut dyn Page; 2] {
        [&mut self.page_exercises, &mut self.page_account]
    }

    pub fn switch_menu(&mut self, item: MenuItem) {
        self.menu.set_current_index(item);

        self.pages_mut().iter_mut().for_each(|p| {
            p.focus(false);
            p.set_visible(false);
        });
        let current_page = self.current_page_mut();
        current_page.focus(true);
        current_page.set_visible(true);
        let help = format!("{} {}", current_page.commands_help(), "Quit[q]");
        self.footer.content = help;
    }
}

impl Component for MainUiLayer {
    fn is_visible(&self) -> bool {
        true
    }

    fn draw(&self, f: &mut RenderFrame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
                ]
                .as_ref(),
                )
            .split(area);

        self.menu.draw(f, chunks[0]);
        self.current_page().draw(f, chunks[1]);
        self.footer.draw(f, chunks[2]);
    }
}

impl TermEventDispatcher for MainUiLayer {}
impl KeyEventDispatcher for MainUiLayer {
    fn on_char(&mut self, c: &char) -> bool {
        if let Some(index) = c.to_digit(10) {
            if index > 0 {
                let index = index as usize - 1;
                if index < self.menu.items_count() {
                    self.switch_menu(MenuItem::from(index));
                    return true;
                }
            }
        }
        false
    }
}

impl EventComponent for MainUiLayer {
    fn focus(&mut self, value: bool) {
        self.is_focused = value;
        self.menu.focus(value);
        if !value {
            self.page_exercises.focus(false);
            self.page_account.focus(false);
            self.footer.content.clear();
        } else {
            self.switch_menu(MenuItem::Exercises);
        }
    }

    fn on_focus(&self) -> bool {
        self.is_focused
    }

    fn on_term_event(&mut self, event: &TerminalEvent) -> bool {
        if !self.on_focus() {
            return false;
        }
        let event_consumed = self.dispatch_term_event(event);
        event_consumed || self.current_page_mut().on_term_event(event)
    }
}
impl Layer for MainUiLayer {
    fn is_modal(&self) -> bool { false }

    fn is_remove_requested(&self) -> bool { false }
}

