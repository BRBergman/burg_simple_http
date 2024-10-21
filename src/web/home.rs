use maud::{html, DOCTYPE};

pub fn home() -> String {
    html! {
            (DOCTYPE)
            html{
            head{
                title{"home"}
            }
            body{
                h1{"Home Page!"}
                p1{"guh"}
                div{"hi"}
            
            }
        }
    }
    .into_string()
}
