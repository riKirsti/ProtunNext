//! Linux UDP adapter. Only attaches to a caller-provided TUN fd; never creates
//! interfaces, installs packages, changes routes, changes DNS, or touches sysctls.
pub use crate::transport::{Counters, Transport};
use crate::{
    Error, Result,
    config::WireguardConfig,
    linux_network,
    network::{Datagram, Network, Stream},
    stream::FramedIo,
    tls::TlsIo,
};
use std::{
    fs::File,
    os::fd::{AsRawFd, FromRawFd},
    sync::atomic::AtomicBool,
    time::Duration,
};

pub fn boottime() -> Result<Duration> {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    #[cfg(any(target_os = "linux", target_os = "android"))]
    let clock = libc::CLOCK_BOOTTIME;
    #[cfg(target_vendor = "apple")]
    let clock = libc::CLOCK_MONOTONIC_RAW;
    #[cfg(not(any(target_os = "linux", target_os = "android", target_vendor = "apple")))]
    let clock = libc::CLOCK_MONOTONIC;
    // SAFETY: initialized output object and a supported monotonic clock ID.
    // Linux BOOTTIME and Apple's MONOTONIC_RAW both advance through sleep.
    if unsafe { libc::clock_gettime(clock, &mut ts) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(Duration::new(
        ts.tv_sec
            .try_into()
            .map_err(|_| Error::Invalid("monotonic seconds"))?,
        ts.tv_nsec as u32,
    ))
}
pub(crate) fn duplicate_nonblocking(fd: i32) -> Result<File> {
    // SAFETY: fcntl duplicates the supplied descriptor without taking its ownership.
    let duplicate = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3) };
    if duplicate < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: duplicate is a new owned descriptor from successful fcntl.
    let file = unsafe { File::from_raw_fd(duplicate) };
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let mut info = [0u8; 40];
        // SAFETY: live descriptor, adequately sized Linux ifreq storage for TUNGETIFF.
        if unsafe { libc::ioctl(file.as_raw_fd(), libc::TUNGETIFF, info.as_mut_ptr()) } < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let tun_flags = i16::from_ne_bytes([info[libc::IFNAMSIZ], info[libc::IFNAMSIZ + 1]]) as i32;
        if tun_flags & (libc::IFF_TUN | libc::IFF_NO_PI) != (libc::IFF_TUN | libc::IFF_NO_PI) {
            return Err(Error::Invalid(
                "descriptor must be TUN without packet-info header",
            ));
        }
    }
    #[cfg(target_vendor = "apple")]
    {
        let mut kind = 0i32;
        let mut length = size_of::<i32>() as libc::socklen_t;
        // SAFETY: live descriptor and correctly sized mutable scalar output.
        if unsafe {
            libc::getsockopt(
                file.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_TYPE,
                (&mut kind as *mut i32).cast(),
                &mut length,
            )
        } < 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        if kind != libc::SOCK_DGRAM {
            return Err(Error::Invalid("utun requires a datagram descriptor"));
        }
    }
    // SAFETY: valid owned descriptor and scalar fcntl arguments.
    let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFL) };
    if flags < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: valid descriptor; preserves all flags besides enabling nonblocking.
    if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(file)
}
pub fn run_udp(
    tun_fd: i32,
    config: &WireguardConfig,
    bind_device: &str,
    stop: &AtomicBool,
) -> Result<Counters> {
    run_transport(
        Some(tun_fd),
        config,
        bind_device,
        Transport::Udp,
        stop,
        None,
    )
}
pub fn run_tcp(
    tun_fd: i32,
    config: &WireguardConfig,
    bind_device: &str,
    stop: &AtomicBool,
) -> Result<Counters> {
    run_transport(
        Some(tun_fd),
        config,
        bind_device,
        Transport::Tcp,
        stop,
        None,
    )
}
/// Explicit TLS policy supplied by the caller, separate from Proton Stealth.
pub fn run_tls(
    tun_fd: i32,
    config: &WireguardConfig,
    bind_device: &str,
    connection: rustls::ClientConnection,
    stop: &AtomicBool,
) -> Result<Counters> {
    run_transport(
        Some(tun_fd),
        config,
        bind_device,
        Transport::Tls(Box::new(connection)),
        stop,
        None,
    )
}
/// Outer certificate identity is not checked; the configured inner WG peer key
/// authenticates all IP packets. Never forwards bare TLS plaintext to TUN.
pub fn run_stealth(
    tun_fd: i32,
    config: &WireguardConfig,
    bind_device: &str,
    sni: &crate::stealth::SniStrategy,
    stop: &AtomicBool,
) -> Result<Counters> {
    run_transport(
        Some(tun_fd),
        config,
        bind_device,
        Transport::Stealth(sni.clone()),
        stop,
        None,
    )
}
pub(crate) fn run_transport(
    tun_fd: Option<i32>,
    config: &WireguardConfig,
    bind_device: &str,
    transport: Transport,
    stop: &AtomicBool,
    observer: Option<&crate::connection::Observer>,
) -> Result<Counters> {
    run_owned(
        tun_fd.map(duplicate_nonblocking).transpose()?,
        config,
        bind_device,
        transport,
        stop,
        observer,
    )
}
pub(crate) fn run_owned(
    tun_file: Option<File>,
    config: &WireguardConfig,
    bind_device: &str,
    transport: Transport,
    stop: &AtomicBool,
    observer: Option<&crate::connection::Observer>,
) -> Result<Counters> {
    if bind_device.is_empty() && observer.is_none() {
        return Err(Error::Invalid("uplink device"));
    }
    match transport {
        Transport::Udp => {
            let addr = if config.endpoint.is_ipv4() {
                "0.0.0.0:0"
            } else {
                "[::]:0"
            };
            let socket = std::net::UdpSocket::bind(addr)?;
            socket.set_nonblocking(true)?;
            if !bind_device.is_empty() {
                linux_network::bind_device(
                    socket.as_raw_fd(),
                    bind_device,
                    config.endpoint.is_ipv6(),
                )?;
            }
            if let Some(protector) = observer.and_then(|o| o.protector())
                && !protector.protect_socket(socket.as_raw_fd())
            {
                return Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied).into());
            }
            socket.connect(config.endpoint)?;
            run(tun_file, config, Datagram(socket), stop, observer)
        }
        other => {
            if config.mtu > 16_351 {
                return Err(Error::Invalid("MTU exceeds normal TCP frame capacity"));
            }
            let socket = linux_network::connect_tcp(
                config.endpoint,
                bind_device,
                stop,
                observer.and_then(|o| o.protector()),
            )?;
            match other {
                Transport::Tcp => run(
                    tun_file,
                    config,
                    Stream(FramedIo::new(socket, 64)?),
                    stop,
                    observer,
                ),
                Transport::Tls(connection) => run(
                    tun_file,
                    config,
                    Stream(FramedIo::new(TlsIo::new(socket, *connection), 64)?),
                    stop,
                    observer,
                ),
                Transport::Stealth(sni) => {
                    let connection = crate::stealth::client(sni.for_peer(config.endpoint.ip())?)?;
                    run(
                        tun_file,
                        config,
                        Stream(FramedIo::new(TlsIo::new(socket, connection), 64)?),
                        stop,
                        observer,
                    )
                }
                Transport::Udp => unreachable!("handled above"),
            }
        }
    }
}
fn run<N: Network>(
    tun_file: Option<File>,
    config: &WireguardConfig,
    network: N,
    stop: &AtomicBool,
    observer: Option<&crate::connection::Observer>,
) -> Result<Counters> {
    crate::engine::run(tun_file, config, network, stop, observer)
}
