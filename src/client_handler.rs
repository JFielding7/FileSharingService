use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Iter;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::client::Client;
use crate::message::Message::UserInfoMessage;
use crate::user_info::UserInfo;

pub type ClientMapRef = Arc<RwLock<HashMap<SocketAddr, Arc<Client>>>>;

pub trait ClientMapOperations {
    fn create() -> Self;
    async fn add_client(&mut self, socket_addr: SocketAddr, client_mutex: Arc<Client>);
    async fn update_client_list(&mut self) -> io::Result<()>;
}

impl ClientMapOperations for ClientMapRef {
    fn create() -> Self {
        Arc::new(RwLock::new(HashMap::new()))
    }

    async fn add_client(&mut self, socket_addr: SocketAddr, client_mutex: Arc<Client>) {
        let mut clients = self.write().await;
        println!("Contains: {}", clients.contains_key(&socket_addr));
        clients.insert(socket_addr, client_mutex);
    }

    async fn update_client_list(&mut self) -> io::Result<()> {
        // let clients = self.read().await;
        // for (_addr, client_lock) in clients.iter() {
        //     let client_guard = client_lock.lock().await;
        //     let info = client_guard.get_info();
        //     drop(client_guard);
        //     send_info_to_others(clients.iter(), info).await?;
        // }

        Ok(())
    }
}

async fn send_info_to_others(clients: Iter<'_, SocketAddr, Arc<Mutex<Client>>>, info: UserInfo) -> io::Result<()> {
    for (&addr, client) in clients {
        if addr != info.socket_addr {
            client.lock().await.send_message(UserInfoMessage(info.clone())).await?;
        }
    }

    Ok(())
}

