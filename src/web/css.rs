use maud::{html,PreEscaped};

#[inline]
pub fn stylesheet(path:&str)-> PreEscaped<String>{
    html!{
        link rel="stylesheet" href=(path);
    }
}