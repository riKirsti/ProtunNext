//! Windows sockets attach to Mio once. Pending TCP connections are serviced by
//! the same reactor as TUN traffic, with no temporary readiness registration.
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
use std::{net::SocketAddr, sync::atomic::AtomicBool};
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
fn tcp(endpoint: SocketAddr, tun: Option<&TunHandle>) -> Result<crate::network::ConnectingTcp> {
    let socket = socket(endpoint, true, tun)?;
    match socket.connect(&endpoint.into()) {
        Ok(()) => {}
        Err(error) if matches!(error.raw_os_error(), Some(WSAEWOULDBLOCK | WSAEINPROGRESS)) => {}
        Err(error) => return Err(error.into()),
    }
    socket.set_tcp_nodelay(true)?;
    let socket: std::net::TcpStream = socket.into();
    Ok(crate::network::ConnectingTcp::new(
        mio::net::TcpStream::from_std(socket),
    ))
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
                Datagram::new(mio::net::UdpSocket::from_std(socket)),
                stop,
                observer,
            )
        }
        other => {
            if config.mtu > 16351 {
                return Err(Error::Invalid("MTU exceeds normal TCP frame capacity"));
            }
            let socket = tcp(config.endpoint, tun.as_ref())?;
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
                    Stream(FramedIo::borrowing(TlsIo::new(socket, *connection), 64)?),
                    stop,
                    observer,
                ),
                Transport::Stealth(sni) => {
                    let connection = crate::stealth::client(sni.for_peer(config.endpoint.ip())?)?;
                    crate::engine::run(
                        tun,
                        config,
                        Stream(FramedIo::borrowing(TlsIo::new(socket, connection), 64)?),
                        stop,
                        observer,
                    )
                }
                Transport::Udp => unreachable!("handled above"),
            }
        }
    }
}
