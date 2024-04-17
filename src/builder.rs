use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use minijinja::context;

use crate::config;
use crate::rss;

pub struct Builder {
    env: minijinja::Environment<'static>,
}

impl Builder {
    pub fn new() -> Self {
        let mut env = minijinja::Environment::new();

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

    async fn build_index(&self) -> Result<(), ()> {
        let path = PathBuf::from("config.toml");
        let config = config::from_file(path).unwrap();

        let articles = rss::aggregate_rss_items(config.rss.urls).await.unwrap();

        let template = self.env.get_template("index.html").unwrap();
        let page = context! {
            title => config.title,
            description => config.description,
            header => config.header,
            footer => config.footer,
            profile => config.profile,
            articles => articles,
        };

        let content = template.render(context!(page)).unwrap();

        // save file
        let mut file = File::create("./generates/index.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    pub async fn build(&self) -> Result<(), ()> {
        self.build_index().await.unwrap();

        Ok(())
    }
}
