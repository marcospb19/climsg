use std::os::unix::net::UnixStream;

use clap::Parser;
use climsg_core::{ClientMessage, MessageStream, ServerMessage, SERVER_SOCKET_FILE};

#[derive(Parser)]
#[command(about, version)]
enum ArgCommand {
    #[command(visible_alias = "s")]
    Send { key: String, value: String },
    #[command(visible_alias = "l")]
    Listen { key: String },
}

fn main() {
    let command = ArgCommand::parse();

    let stream = UnixStream::connect(SERVER_SOCKET_FILE).expect("FAILED TO CONNECT TO SERVER, IS IT RUNNING?");
    let mut stream = MessageStream::new(stream);

    match command {
        ArgCommand::Send { key, value } => {
            let message = ClientMessage::SendSignal(key, value);
            let message = serde_json::to_string(&message).unwrap();
            stream.send(message).unwrap();
        }
        ArgCommand::Listen { key } => {
            let message = ClientMessage::Listen(key.clone());
            let message = serde_json::to_string(&message).unwrap();
            stream.send(message).unwrap();

            loop {
                let message = stream.receive().unwrap();
                let message = std::str::from_utf8(&message).unwrap();
                let message = serde_json::from_str::<ServerMessage>(message).unwrap();
                println!("received message for {key}: '{message:?}'.");
            }
        }
    };
}
