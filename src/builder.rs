use std::fs;
use std::io::Write;
use std::path;

use fs_extra::dir;
use minijinja::context;

use crate::article;
use crate::config;
use crate::rss;

pub struct Builder {
    env: minijinja::Environment<'static>,
    config: config::Config,
    default_ctx: minijinja::Value,

    articles: Vec<rss::Item>,
}

impl Builder {
    pub async fn new() -> Self {
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

        let external_articles = rss::aggregate_rss_items(config.rss.external_rss_links.clone())
            .await
            .unwrap();
        let art = article::Articles::new(env.clone(), default_ctx.clone());
        let mut articles = art.save().unwrap(); // save original articles

        let rss_creator = rss::RSS::new(
            config.rss.title.clone(),
            config.rss.description.clone(),
            config.url.clone(),
            articles.clone(),
        );
        rss_creator.save("./generates/rss.xml").unwrap();

        articles.extend(external_articles.clone());
        articles.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

        Builder {
            env,
            config,
            default_ctx,
            articles,
        }
    }

    fn context(&self, ctx: minijinja::Value) -> minijinja::Value {
        context! {
            ..ctx,
            ..self.default_ctx.clone(),
        }
    }

    fn build_template(&self, template_name: &str, ctx: minijinja::Value) -> Result<String, minijinja::Error>{
        let template = self.env.get_template(template_name)?;
        let page = self.context(ctx);
        template.render(context!(page))
    }

    fn build_index(&self) -> Result<(), ()> {
        let content = self.build_template("index.html", context! {
            profile => self.config.profile,
            articles => {
                let limit = self.articles.len().min(5);
                &self.articles[..limit]
            },
        }).unwrap();

        // save file
        let mut file = fs::File::create("./generates/index.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    fn build_articles(&self) -> Result<(), ()> {
        let content = self.build_template("articles.html", context! {
            articles => self.articles,
        }).unwrap();

        // save file
        let mut file = fs::File::create("./generates/articles.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    fn build_about(&self) -> Result<(), ()> {
        let content = self.build_template("about.html", context! {
            profile => self.config.profile,
            articles => self.articles,
        }).unwrap();

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
        self.build_articles().unwrap();
        self.build_about().unwrap();
        self.build_statics().unwrap();

        Ok(())
    }
}
