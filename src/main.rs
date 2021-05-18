#[macro_use]
mod util;

use std::path::PathBuf;

mod config;
mod data;
mod views;

pub use crate::views::*;

static CONFIG_FILE: &str = "kb.conf";

fn init<T: Into<PathBuf>>(config_file: T) -> std::io::Result<config::Config> {
    let config;
    let config_file_ = config_file.into();
    config = config::Config::new(&config_file_).expect("Error in reading config file");
    Ok(config)
}

fn main() {
    let config = init(CONFIG_FILE).unwrap();
    let mut d = data::Handler::new(&config);
    d.read_all_files();
    ui::ui(d);
}
