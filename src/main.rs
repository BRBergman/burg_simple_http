use std::io::Cursor;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::rc::Rc;
use tiny_http::{Response, Server};
use web::Pages;
mod web;
//https://doc.rust-lang.org/std/keyword.break.html
//this is really cool^
fn main() {
    server();
}
fn server(){
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    let path_base = std::env::current_dir().unwrap().join("website");
    server.incoming_requests().into_iter().for_each(|x| {
        let url = x.url().to_owned();
        x.respond(path_base.join(url.trim_start_matches('/')).to_responce()).unwrap();
    });
}
trait ToResponce {
    fn to_responce(self) -> Response<Cursor<Vec<u8>>>;
}
impl ToResponce for PathBuf {
    fn to_responce(self) -> Response<Cursor<Vec<u8>>> {
        let pages = Rc::new(Pages::default());
        Response::from_data(if self.is_file() {
            std::fs::read(&self).unwrap()
        } else {
            match std::fs::read(self.join("index.html")) {
                Ok(x) => x,
                Err(_) => pages.get_page(self).into_string().into(),
            }
        })
    }
}