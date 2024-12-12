use super::{home, home2, htmx::htmx_test, not_found};
use std::path::PathBuf;
use tiny_http::Response;
#[deny(unused)] //so that if it errors i know i need to put another in the from
pub enum Page {
    None,
    Home,
    Home2,
    HtmxTest,
}
impl From<PathBuf> for Page {
    fn from(value: PathBuf) -> Self {
        match value.as_os_str().to_str() {
            Some("home") => Page::Home,
            Some("home2") => Page::Home2,
            Some("htmx_test") => Page::HtmxTest,
            _ => Page::None,
        }
    }
}
impl Page {
    pub fn get(self) -> Response<std::io::Cursor<Vec<u8>>> {
        let x = match self {
            Page::None => return Response::from_data(not_found()).with_status_code(404),
            Page::Home => home(),
            Page::Home2 => home2(),
            Page::HtmxTest => htmx_test(),
        };
        Response::from_data(x).with_status_code(200)
    }
}

