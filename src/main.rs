
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use tiny_http::Server;
mod web;
use web::ToResponse;
//https://doc.rust-lang.org/std/keyword.break.html
//this is really cool^
fn main() {
    http_server();
}
fn http_server() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    server.incoming_requests().into_iter().for_each(|x| {
        let url = PathBuf::from(x.url().trim_start_matches('/'));
        println!("Url: {}",url.display());
        x.respond(url.to_response()).unwrap()
    });
}
