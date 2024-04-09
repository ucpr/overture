use crate::config;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use minijinja::{context, Environment};

pub struct Builder {
    env: Environment<'static>,
}

impl Builder {
    pub fn new() -> Self {
        let mut env = Environment::new();

        #[cfg(feature = "bundled")]
        {
            minijinja_embed::load_templates!(&mut env);
        }

        #[cfg(not(feature = "bundled"))]
        {
            env.set_loader(minijinja::path_loader("./src/templates"));
        }

        Builder { env }
    }

    pub fn build(&self) -> Result<(), ()> {
        let path = PathBuf::from("config.toml");
        let config = config::from_file(path).unwrap();

        let title = config.title;

        let template = self.env.get_template("index.html").unwrap();
        let page = context! {
            title => title,
            content => "Lorum Ipsum",
        };

        let content = template.render(context!(page)).unwrap();

        // save file
        let mut file = File::create("./generates/index.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }
}
