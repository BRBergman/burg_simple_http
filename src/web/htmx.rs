use maud::html;

use crate::web::web_addons::{script, stylesheet};

use super::pages::Test;
impl Test {
    pub fn htmx_test() -> String {
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
