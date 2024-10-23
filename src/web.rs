use blog::blog;
use home::home;
use maud::html;

pub mod home;
pub mod blog;

pub fn not_found() -> String
{
    html!( {h1{"Not Found"}}).into_string()
}

// make better where we have a vector of the domain and we have a data structure of folders or something
pub fn site_from(value: Vec<&str>) -> String {
    println!("{:?}",value);
        match value.last().unwrap().parse::<i32>() {
            Ok(x) => {
                match value[value.len()-2]{
                    "blog" => blog(x),
                    _ => not_found(),
                }
            },//we end in a number
            Err(_) =>{
                match value[value.len()-1]{
                    "home" => home(),
                    _ => not_found()
                }
            },
        }
}
