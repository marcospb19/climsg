use std::{
    collections::HashMap,
    os::unix::net::UnixListener,
    sync::{Arc, Mutex},
    thread,
};

use async_io::block_on;
use climsg_core::{ClientMessage, MessageStream, ServerMessage, SERVER_SOCKET_FILE};
use dashmap::DashMap;
use tokio::sync::broadcast;

type BroadcastChannels = Arc<DashMap<String, broadcast::Sender<String>>>;

fn main() {
    let broadcast_channels: BroadcastChannels = DashMap::new().into();

    std::fs::remove_file(SERVER_SOCKET_FILE).unwrap();
    let listener = UnixListener::bind(SERVER_SOCKET_FILE).unwrap();

    for stream in listener.incoming() {
        let stream = MessageStream::new(stream.unwrap());
        let broadcast_channels = broadcast_channels.clone();
        let stream = thread::spawn(|| handle_connection(stream, broadcast_channels));
    }
}

fn handle_connection(mut stream: MessageStream, broadcast_channels: BroadcastChannels) {
    let msg = stream.receive().unwrap();
    let msg = std::str::from_utf8(&msg).unwrap();
    let msg = serde_json::from_str(msg).unwrap();

    match msg {
        ClientMessage::Listen(key) => {
            let new_broadcast = || broadcast::channel(128).0;

            let mut receiver = broadcast_channels.entry(key).or_insert_with(new_broadcast).subscribe();

            while let Ok(msg) = block_on(receiver.recv()) {
                let msg = ServerMessage::Signal(msg);
                let msg = serde_json::to_string(&msg).unwrap();
                stream.send(msg).unwrap();
            }
        }
        ClientMessage::SendSignal(key, body) => {
            let Some(sender) = broadcast_channels.get(&key) else { return; };
            sender.send(body).unwrap();
        }
    }
}
