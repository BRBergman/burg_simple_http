use home::home;
use maud::{html, PreEscaped};
use socket_page::socket_page;
use std::io::Cursor;
use std::{path::PathBuf, vec};
use tiny_http::Response;
pub mod css;
pub mod home;
pub mod socket_page;

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

impl Pages {
    fn get(port: u16) -> Self {
        Pages {
            pages: vec![
                Page::new(PathBuf::from("home"), home()),
                Page::new(PathBuf::from("socket_page"), socket_page(port)),
            ],
        }
    }
}
pub trait ToWebResponse {
    fn to_web_response(self, port: u16) -> Response<Cursor<Vec<u8>>>;
}
impl ToWebResponse for PathBuf {
    fn to_web_response(self, port: u16) -> Response<Cursor<Vec<u8>>> {
        let env = std::env::current_dir().unwrap().join("website");
        Response::from_data(match std::fs::read(env.join(&self)) {
            Ok(x) => x,
            Err(_) => match std::fs::read(env.join(&self).join("index.html")) {
                Ok(x) => x,
                Err(_) => match Pages::get(port).pages.iter().find(|&x| x.path == self) {
                    Some(x) => x.page.clone(),
                    None => not_found(),
                }
                .into_string()
                .into(),
            },
        })
        .with_header(
            "Content-type: text/html"
                .parse::<tiny_http::Header>()
                .unwrap(),
        )
    }
}
