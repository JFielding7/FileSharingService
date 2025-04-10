use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::client::Client;
use crate::message::Message;
use crate::user_info::UserInfo;

pub struct ClientHandler {
    clients: Arc<RwLock<HashMap<SocketAddr, Mutex<Client>>>>,
}

async fn send_info_to_others(clients: Iter<'_, SocketAddr, Mutex<Client>>, user_info: UserInfo) -> io::Result<()> {
    for (&addr, client) in clients {
        if addr != user_info.socket_addr {
            client.lock().await.send_message(Message::UserInfoMessage(user_info.clone())).await?;
        }
    }

    Ok(())
}

impl ClientHandler {
    async fn update_client_list(&mut self) -> io::Result<()> {
        let clients = self.clients.read().await;
        for (_addr, client) in clients.iter() {
            send_info_to_others(clients.iter(), client.lock().await.get_info()).await?;
        }

        Ok(())
    }
}
