use std::io::Error;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::{fs::File, path::Path};
use tiny_http::{Response, Server};

fn main() {
    let server = Server::http(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000)).unwrap();
    println!("running");
    for request in server.incoming_requests() {
        let file_404_page = Response::from_string("404 :(");
        let path_full =
            Path::new("../web/").join(Path::new(request.url().split_once('/').unwrap().1));
        //let home = Path::try_exists(&self);
        //make so that it adds the .htm so i dont have to
        //make that i just give the name and it adds .html or something
        match path_full.to_result_path() {
            ResultPath::File(path_buf) => {
                let _ = request.respond(Response::from_file(File::open(path_buf).unwrap()));
            }
            ResultPath::Directory(path_buf) => {
                match File::open(path_buf.join("index.html")) {
                    Ok(x) => {
                        let _ = request.respond(Response::from_file(x));
                    }
                    Err(_) => {
                        //this is what happens if you try to open a folder that exists, but has no index.html
                        let _ = request.respond(file_404_page);
                    }
                }
            }
            ResultPath::Err(_x) => {
                let _ = request.respond(file_404_page);
            }
        };
    }
}

impl ToResultPath for Path {
    fn to_result_path(&self) -> ResultPath {
        ResultPath::from_path(self.to_path_buf())
    }
}
impl ToResultPath for PathBuf {
    fn to_result_path(&self) -> ResultPath {
        ResultPath::from_path(self.to_path_buf())
    }
}
trait ToResultPath {
    fn to_result_path(&self) -> ResultPath;
}
enum ResultPath {
    File(PathBuf),
    Directory(PathBuf),
    Err(Error),
}

impl ResultPath {
    fn from_path(path: PathBuf) -> ResultPath {
        if path.exists() {
            if path.is_file() {
                return ResultPath::File(path);
            } else if path.is_dir() {
                return ResultPath::Directory(path);
            }
        }
        return ResultPath::Err(Error::new(
            std::io::ErrorKind::NotFound,
            path.to_str().unwrap(),
        ));
    }
}
