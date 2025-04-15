use crate::user_info::UserInfo;

pub const USER_INFO_CODE: u8 = 0;
pub const FILE_SEND_REQUEST_MESSAGE: u8 = 1;

pub const MESSAGE_BYTES: usize = 1024;
pub const NAME_BYTES: usize = 64;
pub const IPV4_CODE: u8 = 0;
pub const IPV6_CODE: u8 = 1;
pub const IPV4_SIZE: usize = 4;
pub const IPV6_SIZE: usize = 16;

pub enum Message {
    UserInfoMessage(UserInfo),
    FileSendRequest(UserInfo),
}
