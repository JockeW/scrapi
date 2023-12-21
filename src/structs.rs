use crate::enums::Presentation;

pub struct Scrape {
    pub name: String,
    pub url: String,
    pub selectors: Vec<String>,
    pub keys: Vec<String>,
    pub attributes: Option<Vec<String>>,
    pub prefixes: Option<Vec<String>>,
    pub suffixes: Option<Vec<String>>,
    pub title: Option<String>,
    pub presentation: Option<Presentation>,
    pub export: Option<String>
}