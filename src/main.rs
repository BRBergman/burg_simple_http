use std::net::{Ipv4Addr, SocketAddrV4};
use std::{fs::File, path::Path};
use tiny_http::{Response, Server};

mod result_path;
use result_path::{ResultPath, ToResultPath};

fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    println!("running");
    for request in server.incoming_requests() {
        let file_404_page = Response::from_string("404 :(");
        let path_full = if cfg!(windows) {
            Path::new("./")
        } else {
            Path::new("../")
        }
        .join(Path::new("web"))
        .join(Path::new(request.url().trim_start_matches('/')));
        println!("{:?}", path_full);
        match path_full.to_result_path() {
            ResultPath::File(file) => {
                let _ = request.respond(Response::from_file(file));
            }
            ResultPath::Directory(path) => {
                match File::open(path.join("index.html")) {
                    Ok(file) => {
                        let _ = request.respond(Response::from_file(file));
                    }
                    Err(_) => {
                        //this is what happens if you try to open a folder that exists, but has no index.html
                        let _ = request.respond(file_404_page);
                    }
                }
            }
            ResultPath::Err(_) => {
                let _ = request.respond(file_404_page);
            }
        };
    }
}
