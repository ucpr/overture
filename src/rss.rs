use std::error::Error;

use reqwest;
use serde::Serialize;
use rss::Channel;
use chrono::{Utc, DateTime};
use chrono_tz::Asia::Tokyo;

#[derive(Debug, Serialize)]
pub enum Source {
    Unknown,
    Zenn,
    ZennScrap,
    HatenaBlog,
}

#[derive(Debug, Serialize)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub source: Source,
    pub pub_date: String,
}

pub fn format_jst_pub_date(pub_date: DateTime<Utc>) -> String {
    pub_date.with_timezone(&Tokyo).format("%Y/%m/%d").to_string()
}

pub async fn fetch_feed(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn aggregate_rss_items(urls: Vec<String>) -> Result<Vec<RssItem>, Box<dyn Error>> {
    let mut items = Vec::new();

    for url in urls {
        let channel = fetch_feed(url.to_string()).await?;
        for item in channel.items() {
            let pub_date: DateTime<Utc> = DateTime::parse_from_rfc2822(item.pub_date().unwrap())?.into();
            let source = detect_source(item.link().unwrap());

            items.push(RssItem {
                title: item.title().unwrap().to_string(),
                link: item.link().unwrap().to_string(),
                source,
                pub_date: format_jst_pub_date(pub_date).to_string(),
            });
        }
    }

    items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
    Ok(items)
}

fn detect_source(url: &str) -> Source {
    if url.contains("zenn.dev") && url.contains("articles"){
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
            "https://zenn.dev/ucpr/feed?include_scraps=1".to_string(),
        ]).await.unwrap();
        for item in &items {
            info!("{}: {} ({})", item.pub_date, item.title, item.link);
        }
        assert!(items.len() > 0);
    }
}