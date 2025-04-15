use std::fmt;
use std::fmt::Formatter;
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

impl fmt::Debug for UserInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "User Info: [ Name: {}, Socket Addr: {} ]", self.name, self.socket_addr)
    }
}
