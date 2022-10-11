use tui::style;


#[derive(Clone, Copy)]
pub struct UiTheme {
    pub table: UiTableTheme,
    pub text_edit: UiTextEditTheme,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            table: UiTableTheme::default(),
            text_edit: UiTextEditTheme::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct UiTextEditTheme { //TODO style on focus, error
    pub text_style: style::Style,
}

impl Default for UiTextEditTheme {
    fn default() -> Self {
        Self {
            text_style: style::Style::default().fg(style::Color::White).bg(style::Color::Black),
        }
    }
}

#[derive(Clone, Copy)]
pub struct UiTableTheme {
    pub table_style: style::Style,
    pub header_style: style::Style,
    pub highlight_style: style::Style,
}

impl Default for UiTableTheme {
    fn default() -> Self {
        Self {
            table_style: style::Style::default().fg(style::Color::White),
            header_style: style::Style::default().fg(style::Color::Red),
            highlight_style: style::Style::default().add_modifier(style::Modifier::REVERSED),
        }
    }
}
