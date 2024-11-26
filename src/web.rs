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
pub struct Pages {
    pages: Vec<Page>,
}
struct Page {
    path: PathBuf,
    page: tiny_http::Response<Cursor<Vec<u8>>>,
}
impl Page {
    fn new(path: PathBuf, page: tiny_http::Response<Cursor<Vec<u8>>>) -> Page {
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
        match std::fs::read(env.join(&self)) {
            Ok(x) => Response::from_data(x),
            Err(_) => match std::fs::read(env.join(&self).join("index.html")) {
                Ok(x) => Response::from_data(x),
                Err(_) => match Pages::get(port).pages.into_iter().find(|x| x.path == self) {
                    Some(x) => x.page,
                    None => Response::from_data(not_found().into_string()),
                },
            },
        }
    }
}
