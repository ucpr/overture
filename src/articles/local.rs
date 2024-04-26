use std::error::Error;
use std::fs;
use std::io::prelude::*;

use chrono::DateTime;
use chrono_tz::{Asia::Tokyo, Tz};
use markdown;
use minijinja::context;
use rss::ChannelBuilder;
use serde::Deserialize;

use crate::config::Rss;

#[derive(Debug, Deserialize)]
pub struct Options {
    pub title: String,
    pub description: String,
    pub date: String,
    pub tags: Vec<String>,
}

pub struct LocalArticle {
    pub raw_body: String,
    pub file_name: String,
    pub pub_date: DateTime<Tz>,
    pub options: Options,
}

impl LocalArticle {
    fn options(raw_body: &str) -> Result<Options, toml::de::Error> {
        let config = &markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..markdown::Constructs::default()
            },
            ..markdown::ParseOptions::default()
        };
        let tree = markdown::to_mdast(raw_body, config).ok().unwrap();

        let mut front_matter: &str = "";
        tree.children().into_iter().for_each(|node| {
            for child in node.iter() {
                match child {
                    markdown::mdast::Node::Toml(toml) => {
                        front_matter = &toml.value;
                        break;
                    }
                    _ => {}
                }
            }
        });

        toml::from_str(front_matter)
    }

    pub fn from_file(path: &str) -> Result<LocalArticle, std::io::Error> {
        let file_name = path.split('/').last().unwrap().to_string();
        let raw_body = std::fs::read_to_string(path)?;
        let options = LocalArticle::options(&raw_body).unwrap();
        let pub_date = DateTime::parse_from_rfc3339(&options.date)
            .unwrap()
            .with_timezone(&Tokyo);

        Ok(LocalArticle {
            file_name,
            raw_body,
            options,
            pub_date,
        })
    }

    fn build(&self) -> String {
        let opts = markdown::Options {
            parse: markdown::ParseOptions {
                constructs: markdown::Constructs {
                    frontmatter: true,
                    ..markdown::Constructs::gfm()
                },
                ..markdown::ParseOptions::gfm()
            },
            ..markdown::Options::gfm()
        };

        let mut body = self.raw_body.clone();
        body = markdown::to_html_with_options(&body, &opts).unwrap();

        format!("<h1>{}</h1><div>{}</div>", self.options.title, body)
    }

    pub fn save(
        &self,
        env: &minijinja::Environment<'static>,
        default_ctx: &minijinja::Value,
        path: &str,
    ) -> Result<(), std::io::Error> {
        let html = self.build();

        let template = env.get_template("article.html").unwrap();
        let base = self.file_name.split('.').next().unwrap();
        let page = context! {
            ..context!{
                content => html,
                url_path => format!("/articles/{}", base),
                title => self.options.title,
                description => self.options.description,
            },
            ..default_ctx.clone(),
        };
        let content = template.render(context!(page)).unwrap();

        std::fs::write(path, content)
    }
}

pub struct LocalArticles {
    pub articles: Vec<LocalArticle>,
}

impl LocalArticles {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut articles = Vec::new();

        let paths = std::fs::read_dir("articles")?;
        for path in paths {
            let path = path?.path();
            let path = match path.to_str() {
                Some(path) => path,
                None => continue,
            };
            let article = LocalArticle::from_file(path)?;

            articles.push(article);
        }
        Ok(Self { articles })
    }

    pub fn build_articles(
        &self,
        env: &minijinja::Environment<'static>,
        default_ctx: &minijinja::Value,
    ) -> Result<(), ()> {
        for article in &self.articles {
            let path = format!(
                "generates/articles/{}.html",
                article.file_name.split('.').next().unwrap()
            );
            article.save(env, default_ctx, &path).unwrap();
        }
        Ok(())
    }

    pub fn generate_rss(&self, config: &Rss) -> Result<(), Box<dyn Error>> {
        let mut items = Vec::new();
        for article in &self.articles {
            items.push(
                rss::ItemBuilder::default()
                    .title(article.options.title.clone())
                    .link(format!("/articles/{}", article.file_name))
                    .pub_date(article.pub_date.to_rfc2822())
                    .description(article.options.description.clone())
                    .build(),
            );
        }

        let channel = ChannelBuilder::default()
            .title(&config.title)
            .description(&config.description)
            .link(&config.url)
            .items(items)
            .build();
        let mut file = fs::File::create("generates/rss.xml")?;
        file.write_all(channel.to_string().as_bytes())?;
        Ok(())
    }
}
