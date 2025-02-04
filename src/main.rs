use std::{net::{Ipv4Addr, SocketAddrV4}, thread::spawn};
use tiny_http::Server;
mod web;
use web::DestructedURL;
//TODO: use htmx instead of js
//maud
//impl core::ops::Add <- thing
fn main() {
    let port = 8000;
    println!("Running");
    let Ok(server) = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)) else {
        panic!("failed to make server")
    };
    let mut spawns = Vec::new();
    for x in server.incoming_requests().into_iter() {
        let url = DestructedURL::new(x.url());
        println!("{}", url);
        if let Some(_x @ "end") = url.extra_data.as_deref() {
            server.unblock();
            break;
        }
        spawns.push(spawn(move || x.respond(url.to_web_response())));
    }
    for spawn in spawns {
        println!("joining: {:?}", spawn.thread().id());
        let _ = spawn.join();
    }
}
