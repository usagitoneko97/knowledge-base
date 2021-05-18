use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Deserialize)]
pub struct Config {
    pub data_directories: Vec<String>,
    pub extension: String,
}

impl Config {
    pub fn new(config_file: &PathBuf) -> std::io::Result<Self> {
        let config = read_to_string(config_file)?;
        let toml_value = config.parse::<Value>().unwrap();
        let s = toml_value["data_directories"].as_array().unwrap();
        let data_directories_file: Vec<String> = s
            .iter()
            .map(|x| String::from(x.as_str().unwrap()))
            .collect();
        Ok(Config {
            data_directories: data_directories_file,
            extension: String::from(toml_value["extension"].as_str().unwrap()),
        })
    }
}
