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
use glob::glob;


#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Knowledge {
    pub title: String,
    pub tag: Vec<String>,
    pub text: String,
    pub descriptions: String,
}

impl Knowledge {
    pub fn new(title: String, text: String, descriptions: String, tag: String) -> Self {
        Knowledge {
            title,
            tag: vec![tag],
            text,
            descriptions,
        }
    }

    pub fn from_file<P: Into<PathBuf>>(file: P) -> Self {
        let f = file.into();
        let mut res = std::fs::read_to_string(f).unwrap();
        let mut title: String = String::new();
        let mut descriptions = String::new();
        let mut tags: Vec<String> = vec![];
        for line in res.split("\n").into_iter() {
            if line.contains("# Title:") {
                let t: String = line.replace("# Title:", "");
                title = t.trim().into();
                continue;
            } else if line.contains("# Descriptions:") {
                let t: String = line.replace("# Descriptions:", "");
                descriptions = t.trim().into();
                continue;
            } else if line.contains("# Tags: ") {
                let t = line.replace("# Tags:", "");
                tags = t.split(",")
                    .map(|e| e.trim().to_owned()).collect();
            }
        }
        Knowledge {
            title: title.to_string(),
            descriptions: descriptions.to_string(),
            tag: tags,
            text: res,
        }
    }

    pub fn write(&self, config: &Config) -> std::io::Result<()> {
        let parent = &config.data_directories.get(0).expect("Must contain 1 data directories");
        let mut path = PathBuf::new();
        path.push(parent);
        if !path.exists() {
            create_dir(&path).expect(format!("Failed to create directory: {:?}", path).as_str());
        }
        self.write_to_file(path, &config.extension)
    }

    pub fn write_to_file<T: Into<PathBuf>>(&self, parent_dir: T, ext: &str) -> std::io::Result<()> {
        let mut path = parent_dir.into();
        let file: String = self.title.clone() + "." + ext;
        path.push(file);
        std::fs::write(&path, &format!("{}", self))?;
        Ok(())
    }

}

impl fmt::Display for Knowledge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("# Title: {}\n", self.title));
        result.push_str(&format!("# Descriptions: {}\n\n", self.descriptions));
        result.push_str(&self.text);
        write!(f, "{}", result)
    }
}

pub struct Handler<'a> {
    pub data: Vec<Knowledge>,
    pub config: &'a Config,
}

impl<'a> Handler<'a> {
    pub fn new(config: &'a Config) -> Self {
        Handler {
            data: vec![],
            config,
        }
    }

    pub fn read_all_files(&mut self) {
        for dir in self.config.data_directories.iter() {
            let mut glob_pattern = dir.clone();
            glob_pattern = glob_pattern + "/**/" + "*." + &self.config.extension;
            for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(f) => {
                        let k = Knowledge::from_file(f);
                        self.data.push(k);
                    }
                    _ => {}
                }
            }
        }
    }
    pub fn get_mapping(&self) -> HashMap<String, Vec<&Knowledge>> {
        let mut mapping: HashMap<String, Vec<&Knowledge>> = HashMap::new();
        for k in self.data.iter() {
            for tag in k.tag.clone() {
                mapping.entry(tag.clone()).or_default().push(k);
            }
        }
        // println!("{:?}", mapping);
        mapping
    }
}
