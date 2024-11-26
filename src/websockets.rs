use std::{io::Read, path::PathBuf, thread::spawn};

use rustc_serialize::base64::{CharacterSet::Standard, Config, Newline, ToBase64};
use sha1::{Digest, Sha1};
use tiny_http::Server;

use crate::web::ToWebResponse;

pub fn convert_key(input: &str) -> String {
    let mut input = input.to_string().into_bytes();
    let mut bytes = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
        .to_string()
        .into_bytes();
    input.append(&mut bytes);

    // create a SHA3-256 object
    let mut hasher = Sha1::new();

    // write input message
    hasher.update(&input);

    // read hash digest
    let result = hasher.finalize();

    //make it a string??
    let rerun = result.to_base64(Config {
        char_set: Standard,
        pad: true,
        line_length: None,
        newline: Newline::LF,
    });
    rerun
}

pub fn websocket_server(server: Server) {
    let port = server.server_addr().to_ip().unwrap().port();
    server
        .incoming_requests()
        .into_iter()
        .enumerate()
        .for_each(|(i, request)| {
            println!("Fetch {} | Url: {}", i, request.url());
            spawn(move || {
                let url = PathBuf::from(request.url().trim_start_matches('/'));
                match request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv(&"Upgrade"))
                    .and_then(|header| {
                        if header.value == "websocket" {
                            Some(header)
                        } else {
                            None
                        }
                    }) {
                    None => {
                        request
                            .respond(url.to_web_response(port))
                            .expect("Responded");
                        return;
                    }
                    _ => (),
                }
                let key = match request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv(&"Sec-WebSocket-Key"))
                    .map(|h| h.value.clone())
                {
                    None => {
                        let response = tiny_http::Response::new_empty(tiny_http::StatusCode(400));
                        request.respond(response).expect("Responded");
                        return;
                    }
                    Some(k) => k,
                };

                let response = tiny_http::Response::new_empty(tiny_http::StatusCode(101))
                    .with_header("Upgrade: websocket".parse::<tiny_http::Header>().unwrap())
                    .with_header("Connection: Upgrade".parse::<tiny_http::Header>().unwrap())
                    .with_header(
                        "Sec-WebSocket-Protocol: ping"
                            .parse::<tiny_http::Header>()
                            .unwrap(),
                    )
                    .with_header(
                        format!("Sec-WebSocket-Accept: {}", convert_key(key.as_str()))
                            .parse::<tiny_http::Header>()
                            .unwrap(),
                    );

                //
                let mut stream = request.upgrade("websocket", response);

                //somehow make this depend on the page
                loop {
                    let mut out = Vec::new();
                    match Read::by_ref(&mut stream).take(1).read_to_end(&mut out) {
                        Ok(n) if n >= 1 => {
                            // "Hello" frame
                            let data = [0x81, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f];
                            //let data = [1,2,3];
                            stream.write(&data).ok();

                            stream.flush().ok();
                        }
                        Ok(_) => panic!("eof ; should never happen"),
                        Err(e) => {
                            println!("closing connection because: {}", e);
                            return;
                        }
                    };
                }
            });
        });
}
