use maud::html;
use pages::Page;
use std::io::Cursor;
use std::path::PathBuf;
use std::thread::spawn;
use tiny_http::{Response, Server};

pub mod pages;
pub mod webpages;

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
    fn to_web_response(&self) -> Option<Response<Cursor<Vec<u8>>>>;
}
impl ToWebResponse for PathBuf {
    fn to_web_response(&self) -> Option<Response<Cursor<Vec<u8>>>> {
        if self == &PathBuf::from("exit") {
            return None;
        };
        let env = match std::env::current_dir() {
            Ok(x) => x,
            Err(err) => {
                println!("finding local directory error: {}", err);
                return Some(dir_not_found());
            }
        }
        .join("website");
        match std::fs::read(env.join(&self)) {
            Err(_) => match std::fs::read(env.join(&self).join("in dex.html")) {
                Err(_) => Some(Page::get(self)),
                Ok(x) => Some(Response::from_data(x).with_status_code(200)),
            },
            Ok(x) => Some(Response::from_data(x).with_status_code(200)),
        }
    }
}
pub fn web_server(server: Server) -> Option<()> {
    let mut spawns = Vec::new();
    for x in server.incoming_requests().into_iter() {
        let url = PathBuf::from(x.url().trim_start_matches('/'));
        println!("Url: {}", url.display());
        let real_url = match url.to_web_response() {
            Some(y) => y,
            None => {
                break;
            }
        };
        spawns.push(spawn(move || x.respond(real_url).unwrap()));
    }
    for spawn in spawns {
        let _ = spawn.join();
    }
    Some(())
}
