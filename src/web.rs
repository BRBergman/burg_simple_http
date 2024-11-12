use home::home;
use maud::{html, PreEscaped};
use std::io::Cursor;
use std::{path::PathBuf, vec};
use tiny_http::Response;

pub mod css;
pub mod home;
fn guh() -> PreEscaped<String> {
    html!(h1{"guh"})
}
pub fn not_found() -> PreEscaped<String> {
    html!(h1{"Not Found"})
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

impl Default for Pages {
    fn default() -> Self {
        Pages {
            pages: vec![
                Page::new(PathBuf::from("home"), home()),
                Page::new(PathBuf::from("buh/guh"), guh()),
            ],
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
                Err(_) => match Pages::default().pages.iter().find(|&x| x.path == self) {
                    Some(x) => x.page.clone(),
                    None => not_found(),
                }
                .into_string()
                .into(),
            },
        })
    }
}
