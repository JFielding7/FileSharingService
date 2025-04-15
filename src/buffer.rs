use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::net::IpAddr::{V4, V6};

const IPV4_CODE: u8 = 0;
const IPV6_CODE: u8 = 1;

pub struct Buffer {
    curr: usize,
    bytes: Vec<u8>
}

macro_rules! get_bytes {
    ($buffer:expr, $n:expr) => {{
        let mut bytes: [u8; $n] = [0; $n];
        for i in 0..$n {
            bytes[i] = $buffer.get_byte();
        }
        bytes
    }};
}

impl Buffer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { curr: 0, bytes }
    }

    pub fn get_byte(&mut self) -> u8 {
        let byte = self.bytes[self.curr];
        self.curr += 1;
        byte
    }

    pub fn get_name(&mut self) -> String {
        // TODO: account for overflow
        let name = self.bytes[self.curr..].iter().take_while(|&&byte| byte != 0);
        String::from_utf8(name.map(|byte| *byte).collect()).unwrap()
    }

    fn get_ip_addr(&mut self) -> Option<IpAddr> {
        match self.get_byte() {
            IPV4_CODE => Some(V4(Ipv4Addr::from(get_bytes!(self, 4)))),
            IPV6_CODE => Some(V6(Ipv6Addr::from(get_bytes!(self, 16)))),
            _ => None
        }
    }

    fn get_port_num(&mut self) -> u16 {
        (self.get_byte() as u16) | ((self.get_byte() as u16) << 8)
    }

    pub fn get_socket_addr(&mut self) -> Option<SocketAddr> {
        Some(SocketAddr::new(self.get_ip_addr()?, self.get_port_num()))
    }
}
