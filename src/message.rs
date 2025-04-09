use std::net::SocketAddr;
use crate::client::Client;
use crate::message::Message::UserInfo;

pub enum Message {
    UserInfo(String, SocketAddr)
}

impl Message {
    pub fn fill_buffer(&self, buffer: &[u8]) {
        match self {
            UserInfo(name, addr) => fill_user_info_buffer(user, buffer)
        }
    }
}

fn send_user_info_message(recipient: &Client) {
    // for i in 0..MAX_NAME_LEN {
    //
    // }
}
