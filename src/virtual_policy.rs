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
fn classify(bytes: &[u8], addresses: crate::addressing::Addresses) -> Address {
    let last = match bytes {
        [10, subnet, 0, last] if *subnet == addresses.subnet() => Some(*last as u16),
        [
            0x2a,
            0x07,
            0xb9,
            0x44,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            subnet,
            a,
            b,
        ] if *subnet == addresses.subnet() => Some(u16::from_be_bytes([*a, *b])),
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
fn addresses(packet: &[u8], selected: crate::addressing::Addresses) -> Option<(Address, Address)> {
    match packet.first()? >> 4 {
        4 if packet.len() >= 20 => Some((
            classify(&packet[12..16], selected),
            classify(&packet[16..20], selected),
        )),
        6 if packet.len() >= 40 => Some((
            classify(&packet[8..24], selected),
            classify(&packet[24..40], selected),
        )),
        _ => None,
    }
}
pub fn outbound(packet: &[u8]) -> Route {
    outbound_with(packet, Default::default())
}
pub fn outbound_with(packet: &[u8], selected: crate::addressing::Addresses) -> Route {
    let Some((source, destination)) = addresses(packet, selected) else {
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
    inbound_with(packet, Default::default())
}
pub fn inbound_with(packet: &[u8], selected: crate::addressing::Addresses) -> Route {
    let Some((source, destination)) = addresses(packet, selected) else {
        return Route::Drop;
    };
    if matches!(source, Address::Client | Address::Local)
        || matches!(destination, Address::Local | Address::Unclaimed)
    {
        return Route::Drop;
    }
    // A tunnel-originated packet may only be delivered to the customer; never
    // reflect it into the VPN or permit access to the local HTTP listener.
    if destination == Address::Client {
        Route::Customer
    } else {
        // Upstream route.rs defaults to Tunnel, and router.rs drops any route
        // back to its origin. An unrelated Internet destination cannot reach TUN.
        Route::Drop
    }
}
pub fn inbound_allowed(packet: &[u8], selected: crate::addressing::Addresses) -> bool {
    addresses(packet, selected).is_some_and(|(source, destination)| {
        !matches!(source, Address::Client | Address::Local) && destination != Address::Local
    })
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
        assert_eq!(inbound(&v4([1, 1, 1, 1], [8, 8, 8, 8])), Route::Drop);
        let addresses = crate::addressing::Addresses::new(37).unwrap();
        assert_eq!(
            outbound_with(&v4([10, 37, 0, 2], [10, 37, 0, 3]), addresses),
            Route::LocalService
        );
        assert_eq!(
            inbound_with(&v4([1, 1, 1, 1], [10, 37, 0, 2]), addresses),
            Route::Customer
        );
        assert_eq!(
            inbound_with(&v4([1, 1, 1, 1], [10, 2, 0, 2]), addresses),
            Route::Drop
        );
    }
}
