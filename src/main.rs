use std::net::{Ipv4Addr, SocketAddrV4};
use std::{fs::File, path::Path};
use tiny_http::{Response, Server};

fn main() {
    let ip = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000);
    let server = Server::http(ip).unwrap();

    for request in server.incoming_requests() {
       
        /*println!(
            "received request! method: {:?}\n, url: {:?}\n, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );*/
        
        let path_served = Path::new(request.url().split_once('/').unwrap().1);
        let x = Path::new("../web/").join(path_served);

        //make that i just give the name and it adds .html or something
        let serve_file: File;
        if x.exists() {
            match path_served.extension() {
                Some(_) => {
                    
                    let _ = request.respond(Response::from_file(File::open(x).unwrap()));
                }
                None => {
                    let pathx =
                        File::open(Path::new("../web/").join(path_served).join("index.html"))
                            .unwrap();
                    let _ = request.respond(Response::from_file(pathx));
                }
            };
        } else {
            let index_file = File::open(Path::new("../web/index.html")).unwrap();
            let _ = request.respond(Response::from_file(index_file));
        }
    }
}
