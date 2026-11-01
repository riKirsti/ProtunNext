//! One selected virtual subnet shared by TUN setup, private sockets, DNS,
//! routing, probes and adapter reporting. Default matches published v3.0.5.
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Addresses {
    subnet: u8,
}
impl Default for Addresses {
    fn default() -> Self {
        Self { subnet: 2 }
    }
}
impl Addresses {
    pub fn new(subnet: u8) -> crate::Result<Self> {
        if subnet < 2 {
            return Err(crate::Error::Invalid("virtual subnet must be 2..255"));
        }
        Ok(Self { subnet })
    }
    pub const fn subnet(self) -> u8 {
        self.subnet
    }
    pub const fn v4(self, host: u8) -> Ipv4Addr {
        Ipv4Addr::new(10, self.subnet, 0, host)
    }
    pub const fn v6(self, host: u16) -> Ipv6Addr {
        Ipv6Addr::new(0x2a07, 0xb944, 0, 0, 0, 0, self.subnet as u16, host)
    }
    pub fn http(self) -> SocketAddr {
        SocketAddr::new(self.v4(3).into(), 80)
    }
}
