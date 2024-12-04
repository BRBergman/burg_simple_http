use home::*;
use htmx::htmx_test;
use maud::{html, PreEscaped};
use std::io::Cursor;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};
use std::thread::spawn;
use std::{path::PathBuf, vec};
use tiny_http::{Response, Server};
pub mod home;
pub mod htmx;
pub mod web_addons;

fn not_found() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(html! {h1{"Not Found"}}.into_string())
}
fn dir_not_found() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(
        html! {
        h1{"Directory Error! ENV folder not found!"}
        p{"dont worry, this is a problem on the server, not the user"}}
        .into_string(),
    )
}
pub struct PageRoot {
    pages: Vec<Page>,
}
struct Page {
    path: PathBuf,
    name: String,
    page: Vec<u8>,
}
impl Page {
    fn new<P, N>(path: P, name: N, page: Vec<u8>) -> Page
    where
        P: Into<PathBuf>,
        N: ToString,
    {
        Page {
            path: path.into(),
            name: name.to_string(),
            page,
        }
    }
}
static LIST: LazyLock<PageRoot> = std::sync::LazyLock::new(|| PageRoot::list());

impl PageRoot {
    fn list() -> Self {
        PageRoot {
            pages: vec![
                Page::new("home", "Home Page", home().into()),
                Page::new("home2", "Home Page 2", home2().into()),
                Page::new("htmx_test", "Htmx Test Page", htmx_test().into()),
            ],
        }
    }
    fn get_page(&self, path: PathBuf) -> Option<&Page> {
        let x = self;
        x.pages.iter().find(|&x| x.path == path)
    }
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
                Err(_) => match LIST.get_page(self) {
                    Some(x) => Response::from_data(x.page.clone()),
                    None => not_found(),
                },
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
