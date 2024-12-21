use std::fmt::Display;

use maud::{html, PreEscaped, DOCTYPE};

use crate::web::webpages::{icon, stylesheet, title};

use super::Webpages;

impl Webpages {
    #[expect(non_snake_case)]
    pub fn Blog() -> String {
        let pages = vec![
            //add on to end
            Blog::new(Date::from((12, 20, 2024)), "write it in rust"),
            Blog::new(Date::from((12, 20, 2024)), "i need two things"),
        ];
        html! {
            (DOCTYPE)
            html{
                head{
                    (title("the burgblog"))
                        (stylesheet("index.css"))
                        (icon("favicon.png"))
                }
                body{
                    h1 class="heading" {("The Burgblog")};
                    div class="main"{
                        div class="outerboxes" {
                            (pages.to_pre_escaped())
                        }
                    }
                }
            }
        }
        .into_string()
    }
}
struct Blog {
    date: Date,
    content: String,
}
impl Blog {
    fn new<T: ToString>(date: Date, content: T) -> Self {
        Blog {
            date,
            content: content.to_string(),
        }
    }
}
#[derive(Clone, Copy)]
struct Date {
    pub month: u8,
    pub day: u8,
    pub year: u16,
}
impl From<(u8, u8, u16)> for Date {
    fn from((month, day, year): (u8, u8, u16)) -> Self {
        Date { month, day, year }
    }
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.month, self.day, self.year)
    }
}

trait ToPreEscaped {
    fn to_pre_escaped(self) -> PreEscaped<String>;
}
impl ToPreEscaped for Vec<Blog> {
    fn to_pre_escaped(self) -> PreEscaped<String> {
        let mut x = self
            .iter()
            .map(|x| format!("<div  class=\"innerboxes\"> <h1>{}</h1> \n <p> {}</p>\n</div> \n <div class=\"inbetweenboxes\"> </br></div>", x.date, x.content))
            .collect::<Vec<String>>();
        x.reverse();
        PreEscaped(x.concat())
    }
}
