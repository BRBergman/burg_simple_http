use std::net::{Ipv4Addr, SocketAddrV4};
use tiny_http::Server;
mod web;
use web::web_server;
fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    web_server(server);
}
