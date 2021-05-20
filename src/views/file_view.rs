use crate::data::Knowledge;
use crate::key::Key;
use crate::views::app::{App, FileStatus, ViewState};

pub fn handler(app: &mut App, event: &Key) {
    match event {
        Key::Down | Key::Char('j') => {
            app.file_cycle_stack.last_mut().unwrap().next();
        }
        Key::Up | Key::Char('k') => {
            app.file_cycle_stack.last_mut().unwrap().prev();
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
        Key::Char('D') => {
            fn action(app: &mut App) {
                app.remove_directory();
            }
            let entry = app.get_current_selected_entry();
            app.push_state(ViewState::DialogView);
            app.confirm_action = Some(action);
            app.confirm_text = format!(
                "confirm deleting: {}?",
                entry
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Invalid_file_name")
            );
            // default it to True so we don't need to use arrow key
            app.confirm = false;
            app.previous_view = ViewState::FileView;
        }
        Key::Char('e') => {
            app.push_state(ViewState::AddView);
            let entry = app.get_current_selected_entry();
            if entry.is_file() {
                app.set_add_view_ref();
                let knowledge = Knowledge::from_file(entry.clone());
                app.input_title.insert_string(&knowledge.title);
                app.input_text.insert_string(&knowledge.text);
                app.file_status = FileStatus::Edit(entry.clone());
                // put focus to text
            } else {
                app.enter_directory();
            }
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
