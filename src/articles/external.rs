use chrono::DateTime;
use chrono_tz::{Asia::Tokyo, Tz};

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
}
