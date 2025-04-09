use tokio::net::TcpStream;

use std::io::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::message::Message;

const MAX_NAME_LEN: usize = 64;
const MESSAGE_BYTES: usize = 1024;

pub struct Client {
    name: String,
    socket_addr: SocketAddr,
    tcp_stream: TcpStream,
    tcp_stream_buffer: [u8; MESSAGE_BYTES],
}

impl Client {
    pub fn new(socket_addr: SocketAddr,
               tcp_stream: TcpStream,
) -> Self {
        Self {
            name: "".to_string(),
            socket_addr,
            tcp_stream,
            tcp_stream_buffer: [0; MESSAGE_BYTES],
        }
    }

    pub async fn read(&mut self) -> Result<usize, Error> {
        let num_bytes = self.tcp_stream.read_exact(&mut self.tcp_stream_buffer).await?;
        Ok(num_bytes)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = if name.len() > MAX_NAME_LEN {
            name[0..MAX_NAME_LEN].to_string()
        } else {
            name
        }
    }

    pub fn buffer_byte(&self, n: usize) -> u8 {
        self.tcp_stream_buffer[n]
    }

    pub fn send_message(&mut self, message: Message) {
        self.tcp_stream
    }
}
