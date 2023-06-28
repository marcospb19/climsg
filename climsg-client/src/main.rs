use std::os::unix::net::UnixStream;

use climsg_core::{ClientMessage, MessageStream, ServerMessage, SERVER_SOCKET_FILE};

fn main() {
    let arg = std::env::args().nth(1).expect("Expected one argument");

    let stream = UnixStream::connect(SERVER_SOCKET_FILE).unwrap();
    let mut stream = MessageStream::new(stream);

    let key = "key";

    match arg.as_str() {
        "send" => {
            let message = ClientMessage::SendSignal(key.to_string(), "message-contents-example".into());
            let message = serde_json::to_string(&message).unwrap();
            stream.send(message).unwrap();
        }
        "listen" => {
            let message = ClientMessage::Listen(key.to_string());
            let message = serde_json::to_string(&message).unwrap();
            stream.send(message).unwrap();

            loop {
                let message = stream.receive().unwrap();
                let message = std::str::from_utf8(&message).unwrap();
                let message = serde_json::from_str::<ServerMessage>(message).unwrap();
                println!("received message for {key}: '{message:?}'.");
            }
        }
        _ => unreachable!("Unexpected argument"),
    }
}
