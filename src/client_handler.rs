use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::client::Client;

type ClientHandler = Arc<RwLock<HashMap<SocketAddr, Mutex<Client>>>>;

async fn send_info_to_others(clients: &ClientHandler, curr_addr: &SocketAddr) {
    for (addr, client) in clients.iter() {
        if addr != curr_addr {
            client.lock().await.send_message();
        }
    }
}

trait Handler {
    async fn update_client_list(&mut self);
}

impl Handler for ClientHandler {
    async fn update_client_list(&mut self) {
        let clients = self.read().await;
        for (addr, client) in clients.iter() {
            send_info_to_others(self, addr).await;
        }
    }
}
