//! Linux UDP adapter. Only attaches to a caller-provided TUN fd; never creates
//! interfaces, installs packages, changes routes, changes DNS, or touches sysctls.
use crate::{
    Error, Result,
    config::WireguardConfig,
    packet::{PacketPool, PacketQueue},
    session::{Output, Session},
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
const BUDGET: usize = 64;
#[derive(Debug, Default, serde::Serialize)]
pub struct Counters {
    pub tun_reads: u64,
    pub udp_reads: u64,
    pub decrypted_packets: u64,
    pub invalid_packets: u64,
    pub pool_pressure: u64,
    pub polls: u64,
    pub tun_backpressure: u64,
    pub udp_backpressure: u64,
}
pub fn boottime() -> Result<Duration> {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: initialized output object; CLOCK_BOOTTIME advances through suspend.
    if unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut ts) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(Duration::new(
        ts.tv_sec
            .try_into()
            .map_err(|_| Error::Invalid("monotonic seconds"))?,
        ts.tv_nsec as u32,
    ))
}
fn duplicate_nonblocking(fd: i32) -> Result<File> {
    // SAFETY: fcntl duplicates the supplied descriptor without taking its ownership.
    let duplicate = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3) };
    if duplicate < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: duplicate is a new owned descriptor from successful fcntl.
    let file = unsafe { File::from_raw_fd(duplicate) };
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
pub fn run_udp(
    tun_fd: i32,
    config: &WireguardConfig,
    bind_device: &str,
    stop: &AtomicBool,
) -> Result<Counters> {
    if bind_device.is_empty()
        || bind_device.len() >= libc::IFNAMSIZ
        || bind_device.as_bytes().contains(&0)
    {
        return Err(Error::Invalid("uplink device"));
    }
    let mut tun = duplicate_nonblocking(tun_fd)?;
    let addr = if config.endpoint.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    };
    let socket = std::net::UdpSocket::bind(addr)?;
    socket.set_nonblocking(true)?;
    let device =
        std::ffi::CString::new(bind_device).map_err(|_| Error::Invalid("uplink device"))?;
    // SAFETY: valid socket fd, NUL-terminated bounded interface name, exact length.
    if unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_BINDTODEVICE,
            device.as_ptr().cast(),
            device.as_bytes_with_nul().len() as libc::socklen_t,
        )
    } < 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    socket.connect(config.endpoint)?;
    let mut socket = mio::net::UdpSocket::from_std(socket);
    let mut poll = Poll::new()?;
    poll.registry()
        .register(&mut SourceFd(&tun.as_raw_fd()), TUN, Interest::READABLE)?;
    poll.registry()
        .register(&mut socket, NET, Interest::READABLE)?;
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
    let mut tun_ready = true;
    let mut net_ready = true;
    let mut drain = false;
    while !stop.load(Ordering::Relaxed) {
        let mut timer_packet = pool.take().ok_or(Error::Full)?;
        let timed = session.advance(boottime()?, timer_packet.writable())?;
        match timed {
            Output::Network(n) => {
                timer_packet.set_len(n)?;
                enqueue(&mut pool, &mut to_net, timer_packet)?;
            }
            _ => pool.recycle(timer_packet)?,
        }
        // Bounded work per source, with immediate continuation if a budget expires.
        for _ in 0..BUDGET {
            if !tun_ready || pool.available() < 2 || to_net.len() >= 63 {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            match tun.read(&mut input.writable()[..config.mtu as usize]) {
                Ok(0) => {
                    pool.recycle(input)?;
                    return Err(Error::Invalid("TUN EOF"));
                }
                Ok(n) => {
                    counters.tun_reads += 1;
                    input.set_len(n)?;
                    let mut output = pool.take().ok_or(Error::Full)?;
                    let result = session.encrypt(input.data(), output.writable());
                    pool.recycle(input)?;
                    match result? {
                        Output::Network(n) => {
                            output.set_len(n)?;
                            enqueue(&mut pool, &mut to_net, output)?;
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
                    return Err(e.into());
                }
            }
        }
        for _ in 0..BUDGET {
            if !net_ready || pool.available() < 2 || to_tun.len() >= 63 || to_net.len() >= 63 {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            match socket.recv(input.writable()) {
                Ok(n) => {
                    counters.udp_reads += 1;
                    input.set_len(n)?;
                    let mut output = pool.take().ok_or(Error::Full)?;
                    let result = session.decrypt(input.data(), output.writable());
                    pool.recycle(input)?;
                    match result {
                        Ok(Output::Network(n)) => {
                            output.set_len(n)?;
                            enqueue(&mut pool, &mut to_net, output)?;
                            drain = true;
                        }
                        Ok(Output::Tunnel(n)) => {
                            output.set_len(n)?;
                            counters.decrypted_packets += 1;
                            enqueue(&mut pool, &mut to_tun, output)?;
                        }
                        Ok(Output::None) => pool.recycle(output)?,
                        Err(_) => {
                            counters.invalid_packets += 1;
                            pool.recycle(output)?;
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    net_ready = false;
                    pool.recycle(input)?;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {
                    pool.recycle(input)?;
                }
                Err(e) => {
                    pool.recycle(input)?;
                    return Err(e.into());
                }
            }
        }
        for _ in 0..BUDGET {
            if !drain || to_net.len() >= 63 {
                break;
            }
            let mut output = pool.take().ok_or(Error::Full)?;
            match session.drain(output.writable())? {
                Output::Network(n) => {
                    output.set_len(n)?;
                    enqueue(&mut pool, &mut to_net, output)?;
                }
                _ => {
                    pool.recycle(output)?;
                    drain = false;
                    break;
                }
            }
        }
        for _ in 0..BUDGET {
            let Some(packet) = to_net.front() else {
                break;
            };
            match socket.send(packet.data()) {
                Ok(n) if n == packet.data().len() => {
                    pool.recycle(to_net.pop().expect("front exists"))?
                }
                Ok(_) => return Err(Error::Invalid("partial UDP send")),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    counters.udp_backpressure += 1;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }
        for _ in 0..BUDGET {
            let Some(packet) = to_tun.front() else {
                break;
            };
            match tun.write(packet.data()) {
                Ok(n) if n == packet.data().len() => {
                    pool.recycle(to_tun.pop().expect("front exists"))?
                }
                Ok(_) => return Err(Error::Invalid("partial TUN write")),
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.raw_os_error() == Some(libc::ENOBUFS) =>
                {
                    counters.tun_backpressure += 1;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }
        let wants_net = !to_net.is_empty();
        let wants_tun = !to_tun.is_empty();
        if wants_net != net_writable {
            poll.registry().reregister(
                &mut socket,
                NET,
                if wants_net {
                    Interest::READABLE | Interest::WRITABLE
                } else {
                    Interest::READABLE
                },
            )?;
            net_writable = wants_net;
        }
        if wants_tun != tun_writable {
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
        let can_continue = (tun_ready && to_net.len() < 63)
            || (net_ready && to_tun.len() < 63 && to_net.len() < 63)
            || (drain && to_net.len() < 63);
        let timeout = if can_continue {
            Duration::ZERO
        } else {
            session
                .next_wakeup()
                .unwrap_or(Duration::from_millis(250))
                .min(Duration::from_millis(250))
        };
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
