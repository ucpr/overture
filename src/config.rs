use serde::{Deserialize, Serialize};
use toml;

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
}

pub fn from_file(path: PathBuf) -> Result<Config, toml::de::Error> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    toml::from_str(&contents)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            title: "Default Title".to_string(),
        }
    }
}

impl Config {
    pub fn to_file(&self, path: PathBuf) -> Result<(), ()> {
        let toml = toml::to_string(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        Ok(())
    }
}
