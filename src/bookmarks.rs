use std::collections::HashMap;

pub type Bookmarks = HashMap<String, String>;

#[derive(Debug)]
pub struct Bookmark {
    pub alias: String,
    pub path: String,
}

impl Bookmark {
    pub fn new(alias: String, path: String) -> Self {
        Bookmark { alias, path }
    }
}

