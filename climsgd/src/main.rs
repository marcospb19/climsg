use std::{os::unix::net::UnixListener, sync::Arc, thread};

use async_io::block_on;
use climsg_core::{ClientMessage, MessageStream, ServerMessage, DEFAULT_SERVER_SOCKET_PATH};
use dashmap::DashMap;
use tokio::sync::broadcast;

// optimization TODO: do manual reference-counting to de-allocate senders as listeners go away
type BroadcastChannels = Arc<DashMap<String, broadcast::Sender<String>>>;

fn main() {
    let broadcast_channels: BroadcastChannels = DashMap::new().into();

    let _ = std::fs::remove_file(DEFAULT_SERVER_SOCKET_PATH);
    let listener = UnixListener::bind(DEFAULT_SERVER_SOCKET_PATH).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap().into();
        let broadcast_channels = broadcast_channels.clone();
        thread::spawn(|| handle_connection(stream, broadcast_channels));
    }
}

fn handle_connection(mut stream: MessageStream, broadcast_channels: BroadcastChannels) {
    loop {
        let msg = stream.receive().unwrap();
        let msg = std::str::from_utf8(&msg).unwrap();
        let msg = serde_json::from_str(msg).unwrap();

        match msg {
            ClientMessage::Listen(key) => {
                let new_sender = || broadcast::channel(128).0;

                let mut receiver = broadcast_channels.entry(key).or_insert_with(new_sender).subscribe();

                while let Ok(msg) = block_on(receiver.recv()) {
                    stream.send(ServerMessage(msg)).unwrap();
                }
            }
            ClientMessage::SendSignal(key, body) => {
                if let Some(sender) = broadcast_channels.get(&key) {
                    sender.send(body).unwrap();
                }
            }
            ClientMessage::Close => break,
        }
    }
}
