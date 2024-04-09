use reqwest;
use rss::Channel;
use std::error::Error;

pub async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://zenn.dev/ucpr/feed?include_scraps=1")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
