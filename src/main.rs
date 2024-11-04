use std::fs::File;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use tiny_http::{Response, Server};
use web::Pages;
mod web;
//https://doc.rust-lang.org/std/keyword.break.html
//this is really cool^
fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    for request in server.incoming_requests() {
        let path = PathBuf::from(String::from(request.url().trim_start_matches('/')));
        let path_full = std::env::current_dir()
            .unwrap()
            .join("website")
            .join(&path);
        println!("{:?}", &path_full); //some reason files arent workign
        if path_full.is_file() {
            let _ = request.respond(Response::from_file(File::open(&path_full).unwrap()));
        } else {
            match File::open(path_full.join("index.html")) {
                Ok(x) => {
                    let _ = request.respond(Response::from_file(x));
                }
                Err(_) => {
                    //let iter: Vec<&str> = path.split('/').collect();
                    //let _ = request.respond(Response::from_data(site_from_better(iter)));
                    let _ = request.respond(Response::from_data(Pages::default().get_page(path).into_string()));
                }
            };
        }
    }
}
