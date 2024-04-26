use std::error::Error;
use std::fs;
use std::io::Write;
use std::path;

use fs_extra::dir;
use minijinja::context;

use crate::articles::article;
use crate::config;

pub struct Builder {
    env: minijinja::Environment<'static>,
    config: config::Config,
    default_ctx: minijinja::Value,
    articles: article::Articles,
}

impl Builder {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let mut env = minijinja::Environment::new();
        #[cfg(feature = "bundled")]
        {
            minijinja_embed::load_templates!(&mut env);
        }
        #[cfg(not(feature = "bundled"))]
        {
            env.set_loader(minijinja::path_loader("./src/templates"));
        }

        let config_path = path::PathBuf::from("config.toml");
        let config = config::from_file(config_path).unwrap();
        let default_ctx = context! {
            title => config.title,
            description => config.description,
            header => config.header,
            footer => config.footer,
            google_analytics => config.google_analytics,
        };

        /* 目印 */
        let articles = article::Articles::new(
            config.rss.external_rss_links.clone(),
            env.clone(),
            default_ctx.clone(),
        )
        .await?;

        Ok(Builder {
            env,
            config,
            default_ctx,
            articles,
        })
    }

    fn context(&self, ctx: minijinja::Value) -> minijinja::Value {
        context! {
            ..ctx,
            ..self.default_ctx.clone(),
        }
    }

    fn build_template(
        &self,
        template_name: &str,
        ctx: minijinja::Value,
    ) -> Result<String, minijinja::Error> {
        let template = self.env.get_template(template_name)?;
        let page = self.context(ctx);
        template.render(context!(page))
    }

    fn build_index(&self) -> Result<(), ()> {
        let articles = self.articles.aggregate_articles().unwrap();

        let content = self
            .build_template(
                "index.html",
                context! {
                    profile => self.config.profile,
                    articles => {
                        let limit = articles.len().min(5);
                        &articles[..limit]
                    },
                },
            )
            .unwrap();

        // save file
        let mut file = fs::File::create("./generates/index.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    fn build_articles(&self) -> Result<(), ()> {
        let articles = self.articles.aggregate_articles().unwrap();

        let content = self
            .build_template(
                "articles.html",
                context! {
                    articles => articles,
                },
            )
            .unwrap();

        // save file
        let mut file = fs::File::create("./generates/articles.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    fn build_about(&self) -> Result<(), ()> {
        let content = self
            .build_template(
                "about.html",
                context! {
                    profile => self.config.profile,
                    // articles => self.articles,
                },
            )
            .unwrap();

        // save file
        let mut file = fs::File::create("./generates/about.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    fn build_statics(&self) -> Result<(), ()> {
        let gen_statics = path::Path::new("./generates/statics");
        if gen_statics.exists() {
            fs::remove_dir_all("./generates/statics").unwrap();
        }

        let mut copy_options = dir::CopyOptions::new();
        copy_options.overwrite = true;
        let static_src = path::Path::new("./statics");
        let static_dest = path::Path::new("./generates/");
        dir::copy(static_src, static_dest, &copy_options).unwrap();

        Ok(())
    }

    pub fn build(&self) -> Result<(), ()> {
        self.build_index().unwrap();
        self.articles.build_articles().unwrap();
        self.articles.generate_rss(&self.config.rss).unwrap();
        self.build_articles().unwrap();
        self.build_about().unwrap();
        self.build_statics().unwrap();

        Ok(())
    }
}
