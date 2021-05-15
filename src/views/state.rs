use crate::config::Config;
use crate::file_view;
use crossterm::event::{KeyCode, KeyEvent};
use std::iter::Cycle;
use std::ops::Range;
use tui::widgets::ListState;
use std::path::{PathBuf, Path};


pub enum ViewState {
    FileView(file_view::FileState),
    AddView,
    TagView,
}

impl ViewState {
    fn update_with_keycode(&mut self, event: &KeyEvent) {
        match self {
            ViewState::FileView(file_state) => match event.code {
                KeyCode::Down | KeyCode::Char('j') => {}
                _ => {}
            },
            _ => {}
        }
    }
}

pub struct ProgramState<'a> {
    pub view_state: ViewState,
    pub list_state: &'a mut ListState,
}

impl<'a> ProgramState<'a> {
    pub fn new(view_state: ViewState, list_state: &'a mut ListState) -> Self {
        ProgramState {
            view_state, list_state
        }
    }

    pub fn update_state(&mut self, event: &KeyEvent) {
        match self.view_state {
            ViewState::FileView(ref mut file_view) => {
                file_view.handler(event);
            }
            ViewState::TagView => {

            }
            ViewState::AddView => {

            }
        }
    }
}
