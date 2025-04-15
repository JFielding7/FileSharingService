use std::io;
use tokio::net::TcpStream;

use crate::message::{MESSAGE_BYTES, NAME_BYTES};
use crate::user_info::UserInfo;
use std::net::SocketAddr;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;

pub struct Client {
    client_info: UserInfo,
    tcp_stream: TcpStream,
}

impl Client {
    pub fn new(socket_addr: SocketAddr, tcp_stream: TcpStream) -> Self {
        Self {
            client_info: UserInfo { name: "".to_string(), socket_addr },
            tcp_stream,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.client_info.name = if name.len() >= NAME_BYTES {
            name[0..NAME_BYTES-1].to_string()
        } else {
            name
        }
    }

    pub fn get_info(&self) -> UserInfo {
        self.client_info.clone()
    }

    pub async fn read(&mut self) -> io::Result<BytesMut> {
        let mut buffer = BytesMut::with_capacity(MESSAGE_BYTES);
        buffer.resize(MESSAGE_BYTES, 0);
        self.tcp_stream.read_exact(&mut buffer).await?;
        Ok(buffer)
    }

    // pub async fn send_message(&mut self, message: Message) -> io::Result<()> {
    //     let buffer = message.serialize();
    //     self.tcp_stream.write_all(&buffer).await?;
    //     Ok(())
    // }
}
