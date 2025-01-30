pub mod blog;
pub mod home;
pub mod htmx;
pub mod style;
pub struct Webpages;
use maud::{html, Markup, PreEscaped};
//<link rel="icon" href="/favicon.png">
pub fn icon(path: &str) -> Markup {
    html! {
        link rel="icon" href=(path);
    }
}
pub fn title(name: &str) -> Markup {
    html! {title{(name)}}
}
pub fn script(script_file: &str) -> Markup {
    PreEscaped(format!(r#"<script src="{}"></script>"#, script_file))
}
