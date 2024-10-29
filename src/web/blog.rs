use maud::{html, PreEscaped};

pub fn blog(page: i32)-> String{
    html!(
        html{
            head{}
            body{
                h1{"blog"}
                p1{"Page "(page)}
                div{
                    @match page{
                        1 => {
                            div{"10/23"}
                            div{"got sort of working!"}
                        },
                        2 =>{
                            div{"10/24"}
                            div{"i dont understand"} 
                        },

                        _ => {"welcome to the burgblog"}
                    }
                }
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
