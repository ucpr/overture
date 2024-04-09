use markdown;
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;

pub struct Article {
    title: String,
    raw_body: String,
    ext: String,
}

impl Article {
    pub fn new(title: String, raw_body: String, ext: String) -> Article {
        Article {
            title,
            ext,
            raw_body,
        }
    }

    pub fn from_file(path: &str) -> Result<Article, std::io::Error> {
        let ext = path.split('.').last().unwrap().to_string();

        let raw_body = std::fs::read_to_string(path)?;
        let title = path.split('/').last().unwrap().to_string();
        Ok(Article::new(title, raw_body, ext))
    }

    pub fn build(&self) -> String {
        let mut body = self.raw_body.clone();
        if self.ext == "md" {
            body = markdown::to_html(&body);
        }

        format!("<h1>{}</h1><div>{}</div>", self.title, body)
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let html = self.build();
        std::fs::write(path, html)
    }

    pub fn hash(&self) -> u64 {
        const_xxh3(self.build().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const BODY: &str = "
## h2
body
";

    #[test]
    fn test_build() {
        let article = Article::new("title".to_string(), BODY.to_string(), "md".to_string());

        let want = "<h1>title</h1><div><h2>h2</h2>\n<p>body</p>\n</div>";
        assert_eq!(article.build(), want);
    }

    #[test]
    fn test_hash() {
        let article = Article::new("title".to_string(), BODY.to_string(), "md".to_string());

        let want = 10090120085267840616;
        assert_eq!(article.hash(), want);
    }
}
