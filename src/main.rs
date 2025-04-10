mod client;
mod client_handler;
mod message;
mod user_info;

use crate::client::Client;
use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, MutexGuard, RwLock};
use crate::message::MESSAGE_BYTES;

fn process_message(mut client: MutexGuard<Client>) {

    // match client.buffer_byte(0) {
    //     CONNECT => client.update_client_list()
    // }
}

async fn listen_to_stream(client_mutex: Arc<Mutex<Client>>) -> Result<(), Error> {
    println!("Listening");
    let mut buffer = vec![0; MESSAGE_BYTES];

    loop {
        let mut client = client_mutex.lock().await;

        let bytes = client.read(&mut buffer).await?;

        match bytes {
            0 => { break; }
            _ => process_message(client)
        }
    }

    Ok(())
}

async fn listen_for_connections(listener: &TcpListener) {
    let mut clients = Arc::new(RwLock::new(HashMap::new()));

    loop {
        let (stream, address) = listener.accept().await.unwrap();
        println!("Connected {}", address.port());
        let client_mutex = Arc::new(Mutex::new(Client::new(address, stream)));

        let client_mutex_clone = client_mutex.clone();
        tokio::spawn(async move {
            match listen_to_stream(client_mutex_clone).await {
                Ok(_) => {
                    println!("Connection Closed");
                }
                Err(e) => println!("{e}")
            }
        });

        clients.write().await.insert(address, client_mutex);
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();
    listen_for_connections(&listener).await;
}
