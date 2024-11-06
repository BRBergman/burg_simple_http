use std::fs::File;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use tiny_http::{Response, Server};
use web::Pages;
mod web;
//https://doc.rust-lang.org/std/keyword.break.html
//this is really cool^
fn main() {
    let pages = Pages::default();//gotta be a better way to do this
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    for request in server.incoming_requests() {
        let path = PathBuf::from(String::from(request.url().trim_start_matches('/')));
        let path_full = std::env::current_dir()
            .unwrap()
            .join("website")
            .join(&path);
        println!("Path: {}", &path.display()); 
        if path_full.is_file() {
            let _ = request.respond(Response::from_file(File::open(&path_full).unwrap()));
        } else {
            match File::open(path_full.join("index.html")) {
                Ok(x) => {
                    let _ = request.respond(Response::from_file(x));
                }
                Err(_) => {
                    let _ = request.respond(Response::from_data(pages.get_page(path).into_string()));
                }
            };
        }
    }
}
