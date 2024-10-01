use std::net::{Ipv4Addr, SocketAddrV4};
use std::{fs::File, path::Path};
use tiny_http::{Response, Server};

fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();

    for request in server.incoming_requests() {
        let file_404_page = Response::from_string("404 :(");
        let path_served = Path::new(request.url().split_once('/').unwrap().1);
        let folder_default = Path::new("../web/");
        let path_full = folder_default.join(path_served);
        //make so that it adds the .htm so i dont have to
        //make that i just give the name and it adds .html or something
        if path_full.exists() {
            match path_served.extension() {
                Some(_) => {
                    let _ = request.respond(Response::from_file(File::open(path_full).unwrap()));
                }
                None => {
                    match File::open(path_full.join("index.html")) {
                        Ok(x) => {
                            let _ = request.respond(Response::from_file(x));
                        }
                        Err(_) => {
                            //this is what happens if you try to open a folder that exists, but has no index.html
                            let _ = request.respond(file_404_page);
                        }
                    }
                }
            };
        } else {
            let _ = request.respond(file_404_page);
        }
    }
}
