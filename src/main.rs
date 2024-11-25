use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::thread::spawn;
use tiny_http::Server;
mod web;
mod websockets;
use web::ToWebResponse;
fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    spawn(move || http_server(server));
    println!("press enter to close");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
}
fn http_server(server: Server) {
    server
        .incoming_requests()
        .into_iter()
        .enumerate()
        .for_each(|(i, x)| {
            let url = PathBuf::from(x.url().trim_start_matches('/'));
            println!("Fetch {} | Url: {}", i, url.display());
            x.respond(url.to_web_response()).unwrap()
        });
}
