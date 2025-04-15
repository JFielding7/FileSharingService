use bytes::{Buf, BytesMut};
use crate::buffer::deserialize_user_info;
use crate::message::Message;
use crate::message::Message::{FileSendRequest, UserInfoMessage};
use crate::user_info::UserInfo;

pub const USER_INFO_CODE: u8 = 0;
pub const FILE_SEND_REQUEST_MESSAGE: u8 = 1;

impl Message {
    // pub fn serialize(&self) -> Vec<u8> {
    //     match self {
    //         UserInfoMessage(user) => serialize_user_info_message(user),
    //         FileSendRequest(user) => serialize_file_send_request(user),
    //     }
    // }

    pub fn deserialize(buf: &[u8]) -> Option<Self> {
        let mut buffer = BytesMut::from(buf);
        match buffer.get_u8() {
            USER_INFO_CODE => deserialize_user_info_message(buffer),
            _ => panic!("Can't deserialize")
        }
    }
}

pub fn deserialize_user_info_message(buffer: BytesMut) -> Option<Message> {
    Some(UserInfoMessage(deserialize_user_info(buffer)?))
}

fn deserialize_file_send_request(buffer: BytesMut) -> Option<Message> {
    Some(FileSendRequest(deserialize_user_info(buffer)?))
}
