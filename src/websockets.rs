use rustc_serialize::base64::{CharacterSet::Standard, Config, Newline, ToBase64};
use sha3::{Digest, Sha3_256};
pub fn convert_key(input: &str) -> String {
    let mut input = input.to_string().into_bytes();
    let mut bytes = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
        .to_string()
        .into_bytes();
    input.append(&mut bytes);

    // create a SHA3-256 object
    let mut hasher:Sha3_256 = Sha3_256::new();

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
