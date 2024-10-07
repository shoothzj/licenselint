use crate::license::License;

pub struct Config {
    pub license: License,
    pub allowed_authors: Vec<String>,
    pub formatted_author: String,
    pub formatted_year: String,
}

impl Config {
    pub fn new_from_author(license: License, author: String, formatted_year: String) -> Self {
        Config {
            license,
            allowed_authors: vec![author.clone()],
            formatted_author: author,
            formatted_year,
        }
    }

    pub fn add_allowed_author(&mut self, author: String) {
        self.allowed_authors.push(author);
    }
}
