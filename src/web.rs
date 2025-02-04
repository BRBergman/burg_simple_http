use maud::html;
use webpages::Webpages;
mod webpages;
use std::path::PathBuf;
use std::thread::spawn;
use tiny_http::{Response, Server};

macro_rules! enum_page {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        #[allow(unused)]
        #[expect(non_camel_case_types)]
        #[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
        pub enum $name {
            $($variant = $val),*
        }

        impl $name {
            #[allow(unused)]
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
            fn select(input:&str,data: Option<String>) -> Option<String> {
                match input {
                    $(stringify!($variant) => Some(Webpages::$variant(data))),*
                    ,"" | "/" => Some(Webpages::index(data)),
                    _ => None,
                }
            }
            #[allow(dead_code)]
            fn select_from_num(input:&i32,data: Option<String>) -> Option<String> {
                match input {
                    $($val => Some(Webpages::$variant(data))),*,
                    _ => None,
                }
            }
        }
    };
}
//for each on of these we have to implement a webpages one
enum_page! {
    enum Page {
        home = 0,
        index = 1,
        htmx_test = 2,
        blog = 3,
        default_style = 4,
    }
}

pub type DataResponse = Response<std::io::Cursor<Vec<u8>>>;

fn dir_not_found() -> DataResponse {
    Response::from_data(
        html! {
        h1{"Error 500"}
        p{"Directory Error! ENV folder not found!"}}
        .into_string(),
    )
    .with_status_code(500)
}
impl Page {
    pub fn get(input: &DestructedURL) -> Option<String> {
        input
            .path
            .to_str()
            .map(|x| Page::select(x, input.extra_data.clone()))?
    }
}

pub trait ToWebResponse {
    fn to_web_response(&self) -> DataResponse;
}
impl ToWebResponse for DestructedURL {
    fn to_web_response(&self) -> DataResponse {
        let env = match std::env::current_dir() {
            Ok(x) => x,
            Err(err) => {
                println!("finding local directory error: {}", err);
                return dir_not_found();
            }
        }
        .join("website");
        if let Some(x) = Page::get(self) {
            Response::from_data(x)
        } else if let Ok(x) = std::fs::read(env.join(&self.path)) {
            Response::from_data(x)
        } else if let Ok(x) = std::fs::read(env.join(&self.path).join("index.html")) {
            Response::from_data(x)
        } else {
            Response::from_data(html! {h1{"Not Found"}}.into_string())
        }
    }
}
pub fn web_server(server: &Server) {
    let mut spawns = Vec::new();
    for x in server.incoming_requests().into_iter() {
        let url = DestructedURL::new(x.url());
        if let Some(_x @ "end") = url.extra_data.as_deref() {
            server.unblock();
        }
        println!("{}", url);
        spawns.push(spawn(move || x.respond(url.to_web_response()).unwrap()));
    }
    for spawn in spawns {
        println!("joining: {:?}", spawn.thread().id());
        let _ = spawn.join();
    }
}
#[derive(Debug, Clone)]
pub struct DestructedURL {
    path: PathBuf,
    extra_data: Option<String>,
}
impl DestructedURL {
    fn new<T: ToString>(path: T) -> Self {
        let binding = path.to_string();
        let twos = binding
            .trim_matches('/')
            .splitn(2, '?')
            .collect::<Vec<&str>>();
        let url = PathBuf::from(twos[0]);
        let data = if twos.len() > 1 {
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
            "{}?{}",
            self.path.display(),
            self.extra_data.clone().unwrap_or_default()
        )
    }
}
