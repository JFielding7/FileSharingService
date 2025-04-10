use std::net::SocketAddr;

pub struct UserInfo {
    pub name: String,
    pub socket_addr: SocketAddr,
}

impl Clone for UserInfo {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            socket_addr: self.socket_addr,
        }
    }
}