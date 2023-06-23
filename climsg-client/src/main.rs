use std::{io::Read, os::unix::net::UnixStream};

fn main() {
    let mut listener = UnixStream::connect("/tmp/climsg").unwrap();

    loop {
        let mut buf = [0; 128];
        let bytes = listener.read(&mut buf).unwrap();

        let msg = String::from_utf8_lossy(&buf[..bytes]);
        println!("received message: {msg}");
    }
}
