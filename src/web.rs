use std::{path::PathBuf, vec};

use home::home;
use maud::{html, PreEscaped};

pub mod css;
pub mod home;

pub fn not_found() -> PreEscaped<String> {
    html!( {h1{"Not Found"}})
}
#[derive(Debug, Clone)]
pub struct Pages {
    pages: Vec<Page>,
}
#[derive(Debug, Clone)]
struct Page {
    path: PathBuf,
    page: PreEscaped<String>,
}
impl Page {
    fn new(path: PathBuf, page: PreEscaped<String>) -> Page {
        Page { path, page }
    }
}
impl Pages {
    pub fn get_page(&self, path: PathBuf) -> PreEscaped<String> {
        println!("{}",path.display());
        for page in &self.pages {
            if page.path == path {
                return page.page.clone();
            }
        }
        return not_found();
    }
}
impl Default for Pages{
    fn default() -> Self {
        Pages {
            pages: vec![Page::new(PathBuf::from("home"), home())],
        }
    }
}
