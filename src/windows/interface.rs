//! OS settings are owned by the adapter that this process freshly creates.
//! Cleanup never enumerates/removes another application's adapters or routes.
use super::{device::Session, net};
use crate::{
    Result,
    api::{
        types::IpAddress,
        windows::{AdapterConfig, ProTunFatalError, SocketConfig},
    },
};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::{Arc, Mutex, OnceLock, Weak},
};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    DeleteIpForwardEntry2, DeleteUnicastIpAddressEntry, MIB_IPFORWARD_ROW2,
    MIB_UNICASTIPADDRESS_ROW,
};

static OWNED: OnceLock<Mutex<Vec<Weak<Interface>>>> = OnceLock::new();
pub(crate) struct Interface {
    settings: Mutex<Settings>, // dropped before session so its IDs remain valid.
    pub(crate) session: Arc<Session>,
    pub(crate) socket: SocketConfig,
}
struct Settings {
    addresses: Vec<MIB_UNICASTIPADDRESS_ROW>,
    routes: Vec<MIB_IPFORWARD_ROW2>,
    ipv6: bool,
    mtu: u16,
    custom_dns: Vec<IpAddr>,
}
impl Interface {
    pub(crate) fn create(
        config: AdapterConfig,
        socket: SocketConfig,
    ) -> std::result::Result<Arc<Self>, ProTunFatalError> {
        if config.mtu < 76 || config.mtu > 65503 || (config.is_ipv6_enabled && config.mtu < 1280) {
            return Err(ProTunFatalError::WintunIpAddressSetupFailed(
                "Windows interface MTU".into(),
            ));
        }
        let session = Session::create(config.buffer_size_bytes).map_err(|error| {
            use super::device::CreateError;
            match error {
                CreateError::Library(error) => {
                    ProTunFatalError::WintunLibraryLoadingFailed(error.to_string())
                }
                CreateError::Adapter(error) => {
                    ProTunFatalError::WintunInterfaceCreationFailed(error.to_string())
                }
                CreateError::Index(error) => {
                    ProTunFatalError::WintunAdapterIndexFetchFailed(error.to_string())
                }
                CreateError::Session(error) => {
                    ProTunFatalError::WintunSessionCreationFailed(error.to_string())
                }
            }
        })?;
        let mut settings = Settings {
            addresses: Vec::new(),
            routes: Vec::new(),
            ipv6: false,
            mtu: config.mtu,
            custom_dns: config
                .custom_dns_server_ips
                .into_iter()
                .map(|ip| ip.0)
                .collect(),
        };
        // Keep the standard subnet until the alternate-subnet integration is
        // implemented across routing, probes and private control sockets.
        settings.addresses.push(net::address(
            session.luid,
            Ipv4Addr::new(10, 2, 0, 2).into(),
            32,
        )?);
        net::mtu(session.luid, false, config.mtu)?;
        settings.add_routes(&session, false)?;
        if config.is_ipv6_enabled {
            settings.enable_ipv6(&session)?;
        }
        settings.set_dns(&session)?;
        let interface = Arc::new(Self {
            settings: Mutex::new(settings),
            session,
            socket,
        });
        let mut owned = OWNED
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        owned.retain(|weak| weak.strong_count() > 0);
        owned.push(Arc::downgrade(&interface));
        Ok(interface)
    }
    pub(crate) fn set_ipv6(&self, enabled: bool) -> Result<bool> {
        let mut settings = self.settings.lock().unwrap_or_else(|e| e.into_inner());
        if enabled == settings.ipv6 {
            return Ok(true);
        }
        if enabled {
            if settings.mtu < 1280 {
                return Err(crate::Error::Invalid("IPv6 requires MTU >= 1280"));
            }
            settings.enable_ipv6(&self.session)?;
        } else {
            settings.remove_family(true)?;
            settings.ipv6 = false;
        }
        settings.set_dns(&self.session)?;
        Ok(true)
    }
    pub(crate) fn set_dns(&self, dns: Vec<IpAddress>) -> Result<bool> {
        let mut settings = self.settings.lock().unwrap_or_else(|e| e.into_inner());
        let previous = std::mem::replace(
            &mut settings.custom_dns,
            dns.into_iter().map(|ip| ip.0).collect(),
        );
        if let Err(error) = settings.set_dns(&self.session) {
            settings.custom_dns = previous;
            let _ = settings.set_dns(&self.session);
            return Err(error.into());
        }
        Ok(true)
    }
    pub(crate) fn ipv6(&self) -> bool {
        self.settings.lock().unwrap_or_else(|e| e.into_inner()).ipv6
    }
    pub(crate) fn read_event(&self) -> io::Result<isize> {
        self.session.read_event()
    }
    pub(crate) fn read(&self, output: &mut [u8]) -> io::Result<usize> {
        self.session.read(output)
    }
    pub(crate) fn write(&self, input: &[u8]) -> io::Result<usize> {
        self.session.write(input)
    }
}
impl Settings {
    fn add_routes(&mut self, session: &Session, ipv6: bool) -> io::Result<()> {
        let (gateway, empty, half, bits): (IpAddr, IpAddr, IpAddr, u8) = if ipv6 {
            (
                "2a07:b944::2:1".parse().expect("constant"),
                Ipv6Addr::UNSPECIFIED.into(),
                "8000::".parse().expect("constant"),
                128,
            )
        } else {
            (
                Ipv4Addr::new(10, 2, 0, 1).into(),
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
    fn enable_ipv6(&mut self, session: &Session) -> io::Result<()> {
        self.addresses.push(net::address(
            session.luid,
            "2a07:b944::2:2".parse().expect("constant"),
            128,
        )?);
        if let Err(error) =
            net::mtu(session.luid, true, self.mtu).and_then(|()| self.add_routes(session, true))
        {
            let _ = self.remove_family(true);
            return Err(error);
        }
        self.ipv6 = true;
        Ok(())
    }
    fn remove_family(&mut self, ipv6: bool) -> io::Result<()> {
        let family = if ipv6 {
            windows_sys::Win32::Networking::WinSock::AF_INET6
        } else {
            windows_sys::Win32::Networking::WinSock::AF_INET
        };
        for index in (0..self.routes.len()).rev() {
            // SAFETY: Windows sockaddr's family tag is common to both members.
            if unsafe { self.routes[index].DestinationPrefix.Prefix.si_family } == family {
                // SAFETY: exact row successfully created and retained by us.
                super::status(unsafe { DeleteIpForwardEntry2(&self.routes[index]) })?;
                self.routes.remove(index);
            }
        }
        for index in (0..self.addresses.len()).rev() {
            // SAFETY: Windows sockaddr's family tag is common to both members.
            if unsafe { self.addresses[index].Address.si_family } == family {
                // SAFETY: exact row successfully created and retained by us.
                super::status(unsafe { DeleteUnicastIpAddressEntry(&self.addresses[index]) })?;
                self.addresses.remove(index);
            }
        }
        Ok(())
    }
    fn set_dns(&self, session: &Session) -> io::Result<()> {
        let mut servers = self.custom_dns.clone();
        servers.push(Ipv4Addr::new(10, 2, 0, 1).into());
        if self.ipv6 {
            servers.push("2a07:b944::2:1".parse().expect("constant"));
        }
        net::dns(session.guid, false, &servers)?;
        if self.ipv6 {
            net::dns(session.guid, true, &servers)?;
        }
        Ok(())
    }
    fn clear_routes(&mut self) {
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
pub(crate) fn delete_owned_routes() {
    let Some(owned) = OWNED.get() else {
        return;
    };
    let mut owned = owned.lock().unwrap_or_else(|e| e.into_inner());
    owned.retain(|weak| {
        if let Some(interface) = weak.upgrade() {
            interface
                .settings
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clear_routes();
            true
        } else {
            false
        }
    });
}
