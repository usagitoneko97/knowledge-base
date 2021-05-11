use crate::config::Config;
use crossterm::event::{KeyCode, KeyEvent};
use std::iter::Cycle;
use std::ops::Range;
use tui::widgets::ListState;
use std::path::{PathBuf, Path};

pub struct FileState {
    pub file_hierarchy: String,
    pub files: Vec<String>,
    pub base_path: PathBuf,
    pub cycle: BiCycle,
}

impl FileState {
    fn get_file_list<T: AsRef<Path>>(path: T) -> Vec<String> {
        let item: Vec<_> = std::fs::read_dir(path)
            .unwrap()
            .filter_map(|e| {
                if let Ok(file_type) = e.as_ref().unwrap().file_type() {
                    if file_type.is_file() {
                        if e.as_ref().unwrap().path().extension().unwrap().clone() == "md" {
                            Some(e.unwrap().file_name().into_string().unwrap())
                        } else {
                            None
                        }
                    } else if file_type.is_dir() {
                        Some(e.unwrap().file_name().into_string().unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        item
    }

    pub fn new(config: &Config) -> Self {
        let file_directory = config.data_directories.last().unwrap();
        let item = FileState::get_file_list(file_directory);
        let item_len = item.len();
        FileState {
            file_hierarchy: String::from(""),
            files: item,
            cycle: BiCycle::new(item_len),
            base_path: PathBuf::from(file_directory.clone()),
        }
    }

    pub fn enter_directory(&mut self) {
        // enter directory specify by `self.cycle.current_item`
        let selected_file = self
            .files
            .get(self.cycle.current_item)
            .unwrap();
        self.base_path.push(selected_file);
        let files = FileState::get_file_list(&self.base_path);
        let item_len = files.len();
        self.files = files;
        self.cycle = BiCycle::new(item_len);
    }

    pub fn leave_directory(&mut self) {
        self.base_path.pop();
        let files = FileState::get_file_list(&self.base_path);
        let item_len = files.len();
        self.files = files;
        self.cycle = BiCycle::new(item_len);
    }
}

pub struct BiCycle {
    pub total_len: usize,
    pub current_item: usize,
}

impl BiCycle {
    fn new(total_len: usize) -> Self {
        BiCycle {
            total_len,
            current_item: 0,
        }
    }

    fn next(&mut self) -> Option<usize> {
        self.current_item = if self.current_item >= self.total_len - 1 {
            0
        } else {
            self.current_item + 1
        };
        Some(self.current_item)
    }

    fn prev(&mut self) -> Option<usize> {
        self.current_item = if self.current_item == 0 {
            self.total_len - 1
        } else {
            self.current_item - 1
        };
        Some(self.current_item)
    }
}

pub enum ViewState {
    FileView(FileState),
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
        match event.code {
            KeyCode::Down | KeyCode::Char('j') => match self.view_state {
                ViewState::FileView(ref mut file_view) => {
                    self.list_state.select(file_view.cycle.next());
                }
                ViewState::TagView => {}
            },
            KeyCode::Up | KeyCode::Char('k') => match self.view_state {
                ViewState::FileView(ref mut file_view) => {
                    self.list_state.select(file_view.cycle.prev());
                }
                ViewState::TagView => {}
            }
            KeyCode::Enter | KeyCode::Char('l') => match self.view_state {
                ViewState::FileView(ref mut file_view) => {
                    file_view.enter_directory();
                    self.list_state.select(Some(0));
                }
                ViewState::TagView => {}
            }
            KeyCode::Char('h') => match self.view_state {
                ViewState::FileView(ref mut file_view) => {
                    file_view.leave_directory();
                    self.list_state.select(Some(0));
                }
                ViewState::TagView => {}
            }
            _ => {}
        }
    }
}
