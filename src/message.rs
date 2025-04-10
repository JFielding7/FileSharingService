use std::io::Read;

use crate::message::Message::{FileSendRequest, UserInfoMessage};
use crate::user_info::UserInfo;
use std::net::IpAddr::{V4, V6};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

const USER_INFO_CODE: u8 = 0;
const FILE_SEND_REQUEST_MESSAGE: u8 = 1;

pub enum Message {
    UserInfoMessage(UserInfo),
    FileSendRequest(UserInfo),
}

impl Message {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            UserInfoMessage(user) => serialize_user_info_message(user),
            FileSendRequest(user) => serialize_file_send_request(user),
        }
    }

    pub fn deserialize(buffer: &[u8]) -> Option<Self> {
        match buffer[0] {
            USER_INFO_CODE => deserialize_user_info_message(buffer),
            _ => panic!("Can't deserialize")
        }
    }
}

pub const MESSAGE_BYTES: usize = 1024;
pub const NAME_BYTES: usize = 64;
const IPV4_CODE: u8 = 0;
const IPV6_CODE: u8 = 1;
const IPV4_SIZE: usize = 4;
const IPV6_SIZE: usize = 16;
const NAME_OFFSET: usize = 1;
const SOCKET_ADDR_OFFSET: usize = NAME_OFFSET + NAME_BYTES;
const IP_ADDR_OFFSET: usize = 1;
const PORT_NUM_OFFSET: usize = IP_ADDR_OFFSET + IPV6_SIZE;

fn serialize_name(buffer: &mut [u8], name: &String) {
    for (i, char) in name.bytes().enumerate() {
        buffer[i] = char;
    }
    buffer[name.len()] = 0;
}

fn deserialize_name(buffer: &[u8]) -> String {
    let name = buffer.iter().take_while(|&&byte| byte != 0);
    String::from_utf8(name.map(|byte| *byte).collect()).unwrap()
}

fn serialize_socket_addr(buffer: &mut [u8], addr: SocketAddr) {
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

fn deserialize_ip_addr(buffer: &[u8]) -> Option<IpAddr> {
    let i = IP_ADDR_OFFSET;
    match buffer[0] {
        IPV4_CODE => {
            let octets: [u8; 4] = [buffer[i], buffer[i + 1], buffer[i + 2], buffer[i + 3]];
            Some(V4(Ipv4Addr::from(octets)))
        },
        IPV6_CODE => {
            let mut octets: [u8; 16] = [0; 16];
            for (i, &byte) in buffer[IP_ADDR_OFFSET..IP_ADDR_OFFSET + IPV6_SIZE].iter().enumerate() {
                octets[i] = byte;
            }
            Some(V6(Ipv6Addr::from(octets)))
        },
        _ => None
    }
}

fn deserialize_port_num(buffer: &[u8]) -> u16 {
    (buffer[0] as u16) | ((buffer[1] as u16) << 8)
}

fn deserialize_socket_addr(buffer: &[u8]) -> Option<SocketAddr> {
    let addr = deserialize_ip_addr(buffer);
    Some(SocketAddr::new(addr?, deserialize_port_num(&buffer[PORT_NUM_OFFSET..])))
}

fn serialize_user_info(message_code: u8,
                       user: &UserInfo,
) -> Vec<u8> {
    let mut buffer = vec![0; MESSAGE_BYTES];
    buffer[0] = message_code;
    serialize_name(&mut buffer[NAME_OFFSET..], &user.name);
    serialize_socket_addr(&mut buffer[SOCKET_ADDR_OFFSET..], user.socket_addr);
    buffer
}

fn deserialize_user_info(buffer: &[u8]) -> Option<UserInfo> {
    Some(UserInfo::new(
        deserialize_name(&buffer[NAME_OFFSET..]),
        deserialize_socket_addr(&buffer[SOCKET_ADDR_OFFSET..])?
    ))
}

fn deserialize_user_info_message(buffer: &[u8]) -> Option<Message> {
    Some(UserInfoMessage(deserialize_user_info(buffer)?))
}

fn serialize_user_info_message(user: &UserInfo) -> Vec<u8> {
    serialize_user_info(USER_INFO_CODE, user)
}

fn serialize_file_send_request(user: &UserInfo) -> Vec<u8> {
    serialize_user_info(FILE_SEND_REQUEST_MESSAGE, user)
}

fn deserialize_file_send_request(buffer: &[u8]) -> Option<Message> {
    Some(FileSendRequest(deserialize_user_info(buffer)?))
}
