use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::net::IpAddr::{V4, V6};
use bytes::{Buf, BytesMut};
use crate::message::{Message, IPV4_SIZE, IPV6_SIZE, USER_INFO_CODE, IPV4_CODE, IPV6_CODE};
use crate::message::Message::{FileSendRequest, UserInfoMessage};
use crate::user_info::UserInfo;

fn get_name(buffer: &mut BytesMut) -> String {
    const MAX_NAME_LEN: usize = 64;
    String::from_utf8(buffer.split_to(MAX_NAME_LEN).to_vec()).unwrap()
}

fn get_ip_addr(buffer: &mut BytesMut) -> Option<IpAddr> {
    match buffer.get_u8() {
        IPV4_CODE => {
            let octets: [u8; IPV4_SIZE] = buffer.split_to(IPV4_SIZE).as_ref().try_into().unwrap();
            Some(V4(Ipv4Addr::from(octets)))
        },
        IPV6_CODE => {
            let octets: [u8; IPV6_SIZE] = buffer.split_to(IPV6_SIZE).as_ref().try_into().unwrap();
            Some(V6(Ipv6Addr::from(octets)))
        },
        _ => None
    }
}

fn get_port_num(buffer: &mut BytesMut) -> u16 {
    (buffer.get_u8() as u16) | ((buffer.get_u8() as u16) << 8)
}

fn get_socket_addr(buffer: &mut BytesMut) -> Option<SocketAddr> {
    Some(SocketAddr::new(get_ip_addr(buffer)?, get_port_num(buffer)))
}

fn deserialize_user_info(mut buffer: BytesMut) -> Option<UserInfo> {
    let name = get_name(&mut buffer);
    Some(UserInfo::new(name, get_socket_addr(&mut buffer)?))
}

fn deserialize_user_info_message(buffer: BytesMut) -> Option<Message> {
    Some(UserInfoMessage(deserialize_user_info(buffer)?))
}

fn deserialize_file_send_request(buffer: BytesMut) -> Option<Message> {
    Some(FileSendRequest(deserialize_user_info(buffer)?))
}

pub fn deserialize(mut buffer: BytesMut) -> Option<Message> {
    match buffer.get_u8() {
        USER_INFO_CODE => deserialize_user_info_message(buffer),
        _ => panic!("Can't deserialize")
    }
}
