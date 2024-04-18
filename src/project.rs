use std::fs;
use std::path;

use crate::config::Config;

pub struct Project {
    root: path::PathBuf,
}

// Project Structure
//
// .
// ├── config.toml -- Project configuration
// ├── articles/   -- article files
// ├── generates/   -- Generates files
// └── statics/    -- Static files (images, css, js, etc.)
//

impl Project {
    pub fn new(root: path::PathBuf) -> Self {
        Project { root }
    }

    pub fn create(&self) -> std::io::Result<()> {
        let root = path::PathBuf::from(".");
        fs::create_dir_all(root.join("articles"))?;
        fs::create_dir_all(root.join("generates/articles"))?;
        fs::create_dir_all(root.join("generates/statics"))?;
        fs::create_dir_all(root.join("statics"))?;
        let _ = Config::default().to_file(root.join("config.toml"));

        Ok(())
    }

    pub fn read_file(&self, path: &str) -> String {
        let full_path = self.root.join(path);
        fs::read_to_string(full_path).expect("Unable to read file")
    }
}
