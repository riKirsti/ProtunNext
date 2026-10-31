//! Fixed virtual-network isolation from v3.0.5's published routing/firewall
//! rules. This is separate from configurable WireGuard AllowedIPs filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Tunnel,
    Customer,
    LocalService,
    Drop,
}
#[derive(PartialEq, Eq)]
enum Address {
    Client,
    Gateway,
    Local,
    Unclaimed,
    Internet,
}
fn classify(bytes: &[u8]) -> Address {
    let last = match bytes {
        [10, 2, 0, last] => Some(*last as u16),
        [0x2a, 0x07, 0xb9, 0x44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, a, b] => {
            Some(u16::from_be_bytes([*a, *b]))
        }
        _ => None,
    };
    match last {
        Some(1) => Address::Gateway,
        Some(2) => Address::Client,
        Some(3) => Address::Local,
        Some(_) => Address::Unclaimed,
        None => Address::Internet,
    }
}
fn addresses(packet: &[u8]) -> Option<(Address, Address)> {
    match packet.first()? >> 4 {
        4 if packet.len() >= 20 => Some((classify(&packet[12..16]), classify(&packet[16..20]))),
        6 if packet.len() >= 40 => Some((classify(&packet[8..24]), classify(&packet[24..40]))),
        _ => None,
    }
}
pub fn outbound(packet: &[u8]) -> Route {
    let Some((source, destination)) = addresses(packet) else {
        return Route::Drop;
    };
    if source == Address::Local {
        return Route::Drop;
    }
    match destination {
        Address::Client | Address::Unclaimed => Route::Drop,
        Address::Local => Route::LocalService,
        _ => Route::Tunnel,
    }
}
pub fn inbound(packet: &[u8]) -> Route {
    let Some((source, destination)) = addresses(packet) else {
        return Route::Drop;
    };
    if matches!(source, Address::Client | Address::Local)
        || matches!(destination, Address::Local | Address::Unclaimed)
    {
        return Route::Drop;
    }
    // A tunnel-originated packet may only be delivered to the customer; never
    // reflect it into the VPN or permit access to the local HTTP listener.
    if destination == Address::Gateway {
        Route::Drop
    } else {
        Route::Customer
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn v4(src: [u8; 4], dst: [u8; 4]) -> Vec<u8> {
        let mut p = vec![0; 20];
        p[0] = 0x45;
        p[12..16].copy_from_slice(&src);
        p[16..20].copy_from_slice(&dst);
        p
    }
    #[test]
    fn local_addresses_and_unclaimed_subnet_never_escape() {
        assert_eq!(
            outbound(&v4([10, 2, 0, 2], [10, 2, 0, 3])),
            Route::LocalService
        );
        assert_eq!(outbound(&v4([10, 2, 0, 3], [1, 1, 1, 1])), Route::Drop);
        assert_eq!(outbound(&v4([10, 2, 0, 2], [10, 2, 0, 4])), Route::Drop);
        assert_eq!(outbound(&v4([10, 2, 0, 2], [10, 2, 0, 2])), Route::Drop);
        assert_eq!(inbound(&v4([10, 2, 0, 3], [10, 2, 0, 2])), Route::Drop);
        assert_eq!(inbound(&v4([1, 1, 1, 1], [10, 2, 0, 3])), Route::Drop);
        assert_eq!(inbound(&v4([1, 1, 1, 1], [10, 2, 0, 2])), Route::Customer);
        assert_eq!(outbound(&v4([10, 2, 0, 2], [10, 2, 0, 1])), Route::Tunnel);
    }
}
