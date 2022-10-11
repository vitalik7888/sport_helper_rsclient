use sport_core_db::model::Table;
use std::cell::RefCell;
use crate::{ui_events::{UiEventSender, SharedUiEvents, UiEvent}, exercise_editor::UiExerciseEditor};
use std::rc::Rc;

use core_ui::{
    components::{UiTable, Component, EventComponent},
    render::RenderFrame,
    event_dispatcher::{TermEventDispatcher, KeyEventDispatcher},
    message_box::UiMessageBox
};
use sport_core::controller;
use sport_core_db::entity;
use tui::{layout::{Rect, Constraint}, widgets, style::{Color, Style}};

pub struct UiExercisesTable {
    controller: Rc<controller::Controller>,
    ui_event_sender: UiEventSender,
    is_visible: bool,
    inner: RefCell<UiTable<entity::ID>>,
    commands_help: &'static str,
}

impl UiExercisesTable {
    pub fn new(controller: Rc<controller::Controller>, ui_events_manager: SharedUiEvents) -> Self { 
        Self { 
            ui_event_sender: UiEventSender::new(ui_events_manager),
            controller: controller.clone(),
            is_visible: true ,
            inner: RefCell::new(UiTable::default()),
            commands_help: "Add[a] Delete[d] Update[u]",
        }
    }

    pub fn commands_help(&self) -> &'static str {
        self.commands_help
    }
}

impl Component for UiExercisesTable {
    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visible(&mut self, value: bool) { self.is_visible = value; }

    fn draw(&self, f: &mut RenderFrame, area: Rect) {
        if !self.is_visible() {
            return;
        }
        self.inner.borrow_mut().clear_values();

        let mut ids = vec![];
        let mut rows: Vec<widgets::Row> = vec![];
        for (i, e) in self.controller.db().exercises().get_all().enumerate() {
            let color = if i % 2 == 0 {
                Color::DarkGray
            } else {
                Color::Gray
            };
            rows.push(widgets::Row::new(vec![
                                        widgets::Cell::from((i + 1).to_string()).style(Style::default().fg(Color::Green)),
                                        widgets::Cell::from(e.name.to_owned()).style(Style::default().fg(color)),
                                        widgets::Cell::from(e.description.to_owned()).style(Style::default().fg(color)),
            ]));
            ids.push(e.id);
        }
        self.inner.borrow_mut().set_values(ids);

        let theme = self.inner.borrow().theme().clone();
        let table = widgets::Table::new(rows)
            .style(theme.table_style)
            .header(
                widgets::Row::new(vec!["#", "Name", "Description"])
                .style(theme.header_style)
                .bottom_margin(1),
                )
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .title("Exercises"),
                )
            .widths(&[
                    Constraint::Length(3),
                    Constraint::Length(50),
                    Constraint::Length(30),
            ])
            .column_spacing(0)
            .highlight_style(theme.highlight_style)
            .highlight_symbol(">");

        f.render_stateful_widget(table, area, &mut self.inner.borrow_mut().state());
    }
}

impl TermEventDispatcher for UiExercisesTable {}

impl KeyEventDispatcher for UiExercisesTable {
    fn on_down(&mut self) -> bool {
        self.inner.borrow_mut().next();
        true
    }

    fn on_up(&mut self) -> bool {
        self.inner.borrow_mut().previous();
        true
    }

    fn on_char(&mut self, c: &char) -> bool {
        return match c {
            'r' => {
                if let Err(err) = self.controller.exercises().load_all() {
                    let err = format!("Can`t get all exercises:\n{}", err);
                    self.ui_event_sender.send_add_layer_event(Box::new(UiMessageBox::err("Exercises", err)));
                }
                false
            }
            'a' => {
                let mut popup = Box::new(UiExerciseEditor::new(self.controller.clone()));
                popup.set_ui_events_manager(self.ui_event_sender.ui_events_manager());
                self.ui_event_sender.send_add_layer_event(popup);
                return true;
            }
            'u' => {
                if let Some(id) = self.inner.borrow().get_value() {
                    if let Some(data) = self.controller.db().exercises().get_one(*id) {
                        let mut popup = Box::new(UiExerciseEditor::new(self.controller.clone()));
                        popup.set_ui_events_manager(self.ui_event_sender.ui_events_manager());
                        popup.load_data(data);
                        self.ui_event_sender.send_add_layer_event(popup);
                        return true;
                    }
                }
                false
            }
            'd' => {
                if let Some(id) = self.inner.borrow_mut().get_value() {
                    let id = *id;
                    if let Some(data) = self.controller.db().exercises().get_one(id) {
                        let ui_events_manager = self.ui_event_sender.ui_events_manager();
                        let controller = self.controller.clone();
                        let msg = format!("Are you sure you want to delete the entry `{}`", data.name);
                        let mut popup = Box::new(UiMessageBox::warn("Exercises", msg));
                        popup.set_on_accept(Box::new(move || {
                            if let Err(err) = controller.exercises().remove_exercise(id) {
                                let err = format!("Can`t remove exercise:\n{}", err);
                                let msgbox = Box::new(UiMessageBox::err("Exercises", err));
                                ui_events_manager.add_event(UiEvent::AddLayer(msgbox));
                            }
                        }));
                        self.ui_event_sender.send_add_layer_event(popup);
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}

impl EventComponent for UiExercisesTable {
    fn focus(&mut self, value: bool) {
        self.inner.borrow_mut().focus(value);
    }

    fn on_focus(&self) -> bool {
        self.inner.borrow_mut().on_focus()
    }
}

