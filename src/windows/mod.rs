//! Windows platform implementation. Only an explicit connection constructor
//! may create/configure a VPN adapter; clock/unit checks make no network changes.
mod device;
mod interface;
mod monitor;
mod net;
mod transport;
mod wait;
use crate::Result;
pub(crate) use interface::Interface;
pub(crate) use interface::delete_owned_routes;
pub(crate) use monitor::Monitor;
pub(crate) use net::available as network_available;
use std::{io, time::Duration};
pub(crate) use transport::run_owned;
pub(crate) use wait::EventBridge;

pub fn boottime() -> Result<Duration> {
    let mut ticks = 0u64;
    // SAFETY: correctly sized writable output, 100ns interrupt-time units.
    // QueryInterruptTimePrecise includes suspend/hibernate time.
    unsafe {
        windows_sys::Win32::System::WindowsProgramming::QueryInterruptTimePrecise(&mut ticks);
    }
    Ok(Duration::new(
        ticks / 10_000_000,
        ((ticks % 10_000_000) * 100) as u32,
    ))
}
fn status(code: u32) -> io::Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(code as i32))
    }
}

/// Low-level Windows transport/crypto worker without a system VPN interface.
/// Useful for protocol verification; handshake state is not a full VPN result.
pub fn start_without_tun(
    config: crate::config::WireguardConfig,
    transport: crate::transport::Transport,
) -> Result<crate::connection::Connection> {
    crate::connection::Connection::start_owned(None, config, String::new(), transport, None, None)
}
