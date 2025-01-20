use maud::html;

use crate::web::webpages::{script, stylesheet};

use super::Webpages;
impl Webpages {
    #[expect(non_snake_case)]
    pub fn htmx_test(input: Option<String>) -> String {
        let res = html! {
        (script("https://unpkg.com/htmx.org@2.0.3"));
        (stylesheet("index.css"));
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
