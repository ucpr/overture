use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Source {
    Unknown,
    Zenn,
    ZennScraps,
    HatenaBlog,
}

impl ToString for Source {
    fn to_string(&self) -> String {
        match self {
            Source::Unknown => "Unknown".to_string(),
            Source::Zenn => "Zenn".to_string(),
            Source::ZennScraps => "Zenn Scraps".to_string(),
            Source::HatenaBlog => "HatenaBlog".to_string(),
        }
    }
}

pub struct Article {
}
