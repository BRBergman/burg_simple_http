use home::*;
use maud::html;
use pages::Page;
use std::io::Cursor;
use std::thread::spawn;
use std::path::PathBuf;
use tiny_http::{Response, Server};
pub mod home;
pub mod htmx;
pub mod web_addons;
pub mod pages;

pub fn not_found() -> String {
    html! {h1{"Not Found"}}.into_string()
}
fn dir_not_found() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(
        html! {
        h1{"Directory Error! ENV folder not found!"}
        p{"dont worry, this is a problem on the server, not the user"}}
        .into_string(),
    )
}

pub trait ToWebResponse {
    fn to_web_response(self) -> Response<Cursor<Vec<u8>>>;
}
impl ToWebResponse for PathBuf {
    fn to_web_response(self) -> Response<Cursor<Vec<u8>>> {
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
                Err(_) => Response::from_data( Page::from(self).get()),
                Ok(x) => Response::from_data(x),
            },
            Ok(x) => Response::from_data(x),
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
