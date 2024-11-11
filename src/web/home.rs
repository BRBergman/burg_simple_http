
use maud::{html, PreEscaped, DOCTYPE};

use crate::web::css::stylesheet;
pub fn home() -> PreEscaped<String> {
    let i = 0;
    html! {
            (DOCTYPE)
            html{
            head{
                //link rel="stylesheet" href="index.css";
                (stylesheet("index.css"))
                title{"home"}
            }
            body{
                h1{"Home Page!"}
                div{button href=("/")  { {(i)}}}
                div{p1{"guh"}}
                div{"hi"}
            
            }
        }
    }
}
