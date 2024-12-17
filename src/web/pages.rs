use maud::html;
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};
use tiny_http::Response;
pub struct Webpages;
macro_rules! enum_str {
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
        }
    };
}

enum_str! {
    enum Page {
        Home = 0x00,
        Home2 = 0x01,
        HtmxTest =0x02,
    }
}

impl Page {
    pub const HM: LazyLock<HashMap<Page, String>> = LazyLock::new(|| {
        HashMap::from([
            (Page::Home, Webpages::Home()),
            (Page::Home2, Webpages::Home2()),
            (Page::HtmxTest, Webpages::HtmxTest()),
        ])
    });
    fn not_found() -> String {
        html! {h1{"Not Found"}}.into_string()
    }
    pub fn get(page_dir: PathBuf) -> Response<std::io::Cursor<Vec<u8>>> {
        match Self::HM
            .iter()
            .find(|(&z, _)| Some(z.name().to_lowercase()) == page_dir.try_into_string())
        {
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
        Some(self.to_str()?.to_string())
    }
}
