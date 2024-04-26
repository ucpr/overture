use chrono::DateTime;
use chrono_tz::{Asia::Tokyo, Tz};
use markdown;
use minijinja::context;
use serde::Deserialize;

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

    fn render(
        &self,
        env: minijinja::Environment<'static>,
        default_ctx: minijinja::Value,
    ) -> String {
        let html = self.build();

        let template = env.get_template("article.html").unwrap();
        let path = self.file_name.split('.').next().unwrap();
        let page = context! {
            ..context!{
                content => html,
                url_path => format!("/articles/{}", path),
                title => self.options.title,
                description => self.options.description,
            },
            ..default_ctx.clone(),
        };
        let content = template.render(context!(page)).unwrap();

        content
    }

    pub fn save(
        &self,
        env: minijinja::Environment<'static>,
        default_ctx: minijinja::Value,
        path: &str,
    ) -> Result<(), std::io::Error> {
        let html = self.render(env, default_ctx);
        std::fs::write(path, html)
    }
}
