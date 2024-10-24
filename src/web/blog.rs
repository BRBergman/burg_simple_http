use maud::{html, PreEscaped};

pub fn blog(page: i32)-> String{
    let display_page =match page{
        1 => html!{
            div{"10/23"}
            div{"got sort of working!"}
        },
        2 => html!{
            div{"10/24"}
            div{"i dont understand"}
        },

       _ => Blog::blog_home()
    };
    html!(
        html{
            head{}
            body{
                h1{"blog"}
                p1{"Page "(page)}
                div{(display_page)}
            }
        }
    ).into_string()
}
struct Blog{}

impl Blog{
    fn blog_home()-> PreEscaped<String>{
        html!(
            {"welcome to the burgblog!"

        })
    }
}
