use std::rc::Rc;

use core_ui::{
    render::RenderFrame,
    components::{TextEdit, Component, EventComponent},
    utils,
    event_dispatcher::{KeyEventDispatcher, TermEventDispatcher},
    message_box::UiMessageBox, layer::Layer, validators::StrValidator
};
use crossterm::event::Event;
use sport_core::controller;
use sport_core_db::entity;
use tui::{layout::{Rect, Constraint, Direction, Layout}, widgets};

use crate::ui_events::{UiEventSender, SharedUiEvents};

pub struct UiExerciseEditor {
    controller: Rc<controller::Controller>,
    id: entity::ID,
    name_editor: TextEdit<StrValidator>,
    description_editor: TextEdit<StrValidator>,
    to_insert: bool,
    ui_event_sender: Option<UiEventSender>,
    is_focused: bool,
    is_visible: bool,
    is_removing_needed: bool,
}

impl UiExerciseEditor {
    pub fn new(controller: Rc<controller::Controller>) -> Self {
        Self {
            controller,
            id: 0,
            name_editor: TextEdit::new("Name:", "".to_owned(), StrValidator::new(0, 100)),
            description_editor: TextEdit::new("Description: ", "".to_owned(), StrValidator::default()),
            to_insert: true,
            ui_event_sender: None,
            is_focused: true,
            is_visible: true,
            is_removing_needed: false,
        }
    }

    pub fn data(&self) -> entity::Exercise {
        entity::Exercise::new(self.id, self.name_editor.text.clone(), self.description_editor.text.clone())
    }

    pub fn load_data(&mut self, data: &entity::Exercise) {
        self.to_insert = false;
        self.id = data.id;
        self.name_editor.text = data.name.to_owned();
        self.description_editor.text = data.description.to_owned();
    }

    pub fn set_ui_events_manager(&mut self, ui_events_manager: SharedUiEvents) {
        self.ui_event_sender = Some(UiEventSender::new(ui_events_manager));
    }

    fn close(&mut self) {
        self.set_visible(false);
        self.is_removing_needed = true;
    }

    fn current_editor(&mut self) -> &mut TextEdit<StrValidator> {
        let is_name_focused = self.name_editor.on_focus();
        if is_name_focused { &mut self.name_editor } else { &mut self.description_editor }
    }
}

impl Component for UiExerciseEditor {
    fn draw(&self, f: &mut RenderFrame, area: Rect) {
        if !self.is_visible() {
            return;
        }
        let area = utils::centered_rect(50, 50, area);
        let block = widgets::Block::default()
            .title(r#"Exercise;  -> Commands(Exit: [q], Reject: [ESC], Accept: [ENTER])"#)
            .borders(widgets::Borders::ALL);
        f.render_widget(tui::widgets::Clear, area);
        f.render_widget(block, area);

        let l = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                         Constraint::Length(1),
                         Constraint::Min(2),
                         Constraint::Length(1)].as_ref())
            .split(area);
        let l = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)].as_ref())
            .split(l[1]);
        let l = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Min(6)].as_ref())
            .split(l[1]);
        self.name_editor.draw(f, l[0]);
        self.description_editor.draw(f, l[1]);
    }

    fn is_visible(&self) -> bool { self.is_visible }
    fn set_visible(&mut self, value: bool) { self.is_visible = value; }
}

impl KeyEventDispatcher for UiExerciseEditor {
    fn on_tab(&mut self) -> bool {
        let is_name_focused = self.name_editor.on_focus();
        self.description_editor.focus(is_name_focused);
        self.name_editor.focus(!is_name_focused);
        true
    }

    fn on_back_tab(&mut self) -> bool {
        let is_name_focused = self.name_editor.on_focus();
        self.description_editor.focus(is_name_focused);
        self.name_editor.focus(!is_name_focused);
        true
    }

    fn on_enter(&mut self) -> bool {
        if !self.name_editor.is_valid() || !self.description_editor.is_valid() {
            return false;
        }
        let result = if self.to_insert {
            self.controller.exercises().insert(&self.data()) 
        } else {
            self.controller.exercises().update(self.data()) 
        };
        if let Err(err) = result {
            if let Some(sender) = &mut self.ui_event_sender {
                let err = format!("Can`t insert exercise:\n{}", err);
                sender.send_add_layer_event(Box::new(UiMessageBox::err("Exercises", err)));
                return false;
            }
        } else {
            self.close();
        }

        true
    }

    fn on_esc(&mut self) -> bool {
        self.close();
        true
    }

    fn on_char(&mut self, c: &char) -> bool {
        if *c == 'q' {
            self.close();
        }
        true
    }
}
impl TermEventDispatcher for UiExerciseEditor { }

impl EventComponent for UiExerciseEditor {
    fn focus(&mut self, value: bool) {
        self.is_focused = value;
        if value {
            self.name_editor.focus(value);
            self.description_editor.focus(!value);
        }
    }

    fn on_focus(&self) -> bool { self.is_focused }

    fn on_term_event(&mut self, event: &Event) -> bool {
        if self.is_focused {
            if self.current_editor().on_term_event(event) {
                return true;
            }
            return self.dispatch_term_event(event);
        }
        false
    }
}

impl Layer for UiExerciseEditor {
    fn is_modal(&self) -> bool { true }

    fn is_remove_requested(&self) -> bool { self.is_removing_needed }
}

