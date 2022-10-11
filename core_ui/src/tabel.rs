use tui::widgets;

use crate::{
    theme::{UiTableTheme, UiTheme},
    component::{Component, EventComponent},
    render::RenderFrame,
    event_dispatcher::{KeyEventDispatcher, TermEventDispatcher}
};

pub struct UiTable<V = u64> {
    theme: UiTableTheme,
    state: widgets::TableState,
    values: Vec<V>,
    is_focused: bool,
}

impl<V> Default for UiTable<V> {
    fn default() -> Self {
        Self {
            theme: UiTableTheme::default(),
            state: widgets::TableState::default(),
            values: vec![],
            is_focused: false,
        }
    }
}

impl<V> UiTable<V> {
    pub fn theme(&self) -> &UiTableTheme {
        &self.theme
    }

    pub fn state(&mut self) -> &mut widgets::TableState {
        &mut self.state
    }

    pub fn selected_row(&self) -> Option<usize> {
        if !self.on_focus() {
            return None;
        }
        self.state.selected()
    }

    pub fn select_row(&mut self, index: usize) {
        if index < self.count() {
            self.state.select(Some(index));
        }
    }

    pub fn next(&mut self) {
        if self.count() > 0 && self.on_focus() {
            let i = match self.selected_row() {
                Some(i) => {
                    if i >= self.count() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.select_row(i);
        }
    }

    pub fn previous(&mut self) {
        if self.count() > 0 && self.on_focus() {
            let i = match self.selected_row() {
                Some(i) => {
                    if i == 0 {
                        self.count() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.select_row(i);
        }
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn get_value(&self) -> Option<&V> {
        if let Some(row) = self.selected_row() {
            Some(&self.values[row])
        } else {
            None
        }
    }

    pub fn set_values(&mut self, values: Vec<V>) {
        self.values = values;
    }

    pub fn clear_values(&mut self) {
        self.values.clear();
    }
}

impl<V> EventComponent for UiTable<V> {
    fn focus(&mut self, value: bool) {
        self.is_focused = value;
    }

    fn on_focus(&self) -> bool {
        self.is_focused
    }
}

impl<V> Component for UiTable<V> {
    fn apply_theme(&mut self, theme: &UiTheme) {
        self.theme = theme.table.clone();
    }

    fn draw(&self, _f: &mut RenderFrame, _area: tui::layout::Rect) {}
}

impl<V> TermEventDispatcher for UiTable<V> {}

impl<V> KeyEventDispatcher for UiTable<V> {
    fn on_down(&mut self) -> bool {
        self.next();
        true
    }

    fn on_up(&mut self) -> bool {
        self.previous();
        true

    }
}


