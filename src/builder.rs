use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use minijinja::context;

use crate::config;
use crate::rss;

pub struct Builder {
    env: minijinja::Environment<'static>,
    config: config::Config,
    default_ctx: minijinja::Value,
}

impl Builder {
    pub fn new() -> Self {
        let mut env = minijinja::Environment::new();

        let config_path = PathBuf::from("config.toml");
        let config = config::from_file(config_path).unwrap();
        let default_ctx = context! {
            title => config.title,
            description => config.description,
            header => config.header,
            footer => config.footer,
            profile => config.profile,
        };

        #[cfg(feature = "bundled")]
        {
            minijinja_embed::load_templates!(&mut env);
        }

        #[cfg(not(feature = "bundled"))]
        {
            env.set_loader(minijinja::path_loader("./src/templates"));
        }

        Builder { env, config, default_ctx }
    }

    fn context(&self, ctx: minijinja::Value) -> minijinja::Value {
        context! {
            ..self.default_ctx.clone(),
            ..ctx,
        }
    }

    async fn build_index(&self) -> Result<(), ()> {
        let articles = rss::aggregate_rss_items(self.config.rss.urls.clone()).await.unwrap();

        let template = self.env.get_template("index.html").unwrap();
        let page = self.context(
            context! {
                articles => articles,
            }
        );
        let content = template.render(context!(page)).unwrap();

        // save file
        let mut file = File::create("./generates/index.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    async fn build_articles(&self) -> Result<(), ()> {
        let articles = rss::aggregate_rss_items(self.config.rss.urls.clone()).await.unwrap();

        let template = self.env.get_template("articles.html").unwrap();
        let page = self.context(
            context! {
                articles => articles,
            }
        );
        let content = template.render(context!(page)).unwrap();

        // save file
        let mut file = File::create("./generates/articles.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    pub async fn build(&self) -> Result<(), ()> {
        self.build_index().await.unwrap();
        self.build_articles().await.unwrap();

        Ok(())
    }
}
