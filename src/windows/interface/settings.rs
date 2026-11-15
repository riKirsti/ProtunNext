//! Exact OS rows owned by this interface; settings drop before the device session.
use crate::windows::{device::Session, net};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    DeleteIpForwardEntry2, DeleteUnicastIpAddressEntry, MIB_IPFORWARD_ROW2,
    MIB_UNICASTIPADDRESS_ROW,
};

pub(super) struct Settings {
    pub(super) network: crate::addressing::Addresses,
    pub(super) addresses: Vec<MIB_UNICASTIPADDRESS_ROW>,
    pub(super) routes: Vec<MIB_IPFORWARD_ROW2>,
    pub(super) ipv6: bool,
    pub(super) mtu: u16,
    pub(super) custom_dns: Vec<IpAddr>,
}
impl Settings {
    pub(super) fn add_routes(&mut self, session: &Session, ipv6: bool) -> io::Result<()> {
        let (gateway, empty, half, bits): (IpAddr, IpAddr, IpAddr, u8) = if ipv6 {
            (
                self.network.v6(1).into(),
                Ipv6Addr::UNSPECIFIED.into(),
                "8000::".parse().expect("constant"),
                128,
            )
        } else {
            (
                self.network.v4(1).into(),
                Ipv4Addr::UNSPECIFIED.into(),
                Ipv4Addr::new(128, 0, 0, 0).into(),
                32,
            )
        };
        self.routes
            .push(net::route(session.luid, gateway, bits, empty)?);
        self.routes
            .push(net::route(session.luid, empty, 1, gateway)?);
        self.routes
            .push(net::route(session.luid, half, 1, gateway)?);
        Ok(())
    }
    pub(super) fn enable_ipv6(&mut self, session: &Session) -> io::Result<()> {
        self.addresses
            .push(net::address(session.luid, self.network.v6(2).into(), 128)?);
        if let Err(error) =
            net::mtu(session.luid, true, self.mtu).and_then(|()| self.add_routes(session, true))
        {
            let _ = self.remove_family(true);
            return Err(error);
        }
        self.ipv6 = true;
        Ok(())
    }
    pub(super) fn remove_family(&mut self, ipv6: bool) -> io::Result<()> {
        let family = if ipv6 {
            windows_sys::Win32::Networking::WinSock::AF_INET6
        } else {
            windows_sys::Win32::Networking::WinSock::AF_INET
        };
        for index in (0..self.routes.len()).rev() {
            // SAFETY: Windows sockaddr's family tag is common to both members.
            if unsafe { self.routes[index].DestinationPrefix.Prefix.si_family } == family {
                // SAFETY: exact row successfully created and retained by us.
                crate::windows::status(unsafe { DeleteIpForwardEntry2(&self.routes[index]) })?;
                self.routes.remove(index);
            }
        }
        for index in (0..self.addresses.len()).rev() {
            // SAFETY: Windows sockaddr's family tag is common to both members.
            if unsafe { self.addresses[index].Address.si_family } == family {
                // SAFETY: exact row successfully created and retained by us.
                crate::windows::status(unsafe {
                    DeleteUnicastIpAddressEntry(&self.addresses[index])
                })?;
                self.addresses.remove(index);
            }
        }
        Ok(())
    }
    pub(super) fn set_dns(&self, session: &Session) -> io::Result<()> {
        let mut servers = self.custom_dns.clone();
        servers.push(self.network.v4(1).into());
        if self.ipv6 {
            servers.push(self.network.v6(1).into());
        }
        net::dns(session.guid, false, &servers)?;
        if self.ipv6 {
            net::dns(session.guid, true, &servers)?;
        }
        Ok(())
    }
    pub(super) fn clear_routes(&mut self) {
        for row in self.routes.drain(..) {
            // SAFETY: only exact route rows successfully created by this owner.
            unsafe {
                DeleteIpForwardEntry2(&row);
            }
        }
    }
}
impl Drop for Settings {
    fn drop(&mut self) {
        self.clear_routes();
        for row in self.addresses.drain(..) {
            // SAFETY: only exact address rows successfully created by this owner.
            unsafe {
                DeleteUnicastIpAddressEntry(&row);
            }
        }
    }
}
