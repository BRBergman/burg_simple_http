use std::net::{Ipv4Addr, SocketAddrV4};
use tiny_http::Server;
mod web;
mod websockets;
use websockets::websocket_server;
fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    websocket_server(server);
    /*spawn(move || http_server(server));
    println!("press enter to close");*/
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
}
