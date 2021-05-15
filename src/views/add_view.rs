use crate::state;
use crossterm::event::{
    KeyCode, KeyEvent
};


pub struct AddState {
    pub title: String,
    pub text: String,
    pub file_name: String,
}

pub fn handler(program_state: &mut state::ProgramState, event: &KeyEvent) {
    match event.code {
        KeyCode::Down => {
        }
        _ => {}
    }
}