use crate::user_info::UserInfo;

pub const MESSAGE_BYTES: usize = 1024;
pub const NAME_BYTES: usize = 64;
pub const IPV4_CODE: u8 = 0;
pub const IPV6_CODE: u8 = 1;
pub const IPV4_SIZE: usize = 4;
pub const IPV6_SIZE: usize = 16;
pub const NAME_OFFSET: usize = 1;
pub const SOCKET_ADDR_OFFSET: usize = NAME_OFFSET + NAME_BYTES;
pub const IP_ADDR_OFFSET: usize = 1;
pub const PORT_NUM_OFFSET: usize = IP_ADDR_OFFSET + IPV6_SIZE;

pub enum Message {
    UserInfoMessage(UserInfo),
    FileSendRequest(UserInfo),
}
