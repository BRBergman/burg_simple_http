use maud::{html,DOCTYPE};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::{fs::File, path::Path};
use tiny_http::{Response, Server};

mod result_path;
use result_path::{ResultPath, ToFile, ToResultPath};
//https://doc.rust-lang.org/std/keyword.break.html
//this is really cool^
fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    for request in server.incoming_requests() {
        let folder_not_found =
            html! {
                (DOCTYPE)
                html{
                head{title{"empty folder"}}
                body{
                h1{"Missing Folder!"} 
                p1{"guh"}}
            }
        }.into_string();
        let file_404_page = "404 :(".to_response();
        let path_full = std::env::current_dir()
            .unwrap()
            .join(Path::new("web"))
            .join(Path::new(request.url().trim_start_matches('/')));
        println!("{}", path_full.display());
        match path_full.to_result_path() {
            ResultPath::File(file) => {
                let _ = request.respond(file.to_response());
            }
            ResultPath::Directory(path) => {
                match path.join("index.html").open_file() {
                    Ok(file) => {
                        let _ = request.respond(file.to_response());
                    }
                    Err(_) => {
                        //this is what happens if you try to open a folder that exists, but has no index.html
                        //let _ = request.respond(file_404_page);
                        let _ = request.respond(Response::from_data(folder_not_found));
                    }
                }
            }
            ResultPath::Err(_) => {
                let _ = request.respond(file_404_page);
            }
        };
    }
}

trait ToResponseFile {
    fn to_response(self) -> Response<File>;
}
impl ToResponseFile for File {
    fn to_response(self) -> Response<File> {
        Response::from_file(self)
    }
}
trait ToResponseStr {
    fn to_response(&self) -> Response<std::io::Cursor<Vec<u8>>>;
}
impl ToResponseStr for str {
    fn to_response(&self) -> Response<std::io::Cursor<Vec<u8>>> {
        Response::from_string(self)
    }
}
