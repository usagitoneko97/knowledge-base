use std::fs::read_to_string;
use structopt::StructOpt;
use crate::data::{Knowledge};
use std::path::{PathBuf, Path};
use std::str::FromStr;
use crossterm::event::KeyCode::Enter;

mod data;
mod config;
mod views;

pub use crate::views::*;

static CONFIG_FILE: &str = "kb.conf";

#[derive(StructOpt)]
struct Kb {
    #[structopt(short, long, parse(from_os_str), default_value=CONFIG_FILE)]
    config: PathBuf,
    #[structopt(subcommand)]
    cmd: KbSub
}

#[derive(StructOpt)]
enum KbSub {
    Add {
        topic: String,

        #[structopt(short, long)]
        content: String,

        #[structopt(short, long)]
        tag: String,

        #[structopt(short, long)]
        descriptions: String,
    }
}

fn init <T: Into<PathBuf>> (config_file: T) -> std::io::Result<(config::Config)> {
    // init default config location
    let config;
    let config_file_ = config_file.into();
    config = config::Config::new(&config_file_)
        .expect("Error in reading config file");
    Ok(config)
}

enum C<T> {
    Field(T)
}

fn main() {
    let mut config = init(CONFIG_FILE).unwrap();
    let mut d = data::Handler::new(&config);
    d.read_all_files();
    ui::ui(d);
}

