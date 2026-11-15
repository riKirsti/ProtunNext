//! OS settings are owned by the adapter that this process freshly creates.
//! Cleanup never enumerates/removes another application's adapters or routes.
mod settings;
use super::{device::Session, net};
use crate::{
    Result,
    api::{
        types::IpAddress,
        windows::{AdapterConfig, ProTunFatalError, SocketConfig},
    },
};
use settings::Settings;
use std::{
    io,
    sync::{Arc, Mutex, OnceLock, Weak},
};

static OWNED: OnceLock<Mutex<Vec<Weak<Interface>>>> = OnceLock::new();
pub(crate) struct Interface {
    pub(crate) addresses: crate::addressing::Addresses,
    settings: Mutex<Settings>, // dropped before session so its IDs remain valid.
    pub(crate) session: Arc<Session>,
    pub(crate) socket: SocketConfig,
}
impl Interface {
    pub(crate) fn create(
        config: AdapterConfig,
        socket: SocketConfig,
    ) -> std::result::Result<Arc<Self>, ProTunFatalError> {
        super::version::require_supported()
            .map_err(|error| ProTunFatalError::WintunInterfaceCreationFailed(error.to_string()))?;
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
        let (addresses, address) =
            select_addresses(|addresses| net::address(session.luid, addresses.v4(2).into(), 32))?;
        let mut settings = Settings {
            network: addresses,
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
        settings.addresses.push(address);
        net::mtu(session.luid, false, config.mtu)?;
        settings.add_routes(&session, false)?;
        if config.is_ipv6_enabled && settings.enable_ipv6(&session).is_err() {
            // The pinned Windows setup also permits a working IPv4 interface
            // when the OS cannot configure its optional IPv6 address.
            log::warn!("Windows IPv6 address setup unavailable; IPv4 interface retained");
        }
        settings.set_dns(&session)?;
        let interface = Arc::new(Self {
            addresses,
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

fn select_addresses<T>(
    mut assign: impl FnMut(crate::addressing::Addresses) -> io::Result<T>,
) -> io::Result<(crate::addressing::Addresses, T)> {
    let mut last = io::Error::from(io::ErrorKind::AddrNotAvailable);
    for subnet in 2..=u8::MAX {
        let addresses = crate::addressing::Addresses::new(subnet).expect("validated range");
        match assign(addresses) {
            Ok(value) => return Ok((addresses, value)),
            Err(error) => last = error,
        }
    }
    Err(last)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failed_default_address_uses_next_subnet_and_total_failure_is_bounded() {
        let mut attempted = Vec::new();
        let (addresses, value) = select_addresses(|addresses| {
            attempted.push(addresses.subnet());
            if addresses.subnet() == 2 {
                Err(io::ErrorKind::AddrInUse.into())
            } else {
                Ok(42)
            }
        })
        .unwrap();
        assert_eq!(attempted, [2, 3]);
        assert_eq!(addresses.v4(2).to_string(), "10.3.0.2");
        assert_eq!(addresses.v6(1).to_string(), "2a07:b944::3:1");
        assert_eq!(value, 42);
        let mut count = 0;
        assert!(
            select_addresses::<()>(|_| {
                count += 1;
                Err(io::ErrorKind::AddrNotAvailable.into())
            })
            .is_err()
        );
        assert_eq!(count, 254);
    }
}
