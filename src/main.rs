use std::io::Cursor;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use tiny_http::{Response, Server};
use web::Pages;
mod web;
//https://doc.rust-lang.org/std/keyword.break.html
//this is really cool^
fn main() {
    server();
}
fn server() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    //let path_base = std::env::current_dir().unwrap().join("website");
    server.incoming_requests().into_iter().for_each(|x| {
        let url = x.url().to_owned();
        x.respond(
            //path_base
            PathBuf::new()
                .join(url.trim_start_matches('/'))
                .to_response(),
        )
        .unwrap()
    });
}
trait ToResponse {
    fn to_response(self) -> Response<Cursor<Vec<u8>>>;
}
impl ToResponse for PathBuf {
    fn to_response(self) -> Response<Cursor<Vec<u8>>> {
        let env = std::env::current_dir().unwrap().join("website");
        Response::from_data(match std::fs::read(env.join(&self)) {
            Ok(x) => x,
            Err(_) => match std::fs::read(env.join(&self).join("index.html")) {
                Ok(x) => x,
                Err(_) => Pages::default().get_page(self).into_string().into(),
            },
        })
    }
}
