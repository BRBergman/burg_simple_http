use maud::html;
use webpages::Webpages;
mod webpages;
use std::path::PathBuf;
use tiny_http::Response;

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
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
            pub fn select(input:&str,data: Option<String>) -> Option<String> {
                match input {
                    $(stringify!($variant) => Some(Webpages::$variant(data))),*
                    ,"" | "/" => Some(Webpages::index(data)),
                    _ => None,
                }
            }
            #[allow(dead_code)]
            pub fn select_from_num(input:&i32,data: Option<String>) -> Option<String> {
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
impl DestructedURL {
    pub fn web_response(&self) -> Response<std::io::Cursor<Vec<u8>>> {
        let env = PathBuf::from("./website");
        if let Some(x) = self
            .path
            .to_str()
            .map_or(None, |x| Page::select(x, self.extra_data.clone()))
        {
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
#[derive(Debug, Clone)]
pub struct DestructedURL {
    pub path: PathBuf,
    pub extra_data: Option<String>,
}
impl DestructedURL {
    pub fn new<T: ToString>(path: T) -> Self {
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
