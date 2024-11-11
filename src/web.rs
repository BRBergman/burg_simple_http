use std::{path::PathBuf, vec};
use home::home;
use maud::{html, PreEscaped};
use std::io::Cursor;
use tiny_http::Response;

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
        for page in &self.pages {
            if page.path == path {
                return page.page.clone();
            }
        }
        return not_found();
    }
}
impl Default for Pages {
    fn default() -> Self {
        Pages {
            pages: vec![Page::new(PathBuf::from("home"), home())],
        }
    }
}
pub trait ToResponse {
    fn to_response(self) -> Response<Cursor<Vec<u8>>>;
}
impl ToResponse for PathBuf {
    fn to_response(self) -> Response<Cursor<Vec<u8>>> {
        let env = std::env::current_dir().unwrap().join("website");
        Response::from_data(match std::fs::read(env.join(&self)) {
            Ok(x) => x,
            Err(_) => match std::fs::read(env.join(&self).join("index.html")) {
                Ok(x) => x,
                Err(_) => Pages::default().get_page(self).into_string().into(),
            },
        })
    }
}
