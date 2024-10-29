
use maud::{html, DOCTYPE};

pub fn home() -> String {
    let i = 0;
    html! {
        
            (DOCTYPE)
            html{
            head{
                link rel="stylesheet" href="style.css";
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
    .into_string()
}
