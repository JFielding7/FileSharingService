mod client;
mod client_handler;
mod message;
mod user_info;
mod message_serializer;
mod message_processor;
mod buffer;

use crate::client::Client;
use crate::client_handler::{ClientHandler, ClientMap};
use crate::message::{Message, MESSAGE_BYTES};
use std::io;
use std::sync::Arc;
use bytes::BytesMut;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::message_processor::deserialize_user_info_message;

fn process_message(buffer: BytesMut, client_map: ClientMap) {
    println!("processing");
    match deserialize_user_info_message(buffer).unwrap() {
        Message::UserInfoMessage(user) => {
            println!("{:?}", user)
        }
        _ => {}
    }
}

async fn listen_to_stream(client_mutex: Arc<Mutex<Client>>,
                          client_map: ClientMap,
) -> io::Result<()> {
    println!("Listening");

    loop {
        let mut client = client_mutex.lock().await;
        let buffer = client.read().await?;
        drop(client);

        match buffer.len() {
            0 => break,
            _ => process_message(buffer, client_map.clone())
        }
    }

    Ok(())
}

async fn listen_for_connections(listener: &TcpListener) {
    let client_handler = ClientHandler::new();

    loop {
        let (stream, address) = listener.accept().await.unwrap();
        println!("Connected {}", address.port());
        let client_mutex = Arc::new(Mutex::new(Client::new(address, stream)));

        let client_mutex_clone = client_mutex.clone();
        let client_map = client_handler.client_map.clone();

        tokio::spawn(async move {
            match listen_to_stream(client_mutex_clone, client_map).await {
                Ok(_) => println!("Connection Closed"),
                Err(e) => println!("{e}")
            }
        });

        client_handler.insert(address, client_mutex).await;
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();
    listen_for_connections(&listener).await;
}
