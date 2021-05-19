use crate::views::app::{App, ViewState};
use crossterm::event::{KeyCode, KeyEvent};
use crate::key::Key;

pub fn handler(app: &mut App, event: &Key) {
    match event {
        Key::Down | Key::Char('j') => {
            app.file_cycle.next();
        }
        Key::Up | Key::Char('k') => {
            app.file_cycle.prev();
        }
        Key::Enter | Key::Char('l') => {
            app.enter_directory();
        }
        Key::Char('h') => {
            app.leave_directory();
        }
        Key::Char('a') => {
            app.set_add_view_ref();
            app.push_state(ViewState::AddView);
        }
        _ => {}
    }
}

pub struct BiCycle {
    pub total_len: usize,
    pub current_item: usize,
}

impl BiCycle {
    pub fn new(total_len: usize) -> Self {
        BiCycle {
            total_len,
            current_item: 0,
        }
    }

    pub fn next(&mut self) -> Option<usize> {
        self.current_item = if self.current_item >= self.total_len - 1 {
            0
        } else {
            self.current_item + 1
        };
        Some(self.current_item)
    }

    pub fn prev(&mut self) -> Option<usize> {
        self.current_item = if self.current_item == 0 {
            self.total_len - 1
        } else {
            self.current_item - 1
        };
        Some(self.current_item)
    }
}
