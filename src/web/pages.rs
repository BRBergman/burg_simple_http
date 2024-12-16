use maud::html;
use std::{collections::HashMap, fmt, path::PathBuf, sync::LazyLock};
use tiny_http::Response;
pub struct Home;
pub struct Test;
#[deny(unused)] //so that if it errors i know i need to put another in the from
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Home2,
    HtmxTest,
}

impl Page {
    pub const HM: LazyLock<HashMap<Page, String>> = LazyLock::new(|| {
        HashMap::from([
            (Page::Home, Home::home()),
            (Page::Home2, Home::home2()),
            (Page::HtmxTest, Test::htmx_test()),
        ])
    });
    fn not_found() -> String {
        html! {h1{"Not Found"}}.into_string()
    }
    pub fn get(page_dir: PathBuf) -> Response<std::io::Cursor<Vec<u8>>> {
        match Self::HM
            .iter()
            .find(|(&z, _)| Some(z.to_string()) == page_dir.try_into_string())
        {
            Some((_, x)) => Response::from_data(x.clone()).with_status_code(200),
            None => Response::from_data(Self::not_found()).with_status_code(404),
        }
    }
}
impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Page::Home => write!(f, "home"),
            Page::Home2 => write!(f, "home2"),
            Page::HtmxTest => write!(f, "htmx_test"),
        }
    }
}

trait TryIntoString {
    fn try_into_string(&self) -> Option<String>;
}
impl TryIntoString for PathBuf {
    fn try_into_string(&self) -> Option<String> {
        Some(self.to_str()?.to_string())
    }
}
