use crate::state;
use crossterm::event::{
    KeyCode, KeyEvent
};
use std::path::{PathBuf, Path};
use crate::config::Config;


pub enum FileMode {
    Dir,
    File
}

pub struct FileState {
    pub file_hierarchy: String,
    pub files: Vec<String>,
    pub base_path: PathBuf,
    pub cycle: BiCycle,
    pub mode: FileMode,
}

impl FileState {
    pub fn handler(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Down | KeyCode::Char('j') =>  {
                self.cycle.next();
            },
            KeyCode::Up | KeyCode::Char('k') => {
                self.cycle.prev();
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                self.enter_directory();
            }
            KeyCode::Char('h') => {
                self.leave_directory();
            }
            KeyCode::Char('a') => {
            }
            _ => {}
        }
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

    pub fn new(config: &Config) -> Self {
        let file_directory = config.data_directories.last().unwrap();
        match FileState::get_file_list(file_directory) {
            Ok(item) => {
                let item_len = item.len();
                Self {
                    file_hierarchy: String::from(""),
                    files: item,
                    cycle: BiCycle::new(item_len),
                    base_path: PathBuf::from(file_directory.clone()),
                    mode: FileMode::Dir
                }
            }
            Err(_) => {
                panic!("{}", format!("Data directories specified in config: {} is not a directory!", file_directory));
            }
        }
    }

    pub fn enter_directory(&mut self){
        // enter directory specify by `self.cycle.current_item`
        let selected_file = self
            .files
            .get(self.cycle.current_item)
            .unwrap();
        self.base_path.push(selected_file);
        match FileState::get_file_list(&self.base_path) {
            Ok(files) => {
                let item_len = files.len();
                self.files = files;
                self.cycle = BiCycle::new(item_len);
            }
            Err(e) => {
                self.files.clear();
                self.mode = FileMode::File;
            }
        }
    }

    pub fn leave_directory(&mut self) {
        self.base_path.pop();
        match FileState::get_file_list(&self.base_path) {
            Ok(files) => {
                let item_len = files.len();
                self.files = files;
                self.cycle = BiCycle::new(item_len);
                self.mode = FileMode::Dir;
            }
            Err(e) => {
                panic!("{}", format!("Getting file list from dir: {:?} failed with {:?}", self.base_path, e))
            }
        }
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

