use maud::html;
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};
use tiny_http::Response;

use super::webpages::Webpages;

macro_rules! enum_page {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        #[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
        pub enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
            pub const HM: LazyLock<HashMap<Page, String>> = LazyLock::new(|| {
                HashMap::from([
                    //(Page::Home, Webpages::Home()),
                    $(($name::$variant,Webpages::$variant()) ),*
                ])
            });
        }
    };
}
//for each on of these we have to implement a webpages one
enum_page! {
    enum Page {
        Home = 0,
        Index = 1,
        HtmxTest = 2,
        Blog = 3,
    }
}

impl Page {
    fn not_found() -> String {
        html! {h1{"Not Found"}}.into_string()
    }
    pub fn get(page_dir: &PathBuf) -> Response<std::io::Cursor<Vec<u8>>> {
        match Self::HM.iter().find(|(&z, _)| {
            (Some(z.name().to_lowercase()) == page_dir.try_into_string())
                || (page_dir == &PathBuf::new() && z == Page::Index)
        }) {
            Some((_, x)) => Response::from_data(x.clone()).with_status_code(200),
            None => Response::from_data(Self::not_found()).with_status_code(404),
        }
    }
}

trait TryIntoString {
    fn try_into_string(&self) -> Option<String>;
}
impl TryIntoString for PathBuf {
    fn try_into_string(&self) -> Option<String> {
        Some(self.to_str()?.trim_end_matches('/').to_string())
    }
}
