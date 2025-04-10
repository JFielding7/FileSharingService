use std::net::SocketAddr;

pub struct UserInfo {
    pub name: String,
    pub socket_addr: SocketAddr,
}

impl UserInfo {
    pub fn new(name: String, socket_addr: SocketAddr) -> Self {
        Self { name, socket_addr }
    }
}

impl Clone for UserInfo {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            socket_addr: self.socket_addr,
        }
    }
}