use serde::{Deserialize, Serialize};
use toml;

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,

    pub profile: Profile,
    pub header: Option<Header>,
    pub footer: Footer,
    pub rss: Rss,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderLink {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub title: String,
    pub links: Vec<HeaderLink>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Footer {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rss {
    pub urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub icon_url: String,
    pub introduction: String,
}

pub fn from_file(path: PathBuf) -> Result<Config, toml::de::Error> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    toml::from_str(&contents)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            title: "Default Title".to_string(),
            description: "Default Description".to_string(),
            header: Some(Header {
                title: "Default Header Title".to_string(),
                links: vec![
                    HeaderLink {
                        name: "Home".to_string(),
                        url: "/".to_string(),
                    },
                    HeaderLink {
                        name: "About".to_string(),
                        url: "/about".to_string(),
                    },
                    HeaderLink {
                        name: "Articles".to_string(),
                        url: "/articles".to_string(),
                    },
                ],
            }),
            profile: Profile {
                name: "Default Name".to_string(),
                icon_url: "https://via.placeholder.com/200".to_string(),
                introduction: "私はフロントエンドおよびバックエンドの開発に10年以上の経験があります。
              JavaScript、React、Node.jsを主に使用しています。新しいプロジェクトに挑戦することが大好きで、
              常に最新の技術を学び続けています。".to_string(),
            },
            footer: Footer {
                name: "John Akiyama".to_string(),
            },
            rss: Rss {
                urls: vec![
                    "https://zenn.dev/ucpr/feed?include_scraps=1".to_string(),
                    "https://ucpr.hatenablog.com/rss".to_string(),
                ],
            },
        }
    }
}

impl Config {
    pub fn to_file(&self, path: PathBuf) -> Result<(), ()> {
        let toml = toml::to_string(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        Ok(())
    }
}
