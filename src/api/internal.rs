//! Private control sockets use a separate TCP/IP stack. Ordinary TUN data does
//! not traverse this stack. Only explicitly registered control flows are demuxed.
use crate::{Error, Result};
use futures::{AsyncRead, AsyncWrite};
use proton_boringtun::proton_os_interface::{lock::strict::Strict, rand::SeedableRng};
use proton_vpn_netstack::{
    NetworkConfig, NetworkConfigV4, NetworkConfigV6, NetworkInterface, NetworkStack,
    TransportFactory,
    error::DeviceReadError,
    socket::{AsyncTcpConnect, ConnectedUdpSocket, TcpStream},
};
use rand_chacha::ChaCha20Rng;
use std::{
    collections::HashMap,
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake},
    time::{Duration, Instant},
};

const PRIVILEGED: [u16; 16] = [4, 6, 8, 10, 12, 14, 15, 16, 26, 28, 30, 32, 34, 36, 40, 60];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Origin {
    Control,
    Customer,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Flow {
    origin: Origin,
    protocol: u8,
    local: SocketAddr,
    remote: SocketAddr,
}
struct Mapping {
    flow: Flow,
    wire: u16,
    expires: Option<Instant>,
}
#[derive(Default)]
struct Flows {
    entries: HashMap<Flow, Mapping>,
}
#[derive(Clone)]
pub struct InternalConnector {
    factory: TransportFactory,
    flows: Arc<Mutex<Flows>>,
}
impl std::fmt::Debug for InternalConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InternalConnector")
    }
}
pub struct InternalNetwork {
    interface: NetworkInterface,
    flows: Arc<Mutex<Flows>>,
}
impl std::fmt::Debug for InternalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InternalNetwork")
    }
}
impl InternalNetwork {
    pub fn might_send(packet: &[u8]) -> bool {
        transport(packet).is_some_and(|p| reserved(p.source_port))
    }
    pub fn might_receive(packet: &[u8]) -> bool {
        transport(packet).is_some_and(|p| reserved(p.destination_port))
    }
    pub fn next_wakeup(&self, now: Duration) -> Option<Duration> {
        use proton_vpn_netstack::Deadline;
        self.interface
            .next_deadline(now.as_nanos().min(u64::MAX as u128) as u64)
    }
    pub fn new(
        now: Duration,
        wake: impl Wake + Send + Sync + 'static,
    ) -> Result<(InternalConnector, Self)> {
        let mut seed = [0; 32];
        getrandom::fill(&mut seed).map_err(|e| Error::Protocol(e.to_string()))?;
        let v4 = NetworkConfigV4::new(
            Ipv4Addr::new(10, 2, 0, 2),
            24,
            Some(Ipv4Addr::new(10, 2, 0, 1)),
        )
        .with_second_addr(Ipv4Addr::new(10, 2, 0, 3), 24);
        let client6: Ipv6Addr = "2a07:b944::2:2".parse().expect("constant");
        let v6 = NetworkConfigV6::new(
            client6,
            112,
            Some("2a07:b944::2:1".parse().expect("constant")),
        )
        .with_second_addr("2a07:b944::2:3".parse().expect("constant"), 112);
        let mut stack = NetworkStack::builder()
            .with_time(now.as_millis().min(i64::MAX as u128) as i64)
            .with_config(NetworkConfig::Both(v4, v6))
            .with_rng(ChaCha20Rng::from_seed(seed))
            .with_mtu(1420)
            .with_tcp_rx_buffer_size(65536)
            .with_tcp_tx_buffer_size(65536)
            .build::<Strict>();
        stack.register_waker(wake);
        let (factory, interface) = stack.into_sides();
        let flows = Arc::new(Mutex::new(Flows::default()));
        Ok((
            InternalConnector {
                factory,
                flows: flows.clone(),
            },
            Self { interface, flows },
        ))
    }
    pub fn read_packet(&mut self, output: &mut [u8], now: Duration) -> Result<Option<usize>> {
        let n = match self
            .interface
            .read_packet(output, now.as_micros().min(i64::MAX as u128) as i64)
        {
            Ok(n) => n,
            Err(DeviceReadError::WouldBlock) => return Ok(None),
            Err(_) => return Err(Error::Invalid("internal stack packet capacity")),
        };
        if local_service_source(&output[..n]) {
            return Ok(Some(n));
        }
        let Some(parsed) = transport(&output[..n]) else {
            return Err(Error::Invalid(
                "control stack generated unsupported IP packet",
            ));
        };
        let flow = Flow {
            origin: Origin::Control,
            protocol: parsed.protocol,
            local: SocketAddr::new(parsed.source, parsed.source_port),
            remote: SocketAddr::new(parsed.destination, parsed.destination_port),
        };
        let mappings = self.flows.lock().unwrap_or_else(|e| e.into_inner());
        let Some(mapping) = mappings.entries.get(&flow) else {
            return Ok(None);
        };
        rewrite_port(
            &mut output[..n],
            parsed.offset,
            parsed.protocol,
            true,
            mapping.wire,
        );
        Ok(Some(n))
    }
    /// True means the packet belongs to a registered private socket, never TUN.
    pub fn receive(&mut self, packet: &mut [u8], now: Duration) -> Result<bool> {
        let Some(parsed) = transport(packet) else {
            return Ok(false);
        };
        if !reserved(parsed.destination_port) {
            return Ok(false);
        }
        let mapping = {
            let mut mappings = self.flows.lock().unwrap_or_else(|e| e.into_inner());
            mappings
                .entries
                .values_mut()
                .find(|m| {
                    m.wire == parsed.destination_port
                        && m.flow.protocol == parsed.protocol
                        && m.flow.remote == SocketAddr::new(parsed.source, parsed.source_port)
                        && m.flow.local.ip() == parsed.destination
                        && m.expires.is_none_or(|e| e > Instant::now())
                })
                .map(|m| {
                    if m.flow.origin == Origin::Customer {
                        m.expires = Some(Instant::now() + Duration::from_secs(60));
                    }
                    (m.flow.local.port(), m.flow.origin)
                })
        };
        let Some((original, origin)) = mapping else {
            // Reserve an unsolicited inbound customer flow before a later
            // control socket can choose the same reply tuple.
            self.reserve_customer(&parsed, false)?;
            return Ok(false);
        };
        rewrite_port(packet, parsed.offset, parsed.protocol, false, original);
        if origin == Origin::Customer {
            return Ok(false);
        }
        self.interface
            .write_packet(packet, now.as_micros().min(i64::MAX as u128) as i64)
            .map_err(|_| Error::Full)?;
        Ok(true)
    }
    /// Only the small upstream addressable port range needs connection tracking.
    /// Ordinary application ports bypass this map and its lock entirely.
    pub fn customer_outbound(&mut self, packet: &mut [u8]) -> Result<()> {
        let Some(parsed) = transport(packet) else {
            return Ok(());
        };
        if !reserved(parsed.source_port) {
            return Ok(());
        }
        let port = self.reserve_customer(&parsed, true)?;
        rewrite_port(packet, parsed.offset, parsed.protocol, true, port);
        Ok(())
    }
    fn reserve_customer(&mut self, parsed: &Parsed, outbound: bool) -> Result<u16> {
        let flow = Flow {
            origin: Origin::Customer,
            protocol: parsed.protocol,
            local: if outbound {
                SocketAddr::new(parsed.source, parsed.source_port)
            } else {
                SocketAddr::new(parsed.destination, parsed.destination_port)
            },
            remote: if outbound {
                SocketAddr::new(parsed.destination, parsed.destination_port)
            } else {
                SocketAddr::new(parsed.source, parsed.source_port)
            },
        };
        let mut flows = self.flows.lock().unwrap_or_else(|e| e.into_inner());
        let expires = Some(Instant::now() + Duration::from_secs(60));
        if let Some(mapping) = flows.entries.get_mut(&flow) {
            mapping.expires = expires;
            return Ok(mapping.wire);
        }
        flows
            .entries
            .retain(|_, m| m.expires.is_none_or(|e| e > Instant::now()));
        if flows.entries.len() >= 4096 {
            return Err(Error::Full);
        }
        // Preserve the customer source port when its reply tuple is free.
        // Only control-originated flows preferentially use privileged ports.
        let wire = if !outbound
            || !flows.entries.values().any(|m| {
                m.wire == flow.local.port()
                    && m.flow.protocol == flow.protocol
                    && m.flow.remote == flow.remote
                    && m.flow.local.ip() == flow.local.ip()
            }) {
            flow.local.port()
        } else {
            (1000..=2000)
                .find(|port| {
                    !flows.entries.values().any(|m| {
                        m.wire == *port
                            && m.flow.protocol == flow.protocol
                            && m.flow.remote == flow.remote
                            && m.flow.local.ip() == flow.local.ip()
                    })
                })
                .ok_or(Error::Full)?
        };
        flows.entries.insert(
            flow,
            Mapping {
                flow,
                wire,
                expires,
            },
        );
        Ok(wire)
    }
    pub fn receive_local(&mut self, packet: &[u8], now: Duration) -> Result<bool> {
        if !local_service_destination(packet) {
            return Ok(false);
        }
        self.interface
            .write_packet(packet, now.as_micros().min(i64::MAX as u128) as i64)
            .map_err(|_| Error::Full)?;
        Ok(true)
    }
}
fn reserved(port: u16) -> bool {
    (1000..=2000).contains(&port) || PRIVILEGED.contains(&port)
}
struct Lease {
    flow: Flow,
    flows: Arc<Mutex<Flows>>,
}
impl Drop for Lease {
    fn drop(&mut self) {
        if let Some(m) = self
            .flows
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .entries
            .get_mut(&self.flow)
        {
            m.expires = Some(Instant::now() + Duration::from_secs(60));
        }
    }
}
impl InternalConnector {
    fn lease(&self, protocol: u8, local: SocketAddr, remote: SocketAddr) -> io::Result<Lease> {
        let flow = Flow {
            origin: Origin::Control,
            protocol,
            local,
            remote,
        };
        let mut flows = self.flows.lock().unwrap_or_else(|e| e.into_inner());
        flows
            .entries
            .retain(|_, m| m.expires.is_none_or(|e| e > Instant::now()));
        let wire = PRIVILEGED
            .into_iter()
            .chain(1000..=2000)
            .find(|port| !flows.entries.values().any(|m| m.wire == *port))
            .ok_or(io::ErrorKind::OutOfMemory)?;
        flows.entries.insert(
            flow,
            Mapping {
                flow,
                wire,
                expires: None,
            },
        );
        Ok(Lease {
            flow,
            flows: self.flows.clone(),
        })
    }
    pub async fn connect(&self, address: SocketAddr) -> io::Result<TrackedStream> {
        let stream = self
            .factory
            .tcp_connect(address)
            .map_err(io::Error::other)?;
        let lease = self.lease(6, stream.local_addr(), address)?;
        let stream = AsyncTcpConnect::Pending(stream)
            .await
            .map_err(io::Error::other)?;
        Ok(TrackedStream {
            stream,
            _lease: lease,
        })
    }
    pub fn udp(&self, address: SocketAddr) -> io::Result<TrackedDatagram> {
        let socket = self
            .factory
            .udp_bind(SocketAddr::new(
                if address.is_ipv4() {
                    IpAddr::V4(Ipv4Addr::new(10, 2, 0, 2))
                } else {
                    IpAddr::V6(Ipv6Addr::new(0x2a07, 0xb944, 0, 0, 0, 0, 2, 2))
                },
                0,
            ))
            .map_err(io::Error::other)?;
        let lease = self.lease(17, socket.local_addr(), address)?;
        Ok(TrackedDatagram {
            socket: socket.connect(address),
            _lease: lease,
        })
    }
    pub fn factory(&self) -> &TransportFactory {
        &self.factory
    }
}
pub struct TrackedStream {
    stream: TcpStream,
    _lease: Lease,
}
impl AsyncRead for TrackedStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}
impl AsyncWrite for TrackedStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.stream).poll_write(cx, buf)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_close(cx)
    }
}
pub struct TrackedDatagram {
    socket: ConnectedUdpSocket,
    _lease: Lease,
}
impl TrackedDatagram {
    pub async fn send(&self, bytes: &[u8]) -> io::Result<()> {
        self.socket.send(bytes).await.map_err(Into::into)
    }
    pub async fn recv(&self, bytes: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(bytes).await.map_err(Into::into)
    }
}
struct Parsed {
    source: IpAddr,
    destination: IpAddr,
    protocol: u8,
    source_port: u16,
    destination_port: u16,
    offset: usize,
}
fn transport(p: &[u8]) -> Option<Parsed> {
    let (source, destination, protocol, offset) = match p.first()? >> 4 {
        4 if p.len() >= 20 => {
            let offset = (p[0] & 15) as usize * 4;
            if offset < 20 || p.len() < offset || u16::from_be_bytes([p[6], p[7]]) & 0x1fff != 0 {
                return None;
            }
            (
                IpAddr::V4(Ipv4Addr::from(<[u8; 4]>::try_from(&p[12..16]).ok()?)),
                IpAddr::V4(Ipv4Addr::from(<[u8; 4]>::try_from(&p[16..20]).ok()?)),
                p[9],
                offset,
            )
        }
        6 if p.len() >= 40 => (
            IpAddr::V6(Ipv6Addr::from(<[u8; 16]>::try_from(&p[8..24]).ok()?)),
            IpAddr::V6(Ipv6Addr::from(<[u8; 16]>::try_from(&p[24..40]).ok()?)),
            p[6],
            40,
        ),
        _ => return None,
    };
    let minimum = match protocol {
        6 => 20,
        17 => 8,
        _ => return None,
    };
    if p.len() < offset + minimum {
        return None;
    }
    Some(Parsed {
        source,
        destination,
        protocol,
        offset,
        source_port: u16::from_be_bytes([p[offset], p[offset + 1]]),
        destination_port: u16::from_be_bytes([p[offset + 2], p[offset + 3]]),
    })
}
fn rewrite_port(p: &mut [u8], offset: usize, protocol: u8, source: bool, port: u16) {
    let at = offset + if source { 0 } else { 2 };
    let old = u16::from_be_bytes([p[at], p[at + 1]]);
    let check = offset + if protocol == 6 { 16 } else { 6 };
    let original = u16::from_be_bytes([p[check], p[check + 1]]);
    p[at..at + 2].copy_from_slice(&port.to_be_bytes());
    if protocol == 17 && original == 0 && p[0] >> 4 == 4 {
        return;
    }
    // RFC 1624 incremental checksum update; IP addresses/payload are unchanged.
    let mut sum = (!original as u32 & 0xffff) + (!old as u32 & 0xffff) + port as u32;
    while sum > 0xffff {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    let adjusted = !(sum as u16);
    let adjusted = if protocol == 17 && adjusted == 0 {
        0xffff
    } else {
        adjusted
    };
    p[check..check + 2].copy_from_slice(&adjusted.to_be_bytes());
}
pub fn local_service_source(p: &[u8]) -> bool {
    address_is_local(p, true)
}
pub fn local_service_destination(p: &[u8]) -> bool {
    address_is_local(p, false)
}
fn address_is_local(p: &[u8], source: bool) -> bool {
    match p.first().map(|b| b >> 4) {
        Some(4) if p.len() >= 20 => {
            let at = if source { 12 } else { 16 };
            p[at..at + 4] == [10, 2, 0, 3]
        }
        Some(6) if p.len() >= 40 => {
            let at = if source { 8 } else { 24 };
            p[at..at + 16] == Ipv6Addr::new(0x2a07, 0xb944, 0, 0, 0, 0, 2, 3).octets()
        }
        _ => false,
    }
}
