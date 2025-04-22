use std::{
    net::{Ipv4Addr, SocketAddrV4},
    thread::spawn,
};
//test
use tiny_http::Server;
mod web;
use web::{DestructedURL, Page};
mod export_page;
//TODO: use htmx instead of js
//maud
//impl core::ops::Add <- thing
fn main() {
    
    export_page::export(Page::index, None);
    let port = 8000;
    println!("Running");
    let Ok(server) = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)) else {
        panic!("failed to make server")
    };
    let mut spawns = Vec::new();
    for x in server.incoming_requests().into_iter() {
        spawns.push({
            let url = DestructedURL::new(x.url());
            println!("{}", url);
            if let Some(_x @ "end") = url.extra_data.as_deref() {
                server.unblock();
                break;
            }
            spawn(move || x.respond(url.web_response()))
        });
    }
    for spawn in spawns {
        println!("joining: {:?}", spawn.thread().id());
        let _ = spawn.join();
    }
}
