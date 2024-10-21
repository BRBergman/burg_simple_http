use std::fs::File;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::{Path, PathBuf};
use tiny_http::{Response, Server};
use web::site_from;
mod result_path;
mod web;
//https://doc.rust-lang.org/std/keyword.break.html
//this is really cool^
fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    for request in server.incoming_requests() {
        let path = String::from(request.url().trim_start_matches('/'));
        let path_full = std::env::current_dir()
            .unwrap()
            .join(Path::new("web"))
            .join(Path::new(&path));

        println!("{}", path);
        if path_full.is_file() {
            let _ = request.respond(Response::from_file(File::open(path_full).unwrap()));
        } else {
            match File::open(path_full.join(PathBuf::from("index.html"))) {
                Ok(x) => {
                    let _ = request.respond(Response::from_file(x));
                }
                Err(_) => {
                    let iter: Vec<&str> = Path::new(&path)
                        .iter()
                        .map(|x| x.to_str().unwrap())
                        .collect();
                    println!("g");
                    let _ = request.respond(Response::from_data(site_from(iter)));
                    //request.respond(Response::from_string(file_404_page));
                }
            };
        }
    }
}
