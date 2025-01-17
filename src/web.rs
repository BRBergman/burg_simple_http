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
    fn to_web_response(&self) -> Response<Cursor<Vec<u8>>>;
}
impl ToWebResponse for PathBuf {
    fn to_web_response(&self) -> Response<Cursor<Vec<u8>>> {
        match Page::get(self) {
            Some(x) => x,
            None => {
                let env = match std::env::current_dir() {
                    Ok(x) => x,
                    Err(err) => {
                        println!("finding local directory error: {}", err);
                        return dir_not_found();
                    }
                }
                .join("website");
                let pth = &env.join(&self);
                match std::fs::read(pth) {
                    Ok(x) => Response::from_data(x),
                    Err(_) => match std::fs::read(pth.join("index.html")) {
                        Ok(x) => Response::from_data(x),
                        Err(_) => Response::from_data(html! {h1{"Not Found"}}.into_string()),
                    },
                }
            }
        }
    }
}
pub fn web_server(server: &Server) {
    let mut spawns = Vec::new();
    for x in server.incoming_requests().into_iter() {
        let url = PathBuf::from(x.url().trim_matches('/'));
        println!("Url: {}", url.display());
        if url == PathBuf::from("end") {
            server.unblock()
        } else {
            spawns.push(spawn(move || {
                x.respond(url.to_web_response()).unwrap()
            }))
        }
    }
    for spawn in spawns {
        println!("Joining: {:?}", spawn.thread().id());
        let _ = spawn.join().unwrap();
    }
}
