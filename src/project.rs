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
// ├── generates/   -- Generates files
// └── statics/    -- Static files (images, css, js, etc.)
//

impl Project {
    pub fn new(root: PathBuf) -> Self {
        Project { root }
    }

    pub fn create(&self) -> std::io::Result<()> {
        let root = PathBuf::from(".");
        fs::create_dir_all(root.join("contents"))?;
        fs::create_dir_all(root.join("generates"))?;
        fs::create_dir_all(root.join("statics"))?;
        fs::write(root.join("config.toml"), "")?;
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> String {
        let full_path = self.root.join(path);
        fs::read_to_string(full_path).expect("Unable to read file")
    }
}
