use std::{path::PathBuf, vec};

use home::home;
use maud::{html, PreEscaped};

pub mod home;

pub fn not_found() -> PreEscaped<String> {
    html!( {h1{"Not Found"}})
}
pub struct Pages {
    pages: Vec<Page>,
}
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
    pub fn get_page(self, path: PathBuf) -> PreEscaped<String> {
        for page in self.pages {
            println!("{:?}|{:?}", page.path, path);
            if page.path == path {
                return page.page;
            }
        }
        return not_found();
    }
    pub fn default() -> Self {
        Pages {
            pages: vec![Page::new(PathBuf::from("home"), home())],
        }
    }
}
