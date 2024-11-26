use maud::PreEscaped;

pub fn socket_page(port: u16) -> PreEscaped<String> {
    PreEscaped(format!(
        "
        <script type=\"text/javascript\">
        var socket = new WebSocket(\"ws://localhost:{}/\", \"ping\");

        function send(data) {{
            socket.send(data);
        }}

        socket.onmessage = function(event) {{
            document.getElementById('result').innerHTML += event.data + '<br />';
        }}
        </script>
        <p>This example will receive &quot;Hello&quot; for each byte in the packet being sent.
        Tiny-http doesn't support decoding websocket frames, so we can't do anything better.</p>
        <p><input type=\"text\" id=\"msg\" />
        <button onclick=\"send(document.getElementById('msg').value)\">Send</button></p>
        <p>Received: </p>
        <p id=\"result\"></p>
    ",
        port
    ))
}
