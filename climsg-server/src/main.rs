use std::{
    io::Write,
    os::unix::net::{UnixListener, UnixStream},
    thread,
    time::Duration,
};

use climsg_core::SERVER_SOCKET_FILE;
use flume::{Receiver, Sender};

fn run_message_generator(sender: Sender<String>) {
    for i in 0..u64::MAX {
        thread::sleep(Duration::from_secs(1));
        let msg = format!("Message {i}");
        sender.send(msg).unwrap();
    }
}

fn main() {
    let (tx, rx) = flume::unbounded();

    thread::spawn(|| run_message_generator(tx));
    listen_to_connections(rx);
}

fn listen_to_connections(receiver: Receiver<String>) {
    std::fs::remove_file(SERVER_SOCKET_FILE).unwrap();
    let listener = UnixListener::bind(SERVER_SOCKET_FILE).unwrap();

    for stream in listener.incoming() {
        let receiver = receiver.clone();
        thread::spawn(move || handle_connection(stream.unwrap(), receiver));
    }
}

fn handle_connection(mut stream: UnixStream, receiver: Receiver<String>) {
    // Resubscribe to skip queued messages
    let receiver = receiver;

    loop {
        let msg = receiver.recv().unwrap();
        println!("Sending message {msg}");
        stream.write_all(msg.as_bytes()).unwrap();
    }
}
