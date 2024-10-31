use home::home;
use maud::html;

pub mod home;

pub fn not_found() -> String {
    html!( {h1{"Not Found"}}).into_string()
}

// make better where we have a vector of the domain and we have a data structure of folders or something
pub fn site_from_better(value: Vec<&str>) -> String {
    println!("{:?}", value);
    match value[0].parse::<i32>() {
        Ok(_x) => match value[value.len() - 1] {
           //we start with a number
            _ => not_found(),
        }, 
        Err(_) => match Page::from(*value.first().unwrap()) {
            Page::Home => home(),
            Page::NotFound => not_found(),
        },
    }
}

#[forbid(dead_code)] // so that if we add ap age we have to make that page work
enum Page {
    Home,
    NotFound,
}
impl From<&str> for Page {
    fn from(value: &str) -> Self {
        match value {
            "home" => Self::Home,

            _ => Self::NotFound
        }
        
    }
}