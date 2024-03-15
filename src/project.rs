use std::fs;
use std::path::PathBuf;

pub struct Project {
    root: PathBuf,
}

// Project Structure
//
// .
// ├── config.toml -- Project configuration
// ├── contents/   -- Content files
// └── statics/    -- Generated static files
//

impl Project {
    pub fn new(root: PathBuf) -> Self {
        Project { root }
    }

    pub fn read_file(&self, path: &str) -> String {
        let full_path = self.root.join(path);
        fs::read_to_string(full_path).expect("Unable to read file")
    }
}
