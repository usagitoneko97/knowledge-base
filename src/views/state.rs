use crate::add_view;
use crate::config::Config;
use crate::file_view;
use crate::util::BiCycle;
use crossterm::event::{KeyCode, KeyEvent};
use std::error::Error;
use std::iter::Cycle;
use std::ops::Range;
use std::path::{Path, PathBuf};
use tui::widgets::ListState;

pub enum ViewState {
    FileView,
    AddView,
    TagView,
}

pub enum Tab {
    Title,
    Tags,
    Text,
}

#[derive(Default)]
pub struct Input {
    input: Vec<char>,
    input_idx: usize,
}

impl Input {
    pub fn insert(&mut self, c: char) {
        self.input.insert(self.input_idx, c);
        self.input_idx = self.input_idx + 1;
    }

    pub fn get_string(&self) -> String {
        self.input.iter().collect()
    }

    pub fn backspace(&mut self) {
        self.input_idx -= 1;
        self.input.remove(self.input_idx);
    }

    pub fn move_left(&mut self) {
        if self.input_idx > 0 {
            self.input_idx -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.input_idx < self.input.len() {
            self.input_idx += 1;
        }
    }
}

pub enum FileMode {
    Dir,
    File,
}

pub struct App {
    navigation_stack: Vec<ViewState>,

    pub input_title: Input,
    pub input_text: Input,
    pub input_tags: Input,
    pub input_tabs: Vec<Tab>,
    pub input_current_tab: BiCycle,

    pub file_hierarchy: String,
    pub files: Vec<String>,
    pub base_path: PathBuf,
    pub file_cycle: BiCycle,
    pub file_mode: FileMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_mode: FileMode::Dir,
            navigation_stack: vec![ViewState::FileView],

            input_title: Input::default(),
            input_text: Input::default(),
            input_tags: Input::default(),
            input_tabs: vec![],
            input_current_tab: BiCycle::default(),
            file_hierarchy: String::default(),
            files: vec![],
            base_path: PathBuf::default(),
            file_cycle: BiCycle::default(),
        }
    }
}

impl App {
    pub fn set_file_view(mut self, config: &Config) -> Self {
        let file_directory = config.data_directories.last().unwrap();
        match App::get_file_list(file_directory) {
            Ok(item) => {
                let item_len = item.len();
                self.files = item;
                self.file_cycle = BiCycle::new(item_len);
                self.base_path = PathBuf::from(file_directory.clone());
                self.file_mode = FileMode::Dir
            }
            Err(_) => {
                panic!(
                    "{}",
                    format!(
                        "Data directories specified in config: {} is not a directory!",
                        file_directory
                    )
                );
            }
        }
        self
    }

    pub fn set_add_view(mut self) -> Self {
        self.set_add_view_ref();
        self
    }

    pub fn set_add_view_ref(&mut self) -> &mut Self {
        let s = vec![Tab::Title, Tab::Tags, Tab::Text];
        let len = s.len();
        self.input_title = Input::default();
        self.input_text = Input::default();
        self.input_tags = Input::default();
        self.input_tabs = s;
        self.input_current_tab = BiCycle::new(len);
        self
    }

    pub fn update_state(&mut self, event: &KeyEvent) {
        // return err if it reaches last state in the stack
        if let Some(state) = self.get_latest_mut_state() {
            match state {
                ViewState::FileView => {
                    file_view::handler(self, event);
                }
                ViewState::AddView => {
                    add_view::handler(self, event);
                }
                ViewState::TagView => {}
            }
        } else {
        }
    }

    pub fn push_state(&mut self, view_state: ViewState) {
        self.navigation_stack.push(view_state);
    }

    pub fn pop_state(&mut self) {
        self.navigation_stack.pop();
    }

    pub fn get_latest_mut_state(&mut self) -> Option<&mut ViewState> {
        self.navigation_stack.last_mut()
    }

    pub fn get_latest_state(&mut self) -> Option<&ViewState> {
        self.navigation_stack.last()
    }

    fn get_file_list<T: AsRef<Path>>(path: T) -> std::io::Result<Vec<String>> {
        let item: Vec<_> = std::fs::read_dir(path)?
            .filter_map(|e| {
                let file_type = e.as_ref().unwrap().file_type().ok()?;
                if file_type.is_file() {
                    if e.as_ref().unwrap().path().extension()?.clone() == "md" {
                        Some(e.unwrap().file_name().into_string().unwrap())
                    } else {
                        None
                    }
                } else if file_type.is_dir() {
                    Some(e.unwrap().file_name().into_string().unwrap())
                } else {
                    None
                }
            })
            .collect();
        Ok(item)
    }
    pub fn refresh_directory(&mut self) {
        match App::get_file_list(&self.base_path) {
            Ok(files) => {
                let item_len = files.len();
                self.files = files;
                self.file_cycle = BiCycle::new(item_len);
            }
            Err(e) => {
                self.files.clear();
                self.file_mode = FileMode::File;
            }
        }
    }

    pub fn enter_directory(&mut self) {
        // enter directory specify by `self.cycle.current_item`
        let selected_file = self.files.get(self.file_cycle.current_item).unwrap();
        self.base_path.push(selected_file);
        self.refresh_directory();
    }

    pub fn leave_directory(&mut self) {
        self.base_path.pop();
        match App::get_file_list(&self.base_path) {
            Ok(files) => {
                let item_len = files.len();
                self.files = files;
                self.file_cycle = BiCycle::new(item_len);
                self.file_mode = FileMode::Dir;
            }
            Err(e) => {
                panic!(
                    "{}",
                    format!(
                        "Getting file list from dir: {:?} failed with {:?}",
                        self.base_path, e
                    )
                )
            }
        }
    }

    pub fn get_current_input(&mut self) -> &mut Input {
        if let Some(s) = self.input_tabs.get(self.input_current_tab.current_item) {
            match s {
                Tab::Title => &mut self.input_title,
                Tab::Text => &mut self.input_text,
                Tab::Tags => &mut self.input_tags,
            }
        } else {
            panic!("invalid tab selected!");
        }
    }
}
