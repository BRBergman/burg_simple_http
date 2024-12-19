use maud::html;
use pages::Page;
use std::io::Cursor;
use std::path::PathBuf;
use std::thread::spawn;
use tiny_http::{Response, Server};

pub mod webpages;
pub mod pages;
pub mod web_addons;

fn dir_not_found() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(
        html! {
        h1{"Error 500"}
        p{"Directory Error! ENV folder not found!"}}
        .into_string(),
    )
    .with_status_code(500)
}

pub trait ToWebResponse {
    fn to_web_response(&self) -> Response<Cursor<Vec<u8>>>;
}
impl ToWebResponse for PathBuf {
    fn to_web_response(&self) -> Response<Cursor<Vec<u8>>> {
        let env = match std::env::current_dir() {
            Ok(x) => x,
            Err(err) => {
                println!("finding local directory error: {}", err);
                return dir_not_found();
            }
        }
        .join("website");
        match std::fs::read(env.join(&self)) {
            Err(_) => match std::fs::read(env.join(&self).join("index.html")) {
                Err(_) => Page::get(self),
                Ok(x) => Response::from_data(x).with_status_code(200),
            },
            Ok(x) => Response::from_data(x).with_status_code(200),
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
