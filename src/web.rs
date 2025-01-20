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
impl ToWebResponse for DestructedURL {
    fn to_web_response(&self) -> Response<Cursor<Vec<u8>>> {
        let env = match std::env::current_dir() {
            Ok(x) => x,
            Err(err) => {
                println!("finding local directory error: {}", err);
                return dir_not_found();
            }
        }
        .join("website");
        let pth = env.join(&self.path);
        match Page::get(&self.path, self.extra_data.clone()) {
            Some(x) => x,
            None => match std::fs::read(&pth) {
                Ok(x) => Response::from_data(x),
                Err(_) => match std::fs::read(pth.join("index.html")) {
                    Ok(x) => Response::from_data(x),
                    Err(_) => Response::from_data(html! {h1{"Not Found"}}.into_string()),
                },
            },
        }
    }
}
pub fn web_server(server: &Server) {
    let mut spawns = Vec::new();
    for x in server.incoming_requests().into_iter() {
        let url = DestructedURL::new(x.url());
        if let Some(_x @ "end") = &url.extra_data.as_deref() {
            server.unblock();
        }

        println!("{}", url.clone());
        spawns.push(spawn(move || x.respond(url.to_web_response()).unwrap()));
    }
    for spawn in spawns {
        println!("joining: {:?}", spawn.thread().id());
        let _ = spawn.join();
    }
}
#[derive(Debug, Clone)]
struct DestructedURL {
    pub path: PathBuf,
    pub extra_data: Option<String>,
}
impl DestructedURL {
    fn new<T: ToString>(path: T) -> Self {
        let binding = path.to_string();
        let twos = binding
            .trim_matches('/')
            .splitn(2, '?')
            .map(|x| x)
            .collect::<Vec<&str>>();
        let url = PathBuf::from(twos[0]);
        let data = if twos.len() > 1 {
            println!("{}", twos[1]);
            Some(twos[1].to_owned())
        } else {
            None
        };
        Self {
            path: url,
            extra_data: data,
        }
    }
}
impl std::fmt::Display for DestructedURL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ? {}",
            self.path.display(),
            self.extra_data.clone().unwrap_or_default()
        )
    }
}
