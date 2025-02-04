use maud::html;

use crate::web::webpages::{script, style::Styles};

use super::Webpages;
impl Webpages {
    pub fn htmx_test(_input: Option<String>) -> String {
        let res = html! {
        (script("https://unpkg.com/htmx.org@2.0.3"));
        (Styles::default());
        div{button hx-post=("/home") hx-swap=("outerHTML") {"hi"};}

        }
        .into_string();
        res
    }
}
/*
  <script src="https://unpkg.com/htmx.org@2.0.3"></script>
  <!-- have a button POST a click via AJAX -->
  <button hx-post="/clicked" hx-swap="outerHTML">
    Click Me
  </button>


*/
