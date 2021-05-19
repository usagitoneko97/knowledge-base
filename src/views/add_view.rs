use crate::data::Knowledge;
use crate::views::app::App;
use crossterm::event::{KeyCode, KeyEvent};
use crate::key::Key;

pub fn handler(app: &mut App, event: &Key) {
    match event {
        Key::Char(c) => {
            app.get_current_input().insert(c.clone());
        }
        Key::Tab => {
            app.input_current_tab.next();
        }
        Key::BackTab => {
            app.input_current_tab.prev();
        }
        Key::Backspace => {
            app.get_current_input().backspace();
        }
        Key::Left => {
            app.get_current_input().move_left();
        }
        Key::Right => {
            app.get_current_input().move_right();
        }
        Key::Enter => {
            app.get_current_input().new_line();
        }
        Key::Ctrl('g') => {
            let knowledge = Knowledge::new(
                app.input_title.get_string(),
                app.input_text.get_string(),
                String::new(),
                app.input_tags.get_string(),
            );
            match knowledge.write_to_file(app.base_path.clone(), "md") {
                std::io::Result::Ok(()) => {}
                std::io::Result::Err(_e) => {
                    panic!("Error in writing file!");
                }
            }
            app.pop_state();
            app.refresh_directory();
        }
        _ => {}
    }
}
