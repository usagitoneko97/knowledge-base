use crate::data::Knowledge;
use crate::views::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handler(app: &mut App, event: &KeyEvent) {
    match event.code {
        KeyCode::Char(c) => {
            app.get_current_input().insert(c);
        }
        KeyCode::Tab => {
            app.input_current_tab.next();
        }
        KeyCode::Backspace => {
            app.get_current_input().backspace();
        }
        KeyCode::Left => {
            app.get_current_input().move_left();
        }
        KeyCode::Right => {
            app.get_current_input().move_right();
        }
        KeyCode::Enter => {
            let knowledge = Knowledge::new(
                app.input_title.get_string(),
                app.input_text.get_string(),
                String::new(),
                app.input_tags.get_string(),
            );
            match knowledge.write_to_file(app.base_path.clone(), "md") {
                std::io::Result::Ok(()) => {}
                std::io::Result::Err(_e) => {panic!("Error in writing file!");}
            }
            app.pop_state();
            app.refresh_directory();
        }
        _ => {}
    }
}
