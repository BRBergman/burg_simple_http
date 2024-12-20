pub mod blog;
pub mod home;
pub mod htmx;
pub struct Webpages;

use maud::{html, PreEscaped};

#[inline]
pub fn stylesheet(path: &str) -> PreEscaped<String> {
    html! {
        link rel="stylesheet" href=(path);
    }
}
//<link rel="icon" href="/favicon.png">
pub fn icon(path: &str) -> PreEscaped<String> {
    html! {
        link rel="icon" href=(path);
    }
}
pub fn title(name: &str) -> PreEscaped<String> {
    html! {title{(name)}}
}
pub fn script(script_file: &str) -> PreEscaped<String> {
    PreEscaped(format!(r#"<script src="{}"></script>"#, script_file))
}
