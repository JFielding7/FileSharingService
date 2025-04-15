use std::io;
use tokio::net::TcpStream;

use crate::message::{Message, MESSAGE_BYTES};
use crate::user_info::UserInfo;
use bytes::BytesMut;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::sync::Mutex;
use crate::message_serializer::serialize_message;

#[derive(Debug)]
pub struct Client {
    user_info: UserInfo,
    reader: Arc<Mutex<ReadHalf<TcpStream>>>,
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
}

impl Client {
    pub fn new(socket_addr: SocketAddr, tcp_stream: TcpStream) -> Self {
        let (reader, writer) = split(tcp_stream);
        Self {
            user_info: UserInfo { name: "".to_string(), socket_addr },
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.user_info.name = name;
    }

    pub fn get_name(&self) -> String {
        self.user_info.name.clone()
    }

    pub fn get_info(&self) -> UserInfo {
        self.user_info.clone()
    }

    pub async fn read(&self) -> io::Result<BytesMut> {
        let mut buffer = BytesMut::with_capacity(MESSAGE_BYTES);
        buffer.resize(MESSAGE_BYTES, 0);
        self.reader.lock().await.read_exact(&mut buffer).await?;
        Ok(buffer)
    }

    pub async fn send_message(&mut self, message: Message) -> io::Result<()> {
        let buffer = serialize_message(message);
        self.writer.lock().await.write_all(&buffer).await?;
        Ok(())
    }
}
