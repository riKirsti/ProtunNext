//! Linux UDP adapter. Only attaches to a caller-provided TUN fd; never creates
//! interfaces, installs packages, changes routes, changes DNS, or touches sysctls.
use crate::{
    Error, Result,
    config::WireguardConfig,
    linux_network::{self, Datagram, Network, Stream},
    packet::{PacketPool, PacketQueue},
    session::{Output, Session},
    stream::{FramedIo, Receive},
    tls::TlsIo,
    tun_device::TunDevice,
};
use mio::{Events, Interest, Poll, Token, unix::SourceFd};
use std::{
    fs::File,
    io::{Read, Write},
    os::fd::{AsRawFd, FromRawFd},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const TUN: Token = Token(1);
const NET: Token = Token(2);
const CONTROL: Token = Token(3);
const BUDGET: usize = 64;
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Counters {
    pub tun_reads: u64,
    pub network_reads: u64,
    pub decrypted_packets: u64,
    pub invalid_packets: u64,
    pub outbound_filtered: u64,
    pub inbound_filtered: u64,
    pub pool_pressure: u64,
    pub polls: u64,
    pub tun_backpressure: u64,
    pub network_backpressure: u64,
}
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
fn enqueue(
    pool: &mut PacketPool,
    queue: &mut PacketQueue,
    packet: crate::packet::Packet,
) -> Result<()> {
    if let Err(packet) = queue.push(packet) {
        pool.recycle(packet)?;
        return Err(Error::Full);
    }
    Ok(())
}
fn enqueue_network(
    pool: &mut PacketPool,
    queue: &mut PacketQueue,
    packet: crate::packet::Packet,
    observer: Option<&crate::connection::Observer>,
) -> Result<()> {
    if let Some(observer) = observer {
        observer.capture_wireguard(packet.data(), true);
    }
    enqueue(pool, queue, packet)
}
pub enum Transport {
    Udp,
    Tcp,
    Tls(Box<rustls::ClientConnection>),
    Stealth(crate::stealth::SniStrategy),
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
            run(tun_fd, config, Datagram(socket), stop, observer)
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
                    tun_fd,
                    config,
                    Stream(FramedIo::new(socket, 64)?),
                    stop,
                    observer,
                ),
                Transport::Tls(connection) => run(
                    tun_fd,
                    config,
                    Stream(FramedIo::new(TlsIo::new(socket, *connection), 64)?),
                    stop,
                    observer,
                ),
                Transport::Stealth(sni) => {
                    let connection = crate::stealth::client(sni.for_peer(config.endpoint.ip())?)?;
                    run(
                        tun_fd,
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
    tun_fd: Option<i32>,
    config: &WireguardConfig,
    mut network: N,
    stop: &AtomicBool,
    observer: Option<&crate::connection::Observer>,
) -> Result<Counters> {
    let mut tun = tun_fd
        .map(duplicate_nonblocking)
        .transpose()?
        .map(TunDevice);
    let allowed_ips = crate::routing::AllowedIps::new(&config.allowed_ips)?;
    let compatibility_policy = observer.is_some_and(|o| o.compatibility_policy());
    let network_fd = network.fd();
    let handshake_deadline = boottime()? + Duration::from_secs(10);
    let mut poll = Poll::new()?;
    if let Some(observer) = observer {
        observer.set_waker(std::sync::Arc::new(mio::Waker::new(
            poll.registry(),
            CONTROL,
        )?));
    }
    if let Some(tun) = &tun {
        poll.registry()
            .register(&mut SourceFd(&tun.as_raw_fd()), TUN, Interest::READABLE)?;
    }
    poll.registry()
        .register(&mut SourceFd(&network_fd), NET, Interest::READABLE)?;
    let mut events = Events::with_capacity(16);
    let mut tun_writable = false;
    let mut net_writable = false;
    let mut session = Session::new(
        config.private_key.expose(),
        config.peer_key,
        config.keepalive,
        boottime()?,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Invalid("wall clock"))?,
    )?;
    let mut pool = PacketPool::new(132, 65535)?;
    let mut to_net = PacketQueue::new(64)?;
    let mut to_tun = PacketQueue::new(64)?;
    let mut counters = Counters::default();
    let mut initiation = pool.take().ok_or(Error::Full)?;
    match session.initiate(initiation.writable())? {
        Output::Network(n) => {
            initiation.set_len(n)?;
            enqueue_network(&mut pool, &mut to_net, initiation, observer)?;
        }
        _ => pool.recycle(initiation)?,
    }
    let mut tun_ready = tun.is_some();
    let mut net_ready = true;
    let mut drain = false;
    let mut network_error = None;
    while !stop.load(Ordering::Relaxed) {
        if let Some(replacement) = observer.and_then(|o| o.take_tun_update()) {
            if let Some(old) = &tun {
                poll.registry()
                    .deregister(&mut SourceFd(&old.as_raw_fd()))?;
            }
            tun = replacement.map(TunDevice);
            tun_ready = tun.is_some();
            tun_writable = false;
            if let Some(new) = &tun {
                poll.registry().register(
                    &mut SourceFd(&new.as_raw_fd()),
                    TUN,
                    Interest::READABLE,
                )?;
            }
            if let Some(observer) = observer {
                observer.interface(tun.is_some(), None);
            }
            if tun.is_none() {
                while let Some(packet) = to_tun.pop() {
                    pool.recycle(packet)?;
                }
            }
        }
        if network.handshaking() && boottime()? >= handshake_deadline {
            return Err(std::io::Error::from(std::io::ErrorKind::TimedOut).into());
        }
        if network_error.is_none() {
            let mut timer_packet = pool.take().ok_or(Error::Full)?;
            let timed = session.advance(boottime()?, timer_packet.writable())?;
            match timed {
                Output::Network(n) => {
                    timer_packet.set_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, timer_packet, observer)?;
                }
                _ => pool.recycle(timer_packet)?,
            }
        }
        // Bounded work per source, with immediate continuation if a budget expires.
        for _ in 0..BUDGET {
            if network_error.is_some()
                || !tun_ready
                || pool.available() < 2
                || to_net.len() + network.pending() >= 63
            {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            let Some(device) = tun.as_mut() else {
                pool.recycle(input)?;
                break;
            };
            match device.read(&mut input.writable()[..config.mtu as usize]) {
                Ok(0) => {
                    pool.recycle(input)?;
                    detach_tun(&mut tun, &poll, observer, Error::Invalid("TUN EOF"))?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
                Ok(n) => {
                    counters.tun_reads += 1;
                    input.set_len(n)?;
                    if compatibility_policy
                        && crate::virtual_policy::outbound(input.data())
                            == crate::virtual_policy::Route::Drop
                    {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    if !allowed_ips.outbound(input.data()) {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    #[cfg(feature = "local-agent")]
                    if let Some(observer) = observer
                        && observer.internal_local(input.data(), boottime()?)?
                    {
                        pool.recycle(input)?;
                        continue;
                    }
                    if let Some(observer) = observer {
                        observer.capture(input.data());
                    }
                    #[cfg(feature = "local-agent")]
                    if let Some(observer) = observer
                        && observer.customer_outbound(input.data_mut()).is_err()
                    {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    let mut output = pool.take().ok_or(Error::Full)?;
                    let result = session.encrypt(input.data(), output.writable());
                    pool.recycle(input)?;
                    match result? {
                        Output::Network(n) => {
                            output.set_len(n)?;
                            enqueue_network(&mut pool, &mut to_net, output, observer)?;
                            drain = true;
                        }
                        _ => pool.recycle(output)?,
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    tun_ready = false;
                    pool.recycle(input)?;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {
                    pool.recycle(input)?;
                }
                Err(e) => {
                    pool.recycle(input)?;
                    detach_tun(&mut tun, &poll, observer, e.into())?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
            }
        }
        #[cfg(feature = "local-agent")]
        for _ in 0..BUDGET {
            let Some(observer) = observer else {
                break;
            };
            if network_error.is_some()
                || to_net.len() + network.pending() >= 63
                || pool.available() < 2
            {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            let Some(n) = observer.internal_read(input.writable(), boottime()?)? else {
                pool.recycle(input)?;
                break;
            };
            input.set_len(n)?;
            if crate::api::internal::local_service_source(input.data()) {
                if tun.is_some() && to_tun.len() < 63 {
                    enqueue(&mut pool, &mut to_tun, input)?;
                } else {
                    pool.recycle(input)?;
                }
                continue;
            }
            observer.capture(input.data());
            let mut output = pool.take().ok_or(Error::Full)?;
            let result = session.encrypt(input.data(), output.writable());
            pool.recycle(input)?;
            match result? {
                Output::Network(n) => {
                    output.set_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, output, Some(observer))?;
                    drain = true;
                }
                _ => pool.recycle(output)?,
            }
        }
        for _ in 0..BUDGET {
            if network_error.is_some()
                || !net_ready
                || pool.available() < 2
                || to_tun.len() >= 63
                || to_net.len() + network.pending() >= 63
            {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            match network.receive(&mut input) {
                Ok(Receive::Packet) => {
                    counters.network_reads += 1;
                    if let Some(observer) = observer {
                        observer.capture_wireguard(input.data(), false);
                    }
                    let mut output = pool.take().ok_or(Error::Full)?;
                    let result = session.decrypt(input.data(), output.writable());
                    pool.recycle(input)?;
                    match result {
                        Ok(Output::Network(n)) => {
                            output.set_len(n)?;
                            enqueue_network(&mut pool, &mut to_net, output, observer)?;
                            drain = true;
                        }
                        Ok(Output::Tunnel(n)) => {
                            output.set_len(n)?;
                            counters.decrypted_packets += 1;
                            if let Some(observer) = observer {
                                observer.capture(output.data());
                            }
                            if compatibility_policy
                                && crate::virtual_policy::inbound(output.data())
                                    == crate::virtual_policy::Route::Drop
                            {
                                counters.inbound_filtered += 1;
                                pool.recycle(output)?;
                                continue;
                            }
                            #[cfg(feature = "local-agent")]
                            if let Some(observer) = observer
                                && observer.internal_receive(output.data_mut(), boottime()?)?
                            {
                                pool.recycle(output)?;
                                continue;
                            }
                            if allowed_ips.inbound(output.data()) && tun.is_some() {
                                enqueue(&mut pool, &mut to_tun, output)?;
                            } else {
                                counters.inbound_filtered += 1;
                                pool.recycle(output)?;
                            }
                        }
                        Ok(Output::None) => pool.recycle(output)?,
                        Err(_) => {
                            counters.invalid_packets += 1;
                            pool.recycle(output)?;
                        }
                    }
                }
                Ok(Receive::WouldBlock) => {
                    net_ready = false;
                    pool.recycle(input)?;
                    break;
                }
                Ok(Receive::Yield) => {
                    pool.recycle(input)?;
                }
                Ok(Receive::Eof) => {
                    pool.recycle(input)?;
                    network_error =
                        Some(std::io::Error::from(std::io::ErrorKind::UnexpectedEof).into());
                    tun_ready = false;
                    net_ready = false;
                    drain = false;
                    break;
                }
                Err(e) => {
                    pool.recycle(input)?;
                    network_error = Some(e);
                    tun_ready = false;
                    net_ready = false;
                    drain = false;
                    break;
                }
            }
        }
        for _ in 0..BUDGET {
            if !drain || to_net.len() + network.pending() >= 63 {
                break;
            }
            let mut output = pool.take().ok_or(Error::Full)?;
            match session.drain(output.writable())? {
                Output::Network(n) => {
                    output.set_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, output, observer)?;
                }
                _ => {
                    pool.recycle(output)?;
                    drain = false;
                    break;
                }
            }
        }
        if network_error.is_none() {
            match network.flush(&mut to_net, &mut pool) {
                Ok(true) => counters.network_backpressure += 1,
                Ok(false) => {}
                Err(e) => {
                    network_error = Some(e);
                    tun_ready = false;
                    net_ready = false;
                    drain = false;
                }
            }
        }
        for _ in 0..BUDGET {
            let Some(packet) = to_tun.front() else {
                break;
            };
            let Some(device) = tun.as_mut() else {
                while let Some(packet) = to_tun.pop() {
                    pool.recycle(packet)?;
                }
                break;
            };
            match device.write(packet.data()) {
                Ok(n) if n == packet.data().len() => {
                    pool.recycle(to_tun.pop().expect("front exists"))?
                }
                Ok(_) => {
                    detach_tun(
                        &mut tun,
                        &poll,
                        observer,
                        Error::Invalid("partial TUN write"),
                    )?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.raw_os_error() == Some(libc::ENOBUFS) =>
                {
                    counters.tun_backpressure += 1;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    detach_tun(&mut tun, &poll, observer, e.into())?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
            }
        }
        if let Some(observer) = observer {
            observer.observe(&session, &counters, boottime()?);
        }
        // Preserve already authenticated inbound packets before reporting a
        // terminal stream error. Otherwise a reply followed by FIN in the same
        // readiness batch would be discarded before its TUN write.
        if to_tun.is_empty()
            && let Some(error) = network_error.take()
        {
            return Err(error);
        }
        let wants_net = network_error.is_none()
            && (!to_net.is_empty() || network.pending() > 0 || network.wants_write());
        let wants_tun = !to_tun.is_empty();
        if wants_net != net_writable {
            poll.registry().reregister(
                &mut SourceFd(&network_fd),
                NET,
                if wants_net {
                    Interest::READABLE | Interest::WRITABLE
                } else {
                    Interest::READABLE
                },
            )?;
            net_writable = wants_net;
        }
        if let Some(tun) = &tun
            && wants_tun != tun_writable
        {
            poll.registry().reregister(
                &mut SourceFd(&tun.as_raw_fd()),
                TUN,
                if wants_tun {
                    Interest::READABLE | Interest::WRITABLE
                } else {
                    Interest::READABLE
                },
            )?;
            tun_writable = wants_tun;
        }
        let can_continue = network_error.is_none()
            && ((tun_ready && to_net.len() + network.pending() < 63)
                || (net_ready && to_tun.len() < 63 && to_net.len() + network.pending() < 63)
                || (drain && to_net.len() + network.pending() < 63));
        let timeout = if can_continue {
            Duration::ZERO
        } else {
            session
                .next_wakeup()
                .unwrap_or(Duration::from_millis(250))
                .min(Duration::from_millis(250))
        };
        #[cfg(feature = "local-agent")]
        let timeout = observer
            .and_then(|o| boottime().ok().and_then(|now| o.internal_wakeup(now)))
            .map_or(timeout, |internal| internal.min(timeout));
        match poll.poll(&mut events, Some(timeout)) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        }
        counters.polls += 1;
        for event in &events {
            if event.is_readable() {
                if event.token() == TUN {
                    tun_ready = true;
                } else if event.token() == NET {
                    net_ready = true;
                }
            }
        }
    }
    Ok(counters)
}
fn detach_tun(
    tun: &mut Option<TunDevice>,
    poll: &Poll,
    observer: Option<&crate::connection::Observer>,
    error: Error,
) -> Result<()> {
    let Some(observer) = observer else {
        return Err(error);
    };
    if let Some(old) = tun.take() {
        let _ = poll.registry().deregister(&mut SourceFd(&old.as_raw_fd()));
    }
    observer.interface(false, Some(error.to_string()));
    Ok(())
}
