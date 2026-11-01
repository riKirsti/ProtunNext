//! Native IP Helper operations. Route selection is performed once per outer
//! socket, excludes our TUN and never narrows a Windows SOCKET to a Unix fd.
use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    os::windows::io::AsRawSocket,
};
use windows_sys::Win32::{
    NetworkManagement::{IpHelper::*, Ndis::NET_LUID_LH},
    Networking::WinSock::*,
};

pub(super) fn sockaddr(ip: IpAddr) -> SOCKADDR_INET {
    // SAFETY: integer/union sockaddr storage; the selected family is initialized.
    let mut address: SOCKADDR_INET = unsafe { std::mem::zeroed() };
    match ip {
        IpAddr::V4(ip) => {
            address.Ipv4.sin_family = AF_INET;
            address.Ipv4.sin_addr.S_un.S_addr = u32::from_ne_bytes(ip.octets());
        }
        IpAddr::V6(ip) => {
            address.Ipv6.sin6_family = AF_INET6;
            address.Ipv6.sin6_addr.u.Byte = ip.octets();
        }
    }
    address
}
fn ip(address: &SOCKADDR_INET) -> Option<IpAddr> {
    // SAFETY: Windows rows carry a family tag selecting the active union member.
    unsafe {
        match address.si_family {
            AF_INET => Some(Ipv4Addr::from(address.Ipv4.sin_addr.S_un.S_addr.to_ne_bytes()).into()),
            AF_INET6 => Some(Ipv6Addr::from(address.Ipv6.sin6_addr.u.Byte).into()),
            _ => None,
        }
    }
}
fn matches_prefix(address: IpAddr, prefix: IpAddr, bits: u8) -> bool {
    match (address, prefix) {
        (IpAddr::V4(a), IpAddr::V4(b)) if bits <= 32 => {
            bits == 0 || (u32::from(a) ^ u32::from(b)) >> (32 - bits) == 0
        }
        (IpAddr::V6(a), IpAddr::V6(b)) if bits <= 128 => {
            bits == 0 || (u128::from(a) ^ u128::from(b)) >> (128 - bits) == 0
        }
        _ => false,
    }
}
struct Table(*mut MIB_IPFORWARD_TABLE2);
impl Table {
    fn read() -> io::Result<Self> {
        let mut table = std::ptr::null_mut();
        // SAFETY: valid output pointer, OS allocates a matching table.
        super::status(unsafe { GetIpForwardTable2(AF_UNSPEC, &mut table) })?;
        if table.is_null() {
            return Err(io::ErrorKind::InvalidData.into());
        }
        Ok(Self(table))
    }
    fn rows(&self) -> io::Result<&[MIB_IPFORWARD_ROW2]> {
        // SAFETY: pointer returned by GetIpForwardTable2, valid until Table drop.
        let count = unsafe { (*self.0).NumEntries } as usize;
        if count > 1_000_000 {
            return Err(io::ErrorKind::InvalidData.into());
        }
        // SAFETY: Windows allocates NumEntries rows at the properly aligned
        // Table field, including the header's padding. Borrow cannot outlive self.
        Ok(
            unsafe {
                std::slice::from_raw_parts(std::ptr::addr_of!((*self.0).Table).cast(), count)
            },
        )
    }
}
impl Drop for Table {
    fn drop(&mut self) {
        // SAFETY: exactly the allocation returned by GetIpForwardTable2.
        unsafe {
            FreeMibTable(self.0.cast());
        }
    }
}
pub(super) fn physical_interface(destination: IpAddr, excluded: u32) -> io::Result<u32> {
    let table = Table::read()?;
    let mut best: Option<(u8, u64, u32)> = None;
    for row in table.rows()? {
        if row.InterfaceIndex == excluded || (row.Loopback != 0 && !destination.is_loopback()) {
            continue;
        }
        let Some(prefix) = ip(&row.DestinationPrefix.Prefix) else {
            continue;
        };
        let length = row.DestinationPrefix.PrefixLength;
        if !matches_prefix(destination, prefix, length) {
            continue;
        }
        // SAFETY: output row is initialized, family/index select the interface.
        let mut interface: MIB_IPINTERFACE_ROW = unsafe { std::mem::zeroed() };
        // SAFETY: writable appropriately sized row for the initialization API.
        unsafe {
            InitializeIpInterfaceEntry(&mut interface);
        }
        interface.Family = if destination.is_ipv4() {
            AF_INET
        } else {
            AF_INET6
        };
        interface.InterfaceIndex = row.InterfaceIndex;
        // SAFETY: row contains the requested interface identity and output space.
        if unsafe { GetIpInterfaceEntry(&mut interface) } != 0 || interface.Connected == 0 {
            continue;
        }
        let metric = u64::from(row.Metric) + u64::from(interface.Metric);
        if best.is_none_or(|old| length > old.0 || (length == old.0 && metric < old.1)) {
            best = Some((length, metric, row.InterfaceIndex));
        }
    }
    best.map(|(_, _, index)| index)
        .ok_or(io::ErrorKind::NetworkUnreachable.into())
}
pub(super) fn protect(
    socket: &socket2::Socket,
    endpoint: SocketAddr,
    excluded: u32,
) -> io::Result<()> {
    let index = physical_interface(endpoint.ip(), excluded)?;
    let (level, option, value) = if endpoint.is_ipv4() {
        (IPPROTO_IP, IP_UNICAST_IF, index.to_be())
    } else {
        (IPPROTO_IPV6, IPV6_UNICAST_IF, index)
    };
    let handle =
        usize::try_from(socket.as_raw_socket()).map_err(|_| io::ErrorKind::InvalidInput)?;
    // SAFETY: native-width SOCKET and a correctly sized scalar. IPv4 uses
    // network order; IPv6 uses host order, per Microsoft's socket-option API.
    let result = unsafe {
        setsockopt(
            handle,
            level,
            option,
            (&value as *const u32).cast(),
            size_of::<u32>() as i32,
        )
    };
    if result == SOCKET_ERROR {
        // SAFETY: retrieves this thread's failed Winsock call immediately.
        return Err(io::Error::from_raw_os_error(unsafe { WSAGetLastError() }));
    }
    Ok(())
}
pub(crate) fn available(excluded: u32) -> bool {
    physical_interface(Ipv4Addr::new(1, 1, 1, 1).into(), excluded).is_ok()
        || physical_interface("2001:4860:4860::8888".parse().expect("constant"), excluded).is_ok()
}
pub(super) fn address(
    luid: NET_LUID_LH,
    value: IpAddr,
    prefix: u8,
) -> io::Result<MIB_UNICASTIPADDRESS_ROW> {
    // SAFETY: initialized plain Windows row, then filled by its initializer.
    let mut row = unsafe { std::mem::zeroed::<MIB_UNICASTIPADDRESS_ROW>() };
    // SAFETY: valid writable output object.
    unsafe {
        InitializeUnicastIpAddressEntry(&mut row);
    }
    row.InterfaceLuid = luid;
    row.Address = sockaddr(value);
    row.OnLinkPrefixLength = prefix;
    row.PrefixOrigin = IpPrefixOriginManual;
    row.SuffixOrigin = IpSuffixOriginManual;
    row.ValidLifetime = u32::MAX;
    row.PreferredLifetime = u32::MAX;
    // SAFETY: complete row affects only the freshly owned Wintun interface.
    super::status(unsafe { CreateUnicastIpAddressEntry(&row) })?;
    Ok(row)
}
pub(super) fn route(
    luid: NET_LUID_LH,
    prefix: IpAddr,
    bits: u8,
    gateway: IpAddr,
) -> io::Result<MIB_IPFORWARD_ROW2> {
    // SAFETY: initialized plain Windows row, then filled by its initializer.
    let mut row = unsafe { std::mem::zeroed::<MIB_IPFORWARD_ROW2>() };
    // SAFETY: valid writable output object.
    unsafe {
        InitializeIpForwardEntry(&mut row);
    }
    row.InterfaceLuid = luid;
    row.DestinationPrefix = IP_ADDRESS_PREFIX {
        Prefix: sockaddr(prefix),
        PrefixLength: bits,
    };
    row.NextHop = sockaddr(gateway);
    row.Protocol = MIB_IPPROTO_NETMGMT;
    row.Metric = 0;
    row.ValidLifetime = u32::MAX;
    row.PreferredLifetime = u32::MAX;
    // SAFETY: complete route row, only owned interface and specified prefix.
    super::status(unsafe { CreateIpForwardEntry2(&row) })?;
    Ok(row)
}
pub(super) fn mtu(luid: NET_LUID_LH, ipv6: bool, mtu: u16) -> io::Result<()> {
    // SAFETY: integer-only output structure initialized before use.
    let mut row = unsafe { std::mem::zeroed::<MIB_IPINTERFACE_ROW>() };
    // SAFETY: live output buffer for Windows initializer.
    unsafe {
        InitializeIpInterfaceEntry(&mut row);
    }
    row.InterfaceLuid = luid;
    row.Family = if ipv6 { AF_INET6 } else { AF_INET };
    // SAFETY: selector fields initialized; query fills the remainder.
    super::status(unsafe { GetIpInterfaceEntry(&mut row) })?;
    row.NlMtu = u32::from(mtu);
    row.UseAutomaticMetric = 0;
    row.Metric = 3;
    row.DadTransmits = 0;
    // SAFETY: preserves all queried fields except owned-interface MTU/metric/DAD.
    super::status(unsafe { SetIpInterfaceEntry(&mut row) })
}
pub(super) fn dns(
    guid: windows_sys::core::GUID,
    ipv6: bool,
    addresses: &[IpAddr],
) -> io::Result<()> {
    let mut servers: Vec<u16> = addresses
        .iter()
        .filter(|ip| ip.is_ipv6() == ipv6)
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
        .encode_utf16()
        .chain(Some(0))
        .collect();
    // SAFETY: integer/pointer structure; all unspecified flags/fields stay zero.
    let mut settings = unsafe { std::mem::zeroed::<DNS_INTERFACE_SETTINGS>() };
    settings.Version = DNS_INTERFACE_SETTINGS_VERSION1;
    settings.Flags = u64::from(DNS_SETTING_NAMESERVER | if ipv6 { DNS_SETTING_IPV6 } else { 0 });
    settings.NameServer = servers.as_mut_ptr();
    // SAFETY: terminated server buffer lives through the synchronous API call.
    super::status(unsafe { SetInterfaceDnsSettings(guid, &settings) })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_sockaddrs_preserve_both_address_families_and_prefix_boundaries() {
        for value in ["10.2.0.2", "2a07:b944::2:2"] {
            let address: IpAddr = value.parse().unwrap();
            assert_eq!(ip(&sockaddr(address)), Some(address));
            assert!(matches_prefix(
                address,
                address,
                if address.is_ipv4() { 32 } else { 128 }
            ));
        }
        assert!(matches_prefix(
            "192.168.7.2".parse().unwrap(),
            "0.0.0.0".parse().unwrap(),
            0
        ));
        assert!(!matches_prefix(
            "192.168.7.2".parse().unwrap(),
            "192.168.8.0".parse().unwrap(),
            24
        ));
        assert!(!matches_prefix(
            "10.2.0.2".parse().unwrap(),
            "::".parse().unwrap(),
            0
        ));
        assert!(!matches_prefix(
            "::1".parse().unwrap(),
            "::".parse().unwrap(),
            129
        ));
    }
}
