use markdown;
use serde::Deserialize;
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;

use crate::rss::{Item, Source};

#[derive(Debug, Deserialize)]
pub struct Options {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
}

pub struct Article {
    raw_body: String,
    options: Options,
    pub ext: String,
    pub file_name: String,
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

        Ok(Article {
            ext,
            file_name,
            raw_body,
            options,
        })
    }

    fn build(&self) -> String {
        let opts = markdown::Options {
            parse: markdown::ParseOptions {
                constructs: markdown::Constructs {
                    frontmatter: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let mut body = self.raw_body.clone();
        if self.ext == "md" {
            body = markdown::to_html_with_options(&body, &opts).unwrap();
        }

        format!("<h1>{}</h1><div>{}</div>", self.options.title, body)
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let html = self.build();
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
tags = [\"test\", \"article\"]
+++

## h2
body
";

    #[test]
    fn test_build() {
        let article = {
            let title = "Test Article".to_string();
            let raw_body = BODY.to_string();
            let ext = "md".to_string();
            let options = Options {
                title,
                date: "2021-01-01".to_string(),
                tags: vec!["test".to_string(), "article".to_string()],
            };
            Article {
                ext,
                file_name: "test.md".to_string(),
                raw_body,
                options,
            }
        };

        let want = "<h1>Test Article</h1><div><h2>h2</h2>\n<p>body</p>\n</div>";
        assert_eq!(article.build(), want.to_string());
    }

    #[test]
    fn test_hash() {
        let article = {
            let title = "Test Article".to_string();
            let raw_body = BODY.to_string();
            let ext = "md".to_string();
            let options = Options {
                title,
                date: "2021-01-01".to_string(),
                tags: vec!["test".to_string(), "article".to_string()],
            };
            Article {
                ext,
                file_name: "test.md".to_string(),
                raw_body,
                options,
            }
        };

        let want = 17706308674555399441;
        assert_eq!(article.hash(), want);
    }
}
