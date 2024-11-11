use maud::{html,PreEscaped};

pub fn stylesheet(path:&str)-> PreEscaped<String>{
    html!{
        link rel="stylesheet" href=(path);
    }
}