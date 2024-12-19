use std::net::{Ipv4Addr, SocketAddrV4};
use tiny_http::Server;
mod web;
use web::web_server;
//TODO: use htmx instead of js
//maud
//impl core::ops::Add <- thing
fn main() {
    println!("Running");
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    web_server(server);
}
