use std::io::Cursor;

use maud::{html, PreEscaped};

pub fn socket_page(port: u16) -> tiny_http::Response<Cursor<Vec<u8>>> {
    tiny_http::Response::from_data(html!{(websocket_bloat(port))
        p{"This example will receive \"Hello\" for each byte in the packet being sent. Tiny-http doesn't support decoding websocket frames, so we can't do anything better."}
        p{input type="text" id="msg";
        button onclick="send(document.getElementById('msg').value)" {"Send"};}
        p {"Received: "};
        p id="result";
    }.into_string())
    .with_header(
        "Content-type: text/html"
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}
fn websocket_bloat(port: u16) -> PreEscaped<String> {
    PreEscaped(format!(
        "<script type=\"text/javascript\">
        var socket = new WebSocket(\"ws://localhost:{}/\", \"ping\");

        function send(data) {{
            socket.send(data);
        }}

        socket.onmessage = function(event) {{
            document.getElementById('result').innerHTML += event.data + '<br />';
        }}
        </script>
        ",
        port
    ))
}
