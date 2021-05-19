use crate::data::Knowledge;
use crate::key::{CtrlKey, Key};
use crate::views::app::{App, ViewState};

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
        Key::Up => {
            app.get_current_input().move_up();
        }
        Key::Down => {
            app.get_current_input().move_down();
        }
        Key::Enter => {
            app.get_current_input().new_line();
        }
        Key::Home => {
            app.get_current_input().beginning_of_line();
        }
        Key::End => {
            app.get_current_input().end_of_line();
        }
        Key::Delete => {
            app.get_current_input().delete();
        }
        Key::Ctrl(CtrlKey::Home) => {
            app.get_current_input().top_beginning_of_line();
        }
        Key::Ctrl(CtrlKey::End) => {
            app.get_current_input().bottom_end_of_line();
        }
        Key::Ctrl(CtrlKey::Char('w')) => {
            app.get_current_input().backspace_word();
        }
        Key::Ctrl(CtrlKey::Left) => {
            app.get_current_input().move_left_word();
        }
        Key::Ctrl(CtrlKey::Right) => {
            app.get_current_input().move_right_word();
        }
        Key::Ctrl(CtrlKey::Delete) => {
            app.get_current_input().delete_word();
        }
        Key::Ctrl(CtrlKey::Char('g')) => {
            fn action(app: &mut App) {
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
            }
            app.push_state(ViewState::DialogView);
            app.confirm_action = Some(action);
            app.confirm_text = format!("Confirm writing file: {}?", app.input_title.get_string());
            // default it to True so we don't need to use arrow key
            app.confirm = true;
        }
        Key::Esc => {
            fn action(app: &mut App) {
                app.set_add_view_ref();
                app.pop_state();
            }
            app.push_state(ViewState::DialogView);
            app.confirm_action = Some(action);
            app.confirm_text = format!("Confirm Quiting?");
            app.confirm = false;
        }
        _ => {}
    }
}
