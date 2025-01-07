use maud::{html, PreEscaped, DOCTYPE};
use std::fmt::Display;

use crate::web::webpages::{icon, stylesheet, title};

use super::Webpages;
pub fn blogvec() -> Vec<Blog> {
    vec![
        //add on to end
        Blog::new(Date::from((10, 4, 2024)), " why is everything so weawerewasrw "),
        Blog::new(Date::from((10, 5, 2024)), " i can only do so much reading before the words start to mesh together, maybe i need glasses "),
        Blog::new(Date::from((10, 29, 2024)), "*hacker voice* im in </br>(accepted to a collage) "),
        Blog::new(Date::from((10, 31, 2024)), " halloween happened too fast i didn't have time to get a costume :( "),
        Blog::new(Date::from((12, 20, 2024)), "write it in rust"),
        Blog::new(Date::from((12, 20, 2024)), "i ended up getting glasses"),
        Blog::new(Date::from((1, 6, 2025)), "bangs are peak"),
        Blog::new(Date::from((1, 6, 2025)), "take this shitass"),
    ]
}
impl Webpages {
    #[expect(non_snake_case)]
    pub fn Blog() -> String {
        let pages = blogvec();
        html! {
            (DOCTYPE)
            html{
                head{
                    (title("the burgblog"))
                    (stylesheet("/index.css"))
                    (icon("favicon.png"))
                }
                body{
                    h1 class="heading" {("the burgblog")};
                    p {a href="/" {"Back"}};
                    div class="main"{
                        div class="outerboxes" {
                            (pages.as_inner_boxes())
                        }
                    }
                }
            }
        }
        .into_string()
    }
}
#[derive(Clone)]
pub struct Blog {
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
/// mm/dd/yyyy
/// 12/25/1999
/// December 25th 1999
/// ```
/// let date = Date::from((12/25/1999));
/// ```

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
        write!(
            f,
            "{}/{}/{}",
            if self.month >= 10 {
                self.month.to_string()
            } else {
                format!("0{}", self.month)
            },
            if self.day >= 10 {
                self.day.to_string()
            } else {
                format!("0{}", self.day)
            },
            self.year
        )
    }
}

pub trait ToPreEscaped {
    fn as_inner_boxes(&self) -> PreEscaped<String>;
}
impl ToPreEscaped for Vec<Blog> {
    fn as_inner_boxes(&self) -> PreEscaped<String> {
        let mut x = self
            .iter()
            .map(|x| format!("<div  class=\"innerboxes\"> <h1>{}</h1> \n <p> {}</p>\n</div> \n <div class=\"inbetweenboxes\"> </br></div>", x.date, x.content))
            .collect::<Vec<String>>();
        x.reverse();
        PreEscaped(x.concat())
    }
}
impl ToPreEscaped for Blog {
    fn as_inner_boxes(&self) -> PreEscaped<String> {
        PreEscaped(format!(
            "<div  class=\"innerboxes\"> <h3>Latest Blog: {}</h3> \n <p> {}</p>\n</div> ",
            self.date, self.content
        ))
    }
}
