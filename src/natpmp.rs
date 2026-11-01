//! RFC 6886 wire codec. Entitlement and mapping renewal require a live tunnel.
use crate::{Error, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Udp,
    Tcp,
}
impl Transport {
    fn opcode(self) -> u8 {
        match self {
            Self::Udp => 1,
            Self::Tcp => 2,
        }
    }
}
pub fn request(
    protocol: Transport,
    private_port: u16,
    public_port: u16,
    lifetime: u32,
) -> Result<[u8; 12]> {
    if private_port == 0 {
        return Err(Error::Invalid("zero private port"));
    }
    let mut p = [0; 12];
    p[1] = protocol.opcode();
    p[4..6].copy_from_slice(&private_port.to_be_bytes());
    p[6..8].copy_from_slice(&public_port.to_be_bytes());
    p[8..].copy_from_slice(&lifetime.to_be_bytes());
    Ok(p)
}
#[derive(Debug, PartialEq, Eq)]
pub struct Mapping {
    pub epoch: u32,
    pub private_port: u16,
    pub public_port: u16,
    pub lifetime: u32,
}
pub fn response(bytes: &[u8], protocol: Transport, expected_private: u16) -> Result<Mapping> {
    if bytes.len() != 16 || bytes[0] != 0 || bytes[1] != 128 + protocol.opcode() {
        return Err(Error::Invalid("NAT-PMP reply header"));
    }
    let result = u16::from_be_bytes(bytes[2..4].try_into().expect("header length"));
    if result != 0 {
        return Err(Error::Protocol(format!("NAT-PMP refused: {result}")));
    }
    let private_port = u16::from_be_bytes(bytes[8..10].try_into().expect("header length"));
    if private_port != expected_private {
        return Err(Error::Invalid("NAT-PMP transaction mismatch"));
    }
    Ok(Mapping {
        epoch: u32::from_be_bytes(bytes[4..8].try_into().expect("header length")),
        private_port,
        public_port: u16::from_be_bytes(bytes[10..12].try_into().expect("header length")),
        lifetime: u32::from_be_bytes(bytes[12..16].try_into().expect("header length")),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rfc_request_and_refusal() {
        assert_eq!(
            request(Transport::Tcp, 1234, 0, 60).unwrap(),
            [0, 2, 0, 0, 4, 210, 0, 0, 0, 0, 0, 60]
        );
        let mut r = [0; 16];
        r[1] = 130;
        r[8..10].copy_from_slice(&1234u16.to_be_bytes());
        assert!(response(&r, Transport::Tcp, 1234).is_ok());
        r[3] = 2;
        assert!(response(&r, Transport::Tcp, 1234).is_err());
    }
}
