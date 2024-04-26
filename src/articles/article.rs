use std::error::Error;

use chrono::DateTime;
use chrono_tz::{Asia::Tokyo, Tz};
use minijinja;
use serde::Serialize;

use crate::articles::external;
use crate::articles::local;
use crate::config;

#[derive(Debug, Clone, Serialize)]
pub enum Source {
    Unknown,    // 不明
    Local,      // overture で生成された記事
    Zenn,       // Zenn の記事
    ZennScraps, // Zenn のスクラップ
    HatenaBlog, // はてなブログ
}

impl ToString for Source {
    fn to_string(&self) -> String {
        match self {
            Source::Unknown => "Unknown".to_string(),
            Source::Local => "Local".to_string(),
            Source::Zenn => "Zenn".to_string(),
            Source::ZennScraps => "Zenn Scraps".to_string(),
            Source::HatenaBlog => "HatenaBlog".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub source: Source,
    pub pub_date: String,
}

pub struct Articles {
    local_articles: local::LocalArticles,
    external_articles: external::ExternalArticles,
    env: minijinja::Environment<'static>,
    default_ctx: minijinja::Value,
}

impl Articles {
    pub async fn new(
        external_rss_links: Vec<String>,
        env: minijinja::Environment<'static>,
        default_ctx: minijinja::Value,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            local_articles: local::LocalArticles::new()?,
            external_articles: external::ExternalArticles::from_rss(external_rss_links).await?,
            env,
            default_ctx,
        })
    }

    fn format_jst_pub_date(&self, pub_date: DateTime<Tz>) -> String {
        pub_date
            .with_timezone(&Tokyo)
            .format("%Y/%m/%d")
            .to_string()
    }

    pub fn build_articles(&self) -> Result<(), ()> {
        self.local_articles
            .build_articles(&self.env, &self.default_ctx)
    }

    pub fn generate_rss(&self, cfg: &config::Rss) -> Result<(), Box<dyn Error>> {
        self.local_articles.generate_rss(cfg)
    }

    pub fn aggregate_articles(&self) -> Result<Vec<Article>, Box<dyn Error>> {
        let mut articles = Vec::new();

        for article in &self.local_articles.articles {
            articles.push(Article {
                title: article.options.title.clone(),
                url: article.url_path(),
                source: Source::Local,
                pub_date: self.format_jst_pub_date(article.pub_date),
            });
        }

        for article in &self.external_articles.articles {
            articles.push(Article {
                title: article.title.clone(),
                url: article.url.clone(),
                source: article.source(),
                pub_date: self.format_jst_pub_date(article.pub_date),
            });
        }

        articles.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

        Ok(articles)
    }
}
