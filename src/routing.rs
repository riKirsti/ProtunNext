//! Single-peer cryptokey routing: outbound destination and inbound source ACL.
//! https://www.wireguard.com/#cryptokey-routing
use crate::{Error, Result};
use std::net::IpAddr;
pub struct AllowedIps {
    ipv4: Vec<(u32, u32)>,
    ipv6: Vec<(u128, u128)>,
    all_v4: bool,
    all_v6: bool,
}
impl AllowedIps {
    pub fn new(cidrs: &[String]) -> Result<Self> {
        if cidrs.is_empty() || cidrs.len() > 4096 {
            return Err(Error::Invalid("AllowedIPs count"));
        }
        let mut set = Self {
            ipv4: Vec::new(),
            ipv6: Vec::new(),
            all_v4: false,
            all_v6: false,
        };
        for cidr in cidrs {
            let (address, prefix) = cidr
                .split_once('/')
                .ok_or(Error::Invalid("AllowedIPs CIDR"))?;
            let address: IpAddr = address
                .parse()
                .map_err(|_| Error::Invalid("AllowedIPs address"))?;
            let prefix: u8 = prefix
                .parse()
                .map_err(|_| Error::Invalid("AllowedIPs prefix"))?;
            match address {
                IpAddr::V4(ip) if prefix <= 32 => {
                    if prefix == 0 {
                        set.all_v4 = true;
                    } else {
                        let mask = u32::MAX << (32 - prefix);
                        set.ipv4.push((u32::from(ip) & mask, mask));
                    }
                }
                IpAddr::V6(ip) if prefix <= 128 => {
                    if prefix == 0 {
                        set.all_v6 = true;
                    } else {
                        let mask = u128::MAX << (128 - prefix);
                        set.ipv6.push((u128::from(ip) & mask, mask));
                    }
                }
                _ => return Err(Error::Invalid("AllowedIPs prefix")),
            }
        }
        Ok(set)
    }
    fn accepts(&self, packet: &[u8], source: bool) -> bool {
        match packet.first().map(|b| b >> 4) {
            Some(4) if packet.len() >= 20 => {
                let header = (packet[0] & 15) as usize * 4;
                let total = u16::from_be_bytes([packet[2], packet[3]]) as usize;
                if header < 20 || header > total || total != packet.len() {
                    return false;
                }
                if self.all_v4 {
                    return true;
                }
                let offset = if source { 12 } else { 16 };
                let ip = u32::from_be_bytes(
                    packet[offset..offset + 4]
                        .try_into()
                        .expect("checked header"),
                );
                self.ipv4
                    .iter()
                    .any(|(network, mask)| ip & mask == *network)
            }
            Some(6) if packet.len() >= 40 => {
                let total = 40 + u16::from_be_bytes([packet[4], packet[5]]) as usize;
                if total != packet.len() {
                    return false;
                }
                if self.all_v6 {
                    return true;
                }
                let offset = if source { 8 } else { 24 };
                let ip = u128::from_be_bytes(
                    packet[offset..offset + 16]
                        .try_into()
                        .expect("checked header"),
                );
                self.ipv6
                    .iter()
                    .any(|(network, mask)| ip & mask == *network)
            }
            _ => false,
        }
    }
    pub fn outbound(&self, packet: &[u8]) -> bool {
        self.accepts(packet, false)
    }
    pub fn inbound(&self, packet: &[u8]) -> bool {
        self.accepts(packet, true)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restricts_destinations_sources_and_ip_family_without_packet_allocations() {
        let allowed = AllowedIps::new(&["192.0.2.99/24".into(), "2001:db8:1::/48".into()]).unwrap();
        let mut ipv4 = [0; 20];
        ipv4[0] = 0x45;
        ipv4[2..4].copy_from_slice(&20u16.to_be_bytes());
        ipv4[12..16].copy_from_slice(&[198, 51, 100, 1]);
        ipv4[16..20].copy_from_slice(&[192, 0, 2, 2]);
        assert!(allowed.outbound(&ipv4));
        assert!(!allowed.inbound(&ipv4));
        let mut ipv6 = [0; 40];
        ipv6[0] = 0x60;
        ipv6[8..24].copy_from_slice(
            &"2001:db8:1::2"
                .parse::<std::net::Ipv6Addr>()
                .unwrap()
                .octets(),
        );
        ipv6[24..40].copy_from_slice(
            &"2001:db8:2::2"
                .parse::<std::net::Ipv6Addr>()
                .unwrap()
                .octets(),
        );
        assert!(allowed.inbound(&ipv6));
        assert!(!allowed.outbound(&ipv6));
        let all = AllowedIps::new(&["0.0.0.0/0".into(), "::/0".into()]).unwrap();
        assert!(all.inbound(&ipv4) && all.outbound(&ipv6));
        assert!(!all.inbound(&ipv4[..19]));
        ipv4[0] = 0x44;
        assert!(!all.outbound(&ipv4));
        assert!(AllowedIps::new(&["::/129".into()]).is_err());
    }
}
