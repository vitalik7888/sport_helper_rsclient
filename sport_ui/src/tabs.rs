use core_ui::{
    render::RenderFrame,
    components::{EventComponent, Component},
    event_dispatcher::{TermEventDispatcher, KeyEventDispatcher}
};
use log::warn;
use tui::{layout::Rect, text::Spans, widgets, style::{Style, Color, Modifier}};

use crate::ui_events::UiEventSender;

pub struct UiTab {
    name: String,
    is_focused: bool,
}

impl UiTab {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_focused: false,
        }
    }

    pub fn focus(&mut self, value: bool) {
        self.is_focused = value;
    }

    pub fn on_focus(&self) -> bool {
        self.is_focused
    }
}

pub struct UiTabs {
    tabs: Vec<UiTab>,
    current_index: usize,
    is_focused: bool,
    ui_event_sender: Option<UiEventSender>,
}

impl UiTabs {
    pub fn new(ui_tabs: Vec<UiTab>) -> Self {
        let mut s = Self {
            ui_event_sender: None,
            tabs: ui_tabs,
            current_index: 1,
            is_focused: false,
        };
        s.set_current_tab(0);
        s
    }

    pub fn tab(&self, index: usize) -> Option<&UiTab> {
        self.tabs.get(index)
    }

    pub fn current_tab(&self) -> &UiTab {
        &self.tabs[self.current_index as usize]
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn set_current_tab(&mut self, index: usize) {
        if index >= self.tabs.len() {
            warn!("Can`t set {} as current tab", index);
        } else {
            if self.current_index != index {
                self.current_index = index;
                if let Some(sender) = &mut self.ui_event_sender {
                    sender.send_menu_switched_event(index);
                }
                self.tabs
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, t)| t.focus(i == index));
            }
        }
    }

    pub fn tabs_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn next(&mut self) -> usize {
        let current_index = self.current_index();
        if current_index < self.tabs_count() {
            self.set_current_tab(current_index + 1);
        }
        self.current_index
    }

    pub fn prev(&mut self) -> usize {
        let current_index = self.current_index();
        if current_index > 0 {
            self.set_current_tab(current_index - 1);
        }
        self.current_index
    }
}

impl Component for UiTabs {
    fn draw(&self, f: &mut RenderFrame, area: Rect) {
        let spans = self
            .tabs
            .iter()
            .enumerate()
            .map(|(i, t)| Spans::from(format!("{}[{}]", t.name, i + 1)))
            .collect();
        let tabs = widgets::Tabs::new(spans)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .title("Menu"),
                )
            .select(self.current_index() as usize)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
                );
        f.render_widget(tabs, area);
    }
}

impl TermEventDispatcher for UiTabs {}
impl KeyEventDispatcher for UiTabs { }

impl EventComponent for UiTabs {
    fn focus(&mut self, value: bool) {
        self.is_focused = value;
    }

    fn on_focus(&self) -> bool {
        self.is_focused
    }
}

