//! Windows sockets attach to Mio's IOCP only once. TCP connect uses WSAPoll
//! first: attaching a temporary Poll would prevent reassociation to the worker.
use crate::{
    Error, Result,
    config::WireguardConfig,
    connection::Observer,
    network::{Datagram, Stream},
    platform::TunHandle,
    stream::FramedIo,
    tls::TlsIo,
    transport::{Counters, Transport},
};
use std::{
    io,
    net::SocketAddr,
    os::windows::io::AsRawSocket,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
use windows_sys::Win32::Networking::WinSock::*;

fn socket(endpoint: SocketAddr, tcp: bool, tun: Option<&TunHandle>) -> Result<socket2::Socket> {
    let socket = socket2::Socket::new(
        socket2::Domain::for_address(endpoint),
        if tcp {
            socket2::Type::STREAM
        } else {
            socket2::Type::DGRAM
        },
        Some(if tcp {
            socket2::Protocol::TCP
        } else {
            socket2::Protocol::UDP
        }),
    )?;
    socket.set_nonblocking(true)?;
    if let Some(tun) = tun {
        if tun.socket.send_buffer_size_bytes > 0 {
            socket.set_send_buffer_size(tun.socket.send_buffer_size_bytes as usize)?;
        }
        if tun.socket.receive_buffer_size_bytes > 0 {
            socket.set_recv_buffer_size(tun.socket.receive_buffer_size_bytes as usize)?;
        }
    }
    super::net::protect(&socket, endpoint, tun.map_or(0, |tun| tun.session.index))?;
    Ok(socket)
}
fn tcp(
    endpoint: SocketAddr,
    tun: Option<&TunHandle>,
    stop: &AtomicBool,
) -> Result<mio::net::TcpStream> {
    let socket = socket(endpoint, true, tun)?;
    match socket.connect(&endpoint.into()) {
        Ok(()) => {}
        Err(error) if matches!(error.raw_os_error(), Some(WSAEWOULDBLOCK | WSAEINPROGRESS)) => {
            let handle = usize::try_from(socket.as_raw_socket())
                .map_err(|_| Error::Invalid("native SOCKET width"))?;
            let deadline = Instant::now() + Duration::from_secs(10);
            loop {
                if stop.load(Ordering::Relaxed) {
                    return Err(io::Error::from(io::ErrorKind::Interrupted).into());
                }
                if Instant::now() >= deadline {
                    return Err(io::Error::from(io::ErrorKind::TimedOut).into());
                }
                let mut descriptor = WSAPOLLFD {
                    fd: handle,
                    events: POLLWRNORM,
                    revents: 0,
                };
                // SAFETY: live native-width socket, one mutable poll descriptor,
                // bounded wait. No IOCP is associated with the socket here.
                let ready = unsafe { WSAPoll(&mut descriptor, 1, 100) };
                if ready == SOCKET_ERROR {
                    // SAFETY: error belongs to the immediately preceding call.
                    return Err(io::Error::from_raw_os_error(unsafe { WSAGetLastError() }).into());
                }
                if ready == 0 {
                    continue;
                }
                if let Some(error) = socket.take_error()? {
                    return Err(error.into());
                }
                if socket.peer_addr().is_ok() {
                    break;
                }
            }
        }
        Err(error) => return Err(error.into()),
    }
    socket.set_tcp_nodelay(true)?;
    let socket: std::net::TcpStream = socket.into();
    Ok(mio::net::TcpStream::from_std(socket))
}
pub(crate) fn run_owned(
    tun: Option<TunHandle>,
    config: &WireguardConfig,
    _: &str,
    transport: Transport,
    stop: &AtomicBool,
    observer: Option<&Observer>,
) -> Result<Counters> {
    if observer.and_then(Observer::protector).is_some() {
        return Err(Error::Unsupported("Unix fd protector on Windows"));
    }
    match transport {
        Transport::Udp => {
            let socket = socket(config.endpoint, false, tun.as_ref())?;
            socket.bind(
                &if config.endpoint.is_ipv4() {
                    "0.0.0.0:0"
                } else {
                    "[::]:0"
                }
                .parse::<SocketAddr>()
                .expect("constant")
                .into(),
            )?;
            socket.connect(&config.endpoint.into())?;
            let socket: std::net::UdpSocket = socket.into();
            crate::engine::run(
                tun,
                config,
                Datagram(mio::net::UdpSocket::from_std(socket)),
                stop,
                observer,
            )
        }
        other => {
            if config.mtu > 16351 {
                return Err(Error::Invalid("MTU exceeds normal TCP frame capacity"));
            }
            let socket = tcp(config.endpoint, tun.as_ref(), stop)?;
            match other {
                Transport::Tcp => crate::engine::run(
                    tun,
                    config,
                    Stream(FramedIo::new(socket, 64)?),
                    stop,
                    observer,
                ),
                Transport::Tls(connection) => crate::engine::run(
                    tun,
                    config,
                    Stream(FramedIo::new(TlsIo::new(socket, *connection), 64)?),
                    stop,
                    observer,
                ),
                Transport::Stealth(sni) => {
                    let connection = crate::stealth::client(sni.for_peer(config.endpoint.ip())?)?;
                    crate::engine::run(
                        tun,
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
