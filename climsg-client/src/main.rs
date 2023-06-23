use std::{io::Read, os::unix::net::UnixStream};

use climsg_core::SERVER_SOCKET_FILE;

fn main() {
    let mut listener = UnixStream::connect(SERVER_SOCKET_FILE).unwrap();

    loop {
        let mut buf = [0; 128];
        let bytes = listener.read(&mut buf).unwrap();

        if bytes == 0 {
            println!("received empty message, exiting");
            break;
        }

        let msg = String::from_utf8_lossy(&buf[..bytes]);
        println!("received message: {msg}");
    }
}
