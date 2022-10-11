use tui::{layout, text::{Spans, Span}, widgets, style::{Style, Color}};

use crate::{
    render::RenderFrame,
    theme::{UiTextEditTheme, UiTheme},
    event_dispatcher::{KeyEventDispatcher, TermEventDispatcher},
    component::{Component, EventComponent}, validators::{Validator, EmptyStrValidator}, 
};

pub struct TextEdit<V: Validator<str> = EmptyStrValidator>
{
    pub title: String, // TODO Cow?
    pub text: String,
    theme: UiTextEditTheme,
    is_focused: bool,
    pub is_edit_enabled: bool,
    validator: V,
}

impl<V: Validator<str> + Default> Default for TextEdit<V> {
    fn default() -> Self {
        Self::new("", "".to_owned(), V::default())
    }
}

impl<V: Validator<str>> TextEdit<V> {
    pub fn new(title: &str, text: String, validator: V) -> Self {
        Self { 
            title: title.to_owned(),
            text,
            theme: UiTextEditTheme::default(),
            is_focused: false,
            is_edit_enabled: true,
            validator,
        }

    }

    pub fn is_valid(&self) -> bool {
        if let Ok(_) = self.validator.validate(&self.text) {
            true
        } else {
            false
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validator.validate(&self.text).map_err(|e| e.to_string())
    }
}

impl<V: Validator<str>> TermEventDispatcher for TextEdit<V> {
    fn on_paste(&mut self, s: &str) -> bool {
        self.text = s.to_string(); 
        true  
    }
}

impl<V: Validator<str>> KeyEventDispatcher for TextEdit<V> {
   fn on_backspace(&mut self) -> bool {
    self.text.pop(); 
    true
   }

   fn on_char(&mut self, c: &char) -> bool {
       self.text.push(*c);
       true
   }

   fn on_esc(&mut self) -> bool {
       self.is_focused = false;
       true
   }

   fn on_enter(&mut self) -> bool {
       self.focus(false);
       true
   }
}

impl<V: Validator<str>> Component for TextEdit<V> {
    fn draw(&self, f: &mut RenderFrame, area: layout::Rect) {
        let title: String;
        let title_color: Color;
        if let Err(err) = self.validate() {
            title = format!("{}   Validation error: {}", self.title, err);
            title_color = Color::Red;
        } else {
            title_color = Color::Black;
            title = self.title.clone();
        }
        let text_style = Spans::from(Span::styled(
                self.text.clone(),
                self.theme.text_style,
                ));
        let p = widgets::Paragraph::new(text_style)
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().bg(Color::White).fg(
                        if self.on_focus() {Color::Black} else { Color::Gray }
                    ))
                .title(tui::text::Span::styled(
                        title,
                        // title.push_str(" -> Commands(Done: [ESC], [ENTER])"),
                        Style::default().fg(title_color).add_modifier(tui::style::Modifier::BOLD),
                        )))
            .alignment(tui::layout::Alignment::Left);
        f.render_widget(p, area);
    }

    fn apply_theme(&mut self, theme: &UiTheme) { self.theme = theme.text_edit; }
}

impl<V: Validator<str>> EventComponent for TextEdit<V> {
    fn focus(&mut self, value: bool) { self.is_focused = value; }

    fn on_focus(&self) -> bool { self.is_focused }
}


