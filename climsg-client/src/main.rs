use clap::Parser;
use climsg_core::{ClientMessage, MessageStream, ServerMessage};

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

    let mut stream = MessageStream::connect_to_default().expect("Failed to connect to server, is it running?");

    match command {
        ArgCommand::Send { key, value } => {
            let message = ClientMessage::SendSignal(key, value);
            stream.send(message).unwrap();
        }
        ArgCommand::Listen { key } => {
            let message = ClientMessage::Listen(key.clone());
            stream.send(message).unwrap();

            loop {
                let message = stream.receive().unwrap();
                let message = std::str::from_utf8(&message).unwrap();
                let message = serde_json::from_str::<ServerMessage>(message).unwrap();
                println!("Received message for {key}: '{message:?}'.");
            }
        }
    };
}
