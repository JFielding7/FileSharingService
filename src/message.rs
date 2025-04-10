use std::io::Read;

use crate::message::Message::{FileSendRequest, UserInfo};
use std::net::IpAddr::{V4, V6};
use std::net::SocketAddr;

pub const MAX_NAME_LEN: usize = 63;
pub const MESSAGE_BYTES: usize = 1024;

pub enum Message {
    UserInfo(String, SocketAddr),
    FileSendRequest(String, SocketAddr),
}

impl Message {
    pub fn byte_buffer(&self) -> Vec<u8> {
        match self {
            UserInfo(name, addr) => user_info_buffer(name, addr),
            FileSendRequest(name, addr) => file_send_request_buffer(name, addr),
        }
    }
}

fn fill_name(buffer: &mut [u8], name: &String) {
    for (i, char) in name.bytes().enumerate() {
        buffer[i] = char;
    }
    buffer[name.len()] = 0;
}

fn fill_socket_addr(buffer: &mut [u8], addr: &SocketAddr) {
    const IPV4_CODE: u8 = 0;
    const IPV6_CODE: u8 = 1;
    const IP_ADDR_OFFSET: usize = 1;
    const PORT_NUM_OFFSET: usize = 17;

    let octets: &[u8] = match addr.ip() {
        V4(ip) => {
            buffer[0] = IPV4_CODE;
            &ip.octets()
        }
        V6(ip) => {
            buffer[0] = IPV6_CODE;
            &ip.octets()
        }
    };

    for (i, byte) in octets.bytes().enumerate() {
        buffer[i + IP_ADDR_OFFSET] = byte.unwrap();
    }

    let port = addr.port();
    buffer[PORT_NUM_OFFSET] = (port & 0b1111_1111) as u8;
    buffer[PORT_NUM_OFFSET + 1] = (port >> 8) as u8;
}

fn fill_name_and_socket_addr(message_code: u8,
                             name: &String,
                             addr: &SocketAddr
) -> Vec<u8> {
    const NAME_OFFSET: usize = 1;
    const SOCKET_ADDR_OFFSET: usize = MAX_NAME_LEN + 1;

    let buffer = vec![0; MESSAGE_BYTES];
    buffer[0] = message_code;
    fill_name(&mut buffer[NAME_OFFSET..], name);
    fill_socket_addr(&mut buffer[SOCKET_ADDR_OFFSET..], addr);
    buffer
}

fn user_info_buffer(name: &String, addr: &SocketAddr) -> Vec<u8> {
    const USER_INFO_CODE: u8 = 0;
    fill_name_and_socket_addr(USER_INFO_CODE, name, addr)
}

fn file_send_request_buffer(name: &String, addr: &SocketAddr) -> Vec<u8> {
    const FILE_SEND_REQUEST_CODE: u8 = 1;
    fill_name_and_socket_addr(FILE_SEND_REQUEST_CODE, name, addr)
}

