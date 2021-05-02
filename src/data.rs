use std::collections::HashMap;
use std::hash::Hash;
use std::path::{Path, PathBuf, Display};
use std::fs::{read_to_string, create_dir};
use std::fmt::Error;
use std::io;
use serde::{Deserialize, Serialize};
use core::fmt;
use crate::config::Config;
use toml::ser::Error::KeyNotString;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Knowledge {
    title: String,
    tag: Vec<String>,
    text: String,
    descriptions: String
}

impl Knowledge {
    pub fn new (title: String, text: String, descriptions: String) -> Self {
        Knowledge{
            title,
            tag: Vec::new(),
            text,
            descriptions
        }
    }

    pub fn from_file<T: AsRef<Path>> (file: T) -> Self {
        let f = file.as_ref();
        let mut res = std::fs::read_to_string(f).unwrap();
        let mut title: String = String::new();
        let mut descriptions = String::new();
        let mut content = String::new();
        for line in res.split("\n").into_iter() {
            if line.contains("# Title:") {
                let t: String = line.replace("# Title:", "");
                title = t.trim().into();
                continue;
            } else if line.contains("# Descriptions:") {
                let t: String = line.replace("# Descriptions:", "");
                descriptions = t.trim().into();
                continue;
            }
            content += line;
        }
        Knowledge{
            title: title.to_string(), descriptions: descriptions.to_string(),
            tag: Vec::new(),
            text: content.trim().to_string()
        }
    }

    pub fn write(&self, config: &Config) -> std::io::Result<()> {
        let parent = &config.data_directories.get(0).expect("Must contain 1 data directories");
        let mut path = PathBuf::new();
        path.push(parent);
        if !path.exists() {
            create_dir(&path).expect(format!("Failed to create directory: {:?}", path).as_str());
        }
        let file: String = self.title.clone() + "." + &config.extension;
        path.push(file);
        std::fs::write(&path, &format!("{}", self)).expect("Error in writing file");
        Ok(())
    }
}

impl fmt::Display for Knowledge{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("# Title: {}\n", self.title));
        result.push_str(&format!("# Descriptions: {}\n\n", self.descriptions));
        result.push_str(&self.text);
        write!(f, "{}", result)
    }
}

pub struct Handler <'a> {
    pub datas: HashMap<String, Knowledge>,
    config: &'a Config
}

impl <'a> Handler <'a>  {
    pub fn new (config: &'a Config) -> Self {
        Handler{
            datas: HashMap::<String, Knowledge>::new(),
            config
        }
    }

    pub fn add_knowledge(&mut self, k: Knowledge) {
        self.datas.insert(k.title.clone(), k);
    }

    pub fn read_all_files(&mut self) {
        for dir in self.config.data_directories.iter() {
            let f = std::fs::read_dir(dir).unwrap();
            for file in f {
                let file_path = file.unwrap().path();
                if file_path.extension().unwrap() == self.config.extension.as_str() {
                    let k = Knowledge::from_file(file_path);
                    self.datas.insert(k.title.clone(), k);
                }
            }
        }
    }
}
