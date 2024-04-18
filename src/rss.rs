use std::error::Error;

use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;
use reqwest;
use rss::Channel;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Source {
    Unknown,
    Zenn,
    ZennScrap,
    HatenaBlog,
}

impl ToString for Source {
    fn to_string(&self) -> String {
        match self {
            Source::Unknown => "Unknown".to_string(),
            Source::Zenn => "Zenn".to_string(),
            Source::ZennScrap => "Zenn Scraps".to_string(),
            Source::HatenaBlog => "HatenaBlog".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub source: Source,
    pub pub_date: String,
}

pub fn format_jst_pub_date(pub_date: DateTime<Utc>) -> String {
    pub_date
        .with_timezone(&Tokyo)
        .format("%Y/%m/%d")
        .to_string()
}

pub async fn fetch_feed(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn aggregate_rss_items(urls: Vec<String>) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut items = Vec::new();

    for url in urls {
        let channel = fetch_feed(url.to_string()).await?;
        for item in channel.items() {
            let pub_date: DateTime<Utc> =
                DateTime::parse_from_rfc2822(item.pub_date().unwrap())?.into();
            let source = detect_source(item.link().unwrap());

            items.push(Item {
                title: item.title().unwrap().to_string(),
                link: item.link().unwrap().to_string(),
                source,
                pub_date: format_jst_pub_date(pub_date).to_string(),
            });
        }
    }

    Ok(items)
}

fn detect_source(url: &str) -> Source {
    if url.contains("zenn.dev") && url.contains("articles") {
        Source::Zenn
    } else if url.contains("zenn.dev") && url.contains("scraps") {
        Source::ZennScrap
    } else if url.contains("hatenablog.com") {
        Source::HatenaBlog
    } else {
        Source::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println as info;

    #[tokio::test]
    async fn test_aggregate_rss_items() {
        let items = aggregate_rss_items(vec![
            "https://zenn.dev/ucpr/feed?include_scraps=1".to_string()
        ])
        .await
        .unwrap();
        for item in &items {
            info!("{}: {} ({})", item.pub_date, item.title, item.link);
        }
        assert!(items.len() > 0);
    }
}
