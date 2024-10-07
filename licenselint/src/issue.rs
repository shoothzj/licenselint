pub struct Issue {
    pub filename: String,
}

impl Issue {
    pub fn new(filename: &str) -> Self {
        Issue {
            filename: filename.to_string(),
        }
    }
}
