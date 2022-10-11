use std::{io, cell::RefCell};

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen}, execute, event::DisableMouseCapture};
use tui::{backend::CrosstermBackend, Terminal};

pub type RenderBackend = CrosstermBackend<io::Stdout>;
pub type RenderFrame<'a> = tui::Frame<'a, RenderBackend>;
pub type RenderError = crossterm::ErrorKind;

pub struct Render
{
    term: RefCell<Terminal<RenderBackend>>,
}

impl Default for Render {
    fn default() -> Self {
        Self::new().expect("Can`t create default render")
    }
}

impl Render
{
    pub fn new() -> Result<Self, RenderError> {
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(io::stdout());
        let term = Terminal::new(backend)?;
        Ok(Self { term: RefCell::new(term) })
    }

    pub fn clear(&self) -> Result<(), RenderError> {
        self.term.borrow_mut().clear()?;
        Ok(())
    }

    pub fn draw<'a>(&self, f: Box<dyn FnOnce(&mut RenderFrame) + 'a>) -> Result<(), RenderError> {
        self.term.borrow_mut().draw(f)?;
        Ok(())
    }

}

impl Drop for Render {
    fn drop(&mut self) {
        disable_raw_mode().expect("Can`t disable raw mode");
        {
        let mut term = self.term.borrow_mut();
        execute!(
            term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
            )
            .expect("Can`t execute!");
        }
       self.term.borrow_mut().show_cursor().expect("Can`t show cursor");
    }
}


