use std::time::SystemTime;

use maud::{html, DOCTYPE};

pub fn home() -> String {
    html! {
        
            (DOCTYPE)
            html{
            head{
                link rel="stylesheet" href="style.css";
                title{"home"}
            }
            body{
                h1{"Home Page!"}
                p1{"The time is: "((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos()))}
                div{p1{"guh"}}
                div{"hi"}
            
            }
        }
    }
    .into_string()
}
