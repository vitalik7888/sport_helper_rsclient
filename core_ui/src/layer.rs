use crossterm::event::Event;
use tui::layout;
use std::collections::VecDeque;

use crate::{
    component::{Component, EventComponent},
    theme::UiTheme,
    render::RenderFrame,
    event_dispatcher::{KeyEventDispatcher, TermEventDispatcher}
};

pub trait Layer: Component + EventComponent {
    fn is_modal(&self) -> bool { false }
    fn set_modal(&mut self, _is_modal: bool) {}

    fn is_remove_requested(&self) -> bool { false }
}

pub trait LayerItemComponent: Component + EventComponent {}
#[allow(unused_variables)]
pub struct UiLayer
{
    components: Vec<Box<dyn LayerItemComponent>>,
    is_focused: bool,
    is_visible: bool,
    is_modal: bool,
}

impl UiLayer
{
    pub fn new() -> Self {
        Self::with_components(vec![])
    }

    pub fn with_components(components: Vec<Box<dyn LayerItemComponent>>) -> Self {
        Self {
            components,
            is_focused: false,
            is_visible: true,
            is_modal: false,
        }
    }

    pub fn add(&mut self, component: Box<dyn LayerItemComponent>) {
        self.components.push(component);
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }
}

impl Layer for UiLayer 
{
    fn is_modal(&self) -> bool { self.is_modal }

    fn set_modal(&mut self, is_modal: bool) {
        self.is_modal = is_modal;
    }
}

impl TermEventDispatcher for UiLayer {}
impl KeyEventDispatcher for UiLayer {}

impl EventComponent for UiLayer 
{
    fn focus(&mut self, value: bool) {
        self.is_focused = value;
        self.components
            .iter_mut()
            .for_each(|component| component.focus(value));
    }

    fn on_focus(&self) -> bool {
        self.is_focused
    }

    fn on_term_event(&mut self, event: &Event) -> bool {
        let mut is_consumed = false;
        for component in self.components.iter_mut() {
            is_consumed = is_consumed || component.on_term_event(event);
        }
        is_consumed
    }
}

impl Component for UiLayer 
{
    fn apply_theme(&mut self, theme: &UiTheme) {
        self.components
            .iter_mut()
            .for_each(|component| component.apply_theme(theme));
    }

    fn draw(&self, f: &mut RenderFrame, area: layout::Rect) {
        self.components
            .iter()
            .for_each(|components| components.draw(f, area));
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visible(&mut self, value: bool) {
        self.is_visible = value;
        self.components
            .iter_mut()
            .for_each(|components| components.set_visible(value));
    }
}

pub struct UiLayers
{
    layers: VecDeque<Box<dyn Layer>>,
}

impl UiLayers 
{
    pub fn new() -> Self {
        Self { layers: VecDeque::new() }
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn push(&mut self, layer: Box<dyn Layer>) {
        self.layers.push_back(layer);
    }

    pub fn pop(&mut self) {
        self.layers.pop_back();
    }

    pub fn top<'a>(&'a mut self) -> Option<&'a mut Box<dyn Layer>> {
        let index = self.layers.len() - 1;
        self.layers.get_mut(index)
    }

    pub fn get(&self, index: usize) -> Option<&Box<dyn Layer>> {
        self.layers.get(index)
    }

    pub fn get_all<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Box<dyn Layer>> + 'a> {
        Box::new(self.layers.iter())
    }

    pub fn get_all_mut<'a>(&'a mut self) -> Box<dyn DoubleEndedIterator<Item = &'a mut Box<dyn Layer>> + 'a> {
        Box::new(self.layers.iter_mut())
    }

    pub fn clean(&mut self) {
        self.layers.retain_mut(|l| !l.is_remove_requested());
    }
}

