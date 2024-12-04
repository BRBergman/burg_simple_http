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
pub fn title(name: &str) -> PreEscaped<String>{
    html!{title{(name)}}
}