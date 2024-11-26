use maud::{html, DOCTYPE};

use crate::web::css::stylesheet;
#[inline]
pub fn home() -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let i = 0;
    tiny_http::Response::from_data(
        html! {
                (DOCTYPE)
                html{
                head{
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
        .into_string(),
    )
}
