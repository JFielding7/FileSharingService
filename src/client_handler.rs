use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::client::Client;
use crate::message::Message;
use crate::user_info::UserInfo;

pub type ClientMap = Arc<RwLock<HashMap<SocketAddr, Arc<Mutex<Client>>>>>;

pub struct ClientHandler {
    pub client_map: ClientMap,
}

impl ClientHandler {
    pub fn new() -> Self {
        Self { client_map: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn insert(&self, addr: SocketAddr, client_mutex: Arc<Mutex<Client>>) {
        self.client_map.write().await.insert(addr, client_mutex);
    }

    async fn send_info_to_others(clients: Iter<'_, SocketAddr, Arc<Mutex<Client>>>,
                                 user_info: UserInfo
    ) -> io::Result<()> {
        for (&addr, client) in clients {
            if addr != user_info.socket_addr {
                client.lock().await.send_message(Message::UserInfoMessage(user_info.clone())).await?;
            }
        }

        Ok(())
    }

    async fn update_client_list(&mut self) -> io::Result<()> {
        let clients = self.client_map.read().await;
        for (_addr, client) in clients.iter() {
            Self::send_info_to_others(clients.iter(), client.lock().await.get_info()).await?;
        }

        Ok(())
    }
}
