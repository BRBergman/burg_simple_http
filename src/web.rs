use home::home;
use maud::{html, PreEscaped};
use std::io::Cursor;
use std::thread::spawn;
use std::{path::PathBuf, vec};
use tiny_http::{Response, Server};
pub mod css;
pub mod home;

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
    fn get() -> Self {
        Pages {
            pages: vec![Page::new(PathBuf::from("home"), home())],
        }
    }
}
pub trait ToWebResponse {
    fn to_web_response(self) -> Response<Cursor<Vec<u8>>>;
}
impl ToWebResponse for PathBuf {
    fn to_web_response(self) -> Response<Cursor<Vec<u8>>> {
        let env = std::env::current_dir()
            .unwrap_or({
                println!("err finding current dir");
                PathBuf::from("err")
            })
            .join("website");
        match std::fs::read(env.join(&self)) {
            Ok(x) => Response::from_data(x),
            Err(_) => match std::fs::read(env.join(&self).join("index.html")) {
                Ok(x) => Response::from_data(x),
                Err(_) => match Pages::get().pages.into_iter().find(|x| x.path == self) {
                    Some(x) => x.page,
                    None => Response::from_data(not_found().into_string()),
                },
            },
        }
    }
}
pub fn web_server(server: Server) {
    server.incoming_requests().into_iter().for_each(|x| {
        let url = PathBuf::from(x.url().trim_start_matches('/'));
        println!("Url: {}", url.display());
        spawn(move || x.respond(url.to_web_response()).unwrap());
    });
}
