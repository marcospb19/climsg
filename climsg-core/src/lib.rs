use std::{
    io,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::Path,
};

use serde::{Deserialize, Serialize};

pub const DEFAULT_SERVER_SOCKET_PATH: &str = "/tmp/climsg-default-server";

// Fuck Around 'n Find 0ut
pub const ACKNOWLEDGE_REQUEST_CODE: &[u8] = &0x0000faf0_u32.to_be_bytes();

/// Protocoled abstraction over Unix Sockets that implement "messages"
/// instead of just byte slices.
pub struct MessageStream {
    socket: UnixStream,
}

impl From<UnixStream> for MessageStream {
    fn from(socket: UnixStream) -> Self {
        Self { socket }
    }
}

impl MessageStream {
    pub fn connect_to(path: impl AsRef<Path>) -> io::Result<Self> {
        UnixStream::connect(path.as_ref()).map(|socket| Self { socket })
    }

    pub fn connect_to_default() -> io::Result<Self> {
        Self::connect_to(DEFAULT_SERVER_SOCKET_PATH)
    }

    pub fn send(&mut self, message: String) -> io::Result<()> {
        let message_length = message.len() as u64;
        self.socket.write_all(message_length.to_be_bytes().as_slice())?;
        self.socket.write_all(message.as_bytes())?;
        self.wait_for_receival_acknowledgement();
        Ok(())
    }

    pub fn receive(&mut self) -> io::Result<Vec<u8>> {
        let mut message_length = [0; 8];
        self.socket.read_exact(&mut message_length).unwrap();

        let length = u64::from_be_bytes(message_length);
        let Ok(length) = usize::try_from(length) else {
            panic!("32-bit system does not support huge messages")
        };

        let mut buf = vec![0; length];
        self.socket.read_exact(&mut buf)?;

        // Acknowledge the reading
        self.send_acknowledgement();
        Ok(buf)
    }

    fn wait_for_receival_acknowledgement(&mut self) {
        let mut buf = [0; 4];
        self.socket.read_exact(&mut buf).unwrap();

        assert_eq!(ACKNOWLEDGE_REQUEST_CODE, buf);
    }

    fn send_acknowledgement(&mut self) {
        self.socket.write_all(ACKNOWLEDGE_REQUEST_CODE).unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Signal(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Listen(String),
    SendSignal(String, String),
}
