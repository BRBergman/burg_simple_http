use std::net::{Ipv4Addr, SocketAddrV4};
use std::{fs::File, path::Path};
use tiny_http::{Response, Server};

fn main() {
    let ip = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000);
    let server = Server::http(ip).unwrap();

    for request in server.incoming_requests() {
        /*println!(
            "Request\n url: {:?}\n, headers: {:?}",
            //request.method(),
            request.url(),
            request.headers()
        );*/
        let path_served = Path::new(request.url().split_once('/').unwrap().1);
        let x = Path::new("../web/").join(path_served);

        //make that i just give the name and it adds .html or something
        let serve_file;
        if x.exists() {
            match path_served.extension() {
                Some(_) => {
                    serve_file = Response::from_file(File::open(x).unwrap());
                }
                None => {
                    match File::open(Path::new("../web/").join(path_served).join("index.html")) {
                        Ok(x) => serve_file = Response::from_file(x),
                        Err(_) => {
                            println!("i dont know what brings you here");
                            serve_file = Response::from_file(
                                File::open(Path::new("../web/404.html")).unwrap(),
                            )
                        }
                    }
                }
            };
        } else {
            serve_file = Response::from_file(File::open(Path::new("../web/404.html")).unwrap());
        }
        let _ = request.respond(serve_file);
    }
}
