/*use article::Article;
use std::fs;
use std::path;

pub struct Contents {
    sections: Vec<Section>,
}

pub struct Section {
    name: String,
    articles: Vec<Article>,
}

impl Section {
    pub fn new(name: String) -> Section {
        Section {
            name,
            articles: Vec::new(),
        }
    }
}

impl Contents {
    pub fn new() -> Contents {
        Contents {
            sections: Vec::new(),
        }
    }

    fn read_sections(self) -> &Vec<Section> {
        let mut sections = Vec::new();

        let dir = fs::read_dir("./")?;

        for item in dir.into_iter() {
            sections.push(item?.path());
        }
    }
}
*/
