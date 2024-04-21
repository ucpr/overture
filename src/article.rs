use std::path::PathBuf;

use markdown;
use minijinja::context;
use serde::Deserialize;
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;

use crate::config;
use crate::rss::{Item, Source};

#[derive(Debug, Deserialize)]
pub struct Options {
    pub title: String,
    pub description: String,
    pub date: String,
    pub tags: Vec<String>,
}

pub struct Article {
    raw_body: String,
    options: Options,
    pub ext: String,
    pub file_name: String,

    env: minijinja::Environment<'static>,
    default_ctx: minijinja::Value,
}

pub struct Articles {
    articles: Vec<Article>,
}

impl Article {
    fn options(raw_body: &str) -> Result<Options, toml::de::Error> {
        let config = &markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..markdown::Constructs::default()
            },
            ..markdown::ParseOptions::default()
        };
        let tree = markdown::to_mdast(raw_body, config).ok().unwrap();

        let mut front_matter = String::new();
        tree.children().into_iter().for_each(|node| {
            for child in node.iter() {
                match child {
                    markdown::mdast::Node::Toml(toml) => {
                        front_matter = toml.value.clone();
                    }
                    _ => {}
                }
            }
        });

        toml::from_str(&front_matter)
    }

    pub fn from_file(path: &str) -> Result<Article, std::io::Error> {
        let ext = path.split('.').last().unwrap().to_string();
        let file_name = path.split('/').last().unwrap().to_string();
        let raw_body = std::fs::read_to_string(path)?;
        let options = Article::options(&raw_body).unwrap();

        let mut env = minijinja::Environment::new();
        #[cfg(feature = "bundled")]
        {
            minijinja_embed::load_templates!(&mut env);
        }
        #[cfg(not(feature = "bundled"))]
        {
            env.set_loader(minijinja::path_loader("./src/templates"));
        }

        let config_path = PathBuf::from("config.toml");
        let config = config::from_file(config_path).unwrap();
        let default_ctx = context! {
            url => config.url,
            header => config.header,
            footer => config.footer,
            google_analytics => config.google_analytics,
        };

        Ok(Article {
            ext,
            file_name,
            raw_body,
            options,
            env,
            default_ctx,
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
        if self.ext == "md" {
            body = markdown::to_html_with_options(&body, &opts).unwrap();
        }

        format!("<h1>{}</h1><div>{}</div>", self.options.title, body)
    }

    fn render(&self) -> String {
        let html = self.build();

        let template = self.env.get_template("article.html").unwrap();
        let path = self.file_name.split('.').next().unwrap();
        let page = context! {
            ..self.default_ctx.clone(),
            ..context!{
                content => html,
                url_path => format!("/articles/{}", path),
                title => self.options.title,
                description => self.options.description,
            },
        };
        let content = template.render(context!(page)).unwrap();

        content
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let html = self.render();
        std::fs::write(path, html)
    }

    #[allow(dead_code)]
    fn hash(&self) -> u64 {
        const_xxh3(self.build().as_bytes())
    }
}

impl Articles {
    pub fn new() -> Articles {
        let mut articles = Vec::new();

        let paths = std::fs::read_dir("articles").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap();
            let article = Article::from_file(path).unwrap();

            articles.push(article);
        }

        Articles { articles }
    }

    pub fn save(&self) -> Result<Vec<Item>, std::io::Error> {
        let mut items: Vec<Item> = Vec::new();

        for article in &self.articles {
            let name = &article.file_name;
            let name = name.split('.').next().unwrap();

            let path = format!("generates/articles/{}.html", name);
            article.save(&path)?;

            items.push(Item {
                title: article.options.title.clone(),
                link: format!("articles/{}.html", name),
                source: Source::Unknown,
                pub_date: article.options.date.clone(),
            });
        }

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "+++
title = \"Test Article\"
description = \"Test article description\"
tags = [\"test\", \"article\"]
+++

## h2
body
";

    #[test]
    fn test_build() {
        let env = minijinja::Environment::new();
        let default_ctx = context! {};
        let article = {
            let title = "Test Article".to_string();
            let description = "Test article description".to_string();
            let raw_body = BODY.to_string();
            let ext = "md".to_string();
            let options = Options {
                title,
                description,
                date: "2021-01-01".to_string(),
                tags: vec!["test".to_string(), "article".to_string()],
            };
            Article {
                ext,
                file_name: "test.md".to_string(),
                raw_body,
                options,
                env,
                default_ctx,
            }
        };

        let want = "<h1># Test Article</h1><div><h2>h2</h2>\n<p>body</p>\n</div>";
        assert_eq!(article.build(), want.to_string());
    }
}
