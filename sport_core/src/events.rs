use std::cell::RefCell;
use std::{time::Duration, collections::VecDeque};

use crossterm;
use thiserror::Error;
use log::error;

pub struct Events<T> {
    events: RefCell<VecDeque<T>>,
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self { events: RefCell::new(VecDeque::new()) }
    }
}

impl<T> Events<T> {

    pub fn add_event(&self, event: T) {
        self.events.borrow_mut().push_back(event);
    }

    pub fn get_event(&self) -> Option<T> {
        self.events.borrow_mut().pop_front()
    }

    pub fn clear(&self) {
        self.events.borrow_mut().clear();
    }

    pub fn get_all(&self) -> VecDeque<T> {
        let mut data = VecDeque::with_capacity(self.events.borrow().len());
        while !self.events.borrow().is_empty() {
            if let Some(e) = self.events.borrow_mut().pop_front() {
                data.push_back(e);
            }
        }
        data
    }
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Read event error: `{0}`")]
    Parse(#[from] crossterm::ErrorKind),
    #[error("Can`t get event from events poll")]
    NoEvent,
}

pub fn read_system_event(timeout: Duration) -> Result<crossterm::event::Event, EventError>{
    if crossterm::event::poll(timeout)? {
        return Ok(crossterm::event::read()?);
    }
    Err(EventError::NoEvent)
}
