use serde::Serialize;

use crate::articles::external;
use crate::articles::local;

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

pub struct Articles {
    local_articles: Vec<local::LocalArticle>,
    external_articles: external::ExternalArticles,
}

impl Articles {}
