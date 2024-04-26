use std::error::Error;

use chrono::DateTime;
use chrono_tz::{Asia::Tokyo, Tz};
use reqwest;
use rss::Channel;

use crate::articles::article::Source;

pub struct ExternalArticle {
    pub title: String,
    pub url: String,
    pub pub_date: DateTime<Tz>,
}

impl ExternalArticle {
    pub fn new(title: String, url: String, pub_date: DateTime<Tz>) -> Self {
        Self {
            title,
            url,
            pub_date: pub_date.with_timezone(&Tokyo),
        }
    }

    pub fn source(&self) -> Source {
        if self.url.contains("zenn.dev") {
            if self.url.contains("articles") {
                return Source::Zenn;
            }
            return Source::ZennScraps;
        }
        if self.url.contains("hatenablog.com") {
            return Source::HatenaBlog;
        }
        Source::Unknown
    }
}

pub struct ExternalArticles {
    pub articles: Vec<ExternalArticle>,
}

impl ExternalArticles {
    pub async fn from_rss(urls: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut articles = Vec::new();

        for url in urls {
            let body = reqwest::get(url).await?.bytes().await?;
            let channel = Channel::read_from(&body[..])?;

            for item in channel.items() {
                let title = match item.title() {
                    Some(title) => title.to_string(),
                    None => continue,
                };
                let url = match item.link() {
                    Some(url) => url.to_string(),
                    None => continue,
                };
                let pub_date = match item.pub_date() {
                    Some(pub_date) => {
                        DateTime::parse_from_rfc2822(&pub_date)?.with_timezone(&Tokyo)
                    }
                    None => continue,
                };

                articles.push(ExternalArticle::new(title, url, pub_date));
            }
        }

        Ok(Self { articles })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    const RSS_BODY: &str = "
<rss version=\"2.0\">
<channel>
<title>ucpr.dev rss feed</title>
<link>https://www.ucpr.dev</link>
<description>ucpr's articles rss feed</description>
<item>
<title>GoでXMLのCDATAにXMLを埋め込む</title>
<link>https://www.ucpr.dev/articles/go_nested_xml_in_cdata.html</link>
<description>
<![CDATA[ GoでXMLのCDATAにXMLを埋め込む ]]>
</description>
<pubDate>Thu, 25 Apr 2024 00:00:00 +0900</pubDate>
</item>
<item>
<title>markdown-rsでfront matterの値を参照する</title>
<link>https://www.ucpr.dev/articles/markdown-rs_front-matter.html</link>
<description>
<![CDATA[ markdown-rsでfront matterの値を参照する ]]>
</description>
<pubDate>Thu, 18 Apr 2024 00:00:00 +0900</pubDate>
</item>
<item>
<title>テスト記事</title>
<link>https://www.ucpr.dev/articles/test.html</link>
<description>
<![CDATA[ テスト記事 ]]>
</description>
<pubDate>Sat, 1 Jan 2000 00:00:00 +0900</pubDate>
</item>
</channel>
</rss>
";

    #[tokio::test]
    async fn test_articles_from_rss() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/rss+xml")
            .with_body(RSS_BODY)
            .create_async()
            .await;

        let urls = vec![url.to_string()];
        match ExternalArticles::from_rss(urls).await {
            Ok(articles) => {
                assert_eq!(articles.articles.len(), 3);
            }
            Err(e) => {
                panic!("Failed to fetch RSS: {:?}", e);
            }
        };

        mock.assert();
    }
}
