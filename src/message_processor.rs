use crate::buffer::Buffer;
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

    pub fn deserialize(buffer_vec: Vec<u8>) -> Option<Self> {
        let mut buffer = Buffer::new(buffer_vec);
        match buffer.get_byte() {
            USER_INFO_CODE => deserialize_user_info_message(buffer),
            _ => panic!("Can't deserialize")
        }
    }
}

fn deserialize_user_info(mut buffer: Buffer) -> Option<UserInfo> {
    Some(UserInfo::new(buffer.get_name(), buffer.get_socket_addr()?))
}

pub fn deserialize_user_info_message(buffer: Buffer) -> Option<Message> {
    Some(UserInfoMessage(deserialize_user_info(buffer)?))
}

fn deserialize_file_send_request(buffer: Buffer) -> Option<Message> {
    Some(FileSendRequest(deserialize_user_info(buffer)?))
}
