use crate::message::Message::UserInfoMessage;
use crate::message::{Message, FILE_SEND_REQUEST_MESSAGE, IPV4_CODE, IPV6_CODE, MESSAGE_BYTES, NAME_BYTES, USER_INFO_CODE};
use crate::user_info::UserInfo;
use bytes::{BufMut, BytesMut};
use std::io::Read;
use std::net::IpAddr::{V4, V6};
use std::net::{IpAddr, SocketAddr};

fn serialize_name(buffer: &mut BytesMut, name: &String) {
    let len = buffer.len();

    for &char in name.as_bytes()[..NAME_BYTES].iter() {
        buffer.put_u8(char);
    }

    buffer.resize(len + NAME_BYTES, 0);

    println!("{}", buffer.len());
}
fn serialize_ip_addr(buffer: &mut BytesMut, ip_addr: IpAddr) {
    let octets: &[u8] = match ip_addr {
        V4(ip) => {
            buffer.put_u8(IPV4_CODE);
            &ip.octets()
        }
        V6(ip) => {
            buffer.put_u8(IPV6_CODE);
            &ip.octets()
        }
    };

    // TODO: error check ip
    for byte in octets.bytes() {
        buffer.put_u8(byte.unwrap());
    }
}

fn serialize_port_num(buffer: &mut BytesMut, port: u16) {
    buffer.put_u8((port & 0b1111_1111) as u8);
    buffer.put_u8((port >> 8) as u8);
}

fn serialize_socket_addr(buffer: &mut BytesMut, addr: SocketAddr) {
    serialize_ip_addr(buffer, addr.ip());
    serialize_port_num(buffer, addr.port());
}

fn serialize_user_info(message_code: u8,
                       user: &UserInfo,
) -> BytesMut {
    let mut buffer = BytesMut::with_capacity(MESSAGE_BYTES);
    buffer.put_u8(message_code);

    serialize_name(&mut buffer, &user.name);
    serialize_socket_addr(&mut buffer, user.socket_addr);
    buffer
}

fn serialize_user_info_message(user: &UserInfo) -> BytesMut {
    serialize_user_info(USER_INFO_CODE, user)
}

fn serialize_file_send_request(user: &UserInfo) -> BytesMut {
    serialize_user_info(FILE_SEND_REQUEST_MESSAGE, user)
}

pub fn serialize_message(message: Message) -> BytesMut {
    let mut buffer = match message {
        UserInfoMessage(user_info) => serialize_user_info_message(&user_info),
        _ => panic!("Invalid Message")
    };

    buffer.resize(MESSAGE_BYTES, 0);
    buffer
}
