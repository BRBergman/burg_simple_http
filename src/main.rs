use std::fs::File;
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
    //let pages = Pages::default(); //gotta be a better way to do this
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    let path_base = std::env::current_dir().unwrap().join("website");
    server.incoming_requests().into_iter().for_each(|x| {
        let url = x.url().to_owned();
        x.respond({
            match PathResponce::from(path_base.join(url.trim_start_matches('/'))) {
                PathResponce::FileExists(file) => file,
                PathResponce::IndexExists(index) => index,
                PathResponce::Imbedded(vec) => vec,
            }.to_responce()
        })
        .unwrap()
    });
}
trait ToResponce {
    fn to_responce(self) -> Response<Cursor<Vec<u8>>>;
}
impl ToResponce for Vec<u8>{
    fn to_responce(self) -> Response<Cursor<Vec<u8>>> {
        Response::from_data(self)
    }
}
enum PathResponce {
    FileExists(Vec<u8>),
    IndexExists(Vec<u8>),
    Imbedded(Vec<u8>),
}
impl PathResponce {
    fn from(path: PathBuf) -> PathResponce {
        let pages = Rc::new(Pages::default());
        if path.is_file() {
            Self::FileExists(std::fs::read(&path).unwrap())
        } else {
            match std::fs::read(path.join("index.html")) {
                Ok(x) => Self::IndexExists(x),
                Err(_) => Self::Imbedded(pages.get_page(path).into_string().into()),
            }
        }
    }
}
