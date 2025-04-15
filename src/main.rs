mod client;
mod client_handler;
mod message;
mod user_info;
mod message_serializer;
mod message_deserializer;

use crate::client::Client;
use crate::message::Message::UserInfoMessage;
use crate::message_deserializer::deserialize;
use bytes::BytesMut;
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::client_handler::{ClientMapOperations, ClientMapRef};

fn process_message(buffer: BytesMut, clients: ClientMapRef) {
    println!("processing");
    match deserialize(buffer).unwrap() {
        UserInfoMessage(user) => {
            // println!("{:?}", user)
        }
        _ => {}
    }
}

async fn listen_to_stream(client: Arc<Client>,
                          client_map: ClientMapRef,
) -> io::Result<()> {
    println!("Listening");

    loop {
        let buffer = client.read().await?;

        match buffer.len() {
            0 => break,
            _ => process_message(buffer, client_map.clone())
        }
    }

    Ok(())
}

async fn listen_for_connections(listener: &TcpListener) {
    let mut client_handler = ClientMapRef::create();

    loop {
        for c in client_handler.read().await.values() {
            println!("Client: {:?}", c.get_info());
        }

        let (stream, address) = listener.accept().await.unwrap();
        // println!("Connected {}", address.port());

        let client = Arc::new(Client::new(address, stream));

        let client_clone = client.clone();
        let clients = client_handler.clone();

        tokio::spawn(async move {
            match listen_to_stream(client_clone, clients).await {
                Ok(_) => println!("Connection Closed"),
                Err(e) => println!("{e}")
            }
        });

        client_handler.add_client(address, client).await;
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();
    listen_for_connections(&listener).await;
}
