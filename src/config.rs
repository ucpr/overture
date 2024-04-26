use std::fs;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub url: String,

    pub profile: Profile,
    pub header: Header,
    pub footer: Footer,
    pub rss: Rss,
    pub google_analytics: Option<GoogleAnalytics>,
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
    pub external_rss_links: Vec<String>,
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub icon_url: String,
    pub introduction: String,
    pub work_experiences: Vec<WorkExperience>,
    pub certificates: Vec<Certificate>,
    pub spotify_playlist_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkExperience {
    pub company: String,
    pub active: bool,
    pub projects: Vec<WorkExperienceProject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkExperienceProject {
    pub name: String,
    pub period: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub name: String,
    pub date: String,
    pub description: Option<String>,
    pub is_expired: bool,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAnalytics {
    pub tracking_id: String,
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
            url: "https://example.com".to_string(),
            header: Header {
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
            },
            profile: Profile {
                name: "Default Name".to_string(),
                icon_url: "https://via.placeholder.com/200".to_string(),
                introduction: "私はフロントエンドおよびバックエンドの開発に10年以上の経験があります。
              JavaScript、React、Node.jsを主に使用しています。新しいプロジェクトに挑戦することが大好きで、
              常に最新の技術を学び続けています。".to_string(),
                spotify_playlist_id: Some("3RktWZ6EsWwBXIgnJOm9EM".to_string()),
                work_experiences: vec![WorkExperience {
                    company: "Default Company".to_string(),
                    active: true,
                    projects: vec![
                        WorkExperienceProject {
                            name: "Default Project".to_string(),
                            period: "2020/01 - 2020/12".to_string(),
                            description: "Default Description.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
                            tags: vec!["Default Tag".to_string()],
                        },
                    ],
                }],
                certificates: vec![Certificate {
                    name: "Default Certificate".to_string(),
                    date: "2020/12".to_string(),
                    description: Some("Default Description".to_string()),
                    is_expired: false,
                    url: Some("https://example.com".to_string()),
                }],
            },
            footer: Footer {
                name: "John Akiyama".to_string(),
            },
            rss: Rss {
                title: "Default RSS Title".to_string(),
                description: "Default RSS Description".to_string(),
                url: "https://example.com/rss".to_string(),
                external_rss_links: vec![
                    "https://zenn.dev/ucpr/feed?include_scraps=1".to_string(),
                    "https://ucpr.hatenablog.com/rss".to_string(),
                ],
            },
            google_analytics: Some(GoogleAnalytics {
                tracking_id: "UA-123456789-0".to_string(),
            }),
        }
    }
}

impl Config {
    pub fn to_file(&self, path: PathBuf) -> Result<(), ()> {
        let toml = toml::to_string(self).unwrap();
        let mut file = fs::File::create(path).unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        Ok(())
    }
}
