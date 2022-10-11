use crate::component::TerminalEvent;

pub type KeyEvent = crossterm::event::KeyCode;

pub trait KeyEventDispatcher {
    fn dispatch_key_event(&mut self, key_code: &KeyEvent) -> bool {
        match key_code {
            KeyEvent::Backspace => self.on_backspace(),
            KeyEvent::Enter => self.on_enter(),
            KeyEvent::Tab => self.on_tab(),
            KeyEvent::BackTab => self.on_back_tab(),
            KeyEvent::Char(c) => self.on_char(c),
            KeyEvent::Esc => self.on_esc(),
            KeyEvent::Left => self.on_left(),
            KeyEvent::Right => self.on_right(),
            KeyEvent::Up => self.on_up(),
            KeyEvent::Down => self.on_down(),
            KeyEvent::Home => self.on_home(),
            KeyEvent::End => self.on_end(),
            KeyEvent::PageUp => self.on_page_up(),
            KeyEvent::PageDown => self.on_page_down(),
            KeyEvent::Delete => self.on_delete(),
            KeyEvent::Insert => self.on_insert(),
            KeyEvent::Null => self.on_null(),
            KeyEvent::CapsLock => self.on_caps_lock(),
            KeyEvent::ScrollLock => self.on_scroll_lock(),
            KeyEvent::NumLock => self.on_num_lock(),
            KeyEvent::PrintScreen => self.on_print_screen(),
            KeyEvent::Pause => self.on_pause(),
            KeyEvent::Menu => self.on_menu(),
            KeyEvent::KeypadBegin => self.on_keypad_begin(),
            KeyEvent::F(_) => false,
            KeyEvent::Media(_) => false,
            KeyEvent::Modifier(_) => false,
        }
    }

    fn on_keypad_begin(&mut self) -> bool { false }
    fn on_menu(&mut self) -> bool { false }
    fn on_pause(&mut self) -> bool { false }
    fn on_print_screen(&mut self) -> bool { false }
    fn on_num_lock(&mut self) -> bool { false }
    fn on_scroll_lock(&mut self) -> bool { false }
    fn on_caps_lock(&mut self) -> bool { false }
    fn on_null(&mut self) -> bool { false }
    fn on_insert(&mut self) -> bool { false }
    fn on_delete(&mut self) -> bool { false }
    fn on_page_down(&mut self) -> bool { false }
    fn on_page_up(&mut self) -> bool { false }
    fn on_end(&mut self) -> bool { false }
    fn on_home(&mut self) -> bool { false }
    fn on_down(&mut self) -> bool { false }
    fn on_up(&mut self) -> bool { false }
    fn on_right(&mut self) -> bool { false }
    fn on_left(&mut self) -> bool { false }
    fn on_esc(&mut self) -> bool { false }
    fn on_backspace(&mut self) -> bool { false }
    fn on_enter(&mut self) -> bool { false }
    fn on_tab(&mut self) -> bool { false }
    fn on_back_tab(&mut self) -> bool { false }
    fn on_char(&mut self, _c: &char) -> bool { false }
}

pub trait TermEventDispatcher: KeyEventDispatcher {
    fn dispatch_term_event(&mut self, event: &TerminalEvent) -> bool {
        return match event {
            TerminalEvent::FocusGained => todo!(),
            TerminalEvent::FocusLost => todo!(),
            TerminalEvent::Key(key_event) => self.dispatch_key_event(&key_event.code),
            TerminalEvent::Mouse(_) => todo!(),
            TerminalEvent::Paste(s) => self.on_paste(s),
            TerminalEvent::Resize(_, _) => todo!(),
        }
    }

    fn on_paste(&mut self, _s: &str) -> bool { false }
}
