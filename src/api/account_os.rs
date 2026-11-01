//! Muon/RCRL host capabilities. All DNS and TCP use the private in-tunnel stack.
use super::internal::{InternalConnector, TrackedStream};
use futures::{AsyncReadExt, AsyncWriteExt};
use muon::rt::{self, SinceUnixEpoch};
use std::{
    future::Future,
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
#[derive(Debug, Clone)]
#[cfg_attr(any(unix, windows), derive(Default))]
pub struct Clock {
    #[cfg(not(any(unix, windows)))]
    start: std::time::Instant,
}
#[cfg(not(any(unix, windows)))]
impl Default for Clock {
    fn default() -> Self {
        Self {
            #[cfg(not(any(unix, windows)))]
            start: std::time::Instant::now(),
        }
    }
}
impl rt::InstantFactory for Clock {
    type Instant = rt::MuonInstant;
    fn now(&self) -> Self::Instant {
        #[cfg(any(unix, windows))]
        let now = crate::platform::boottime().expect("supported continuous monotonic clock");
        #[cfg(not(any(unix, windows)))]
        let now = self.start.elapsed();
        rt::MuonInstant::from_duration(now)
    }
}
impl rt::SystemTimeFactory for Clock {
    type SystemTime = rt::MuonSystemTime;
    fn now(&self) -> Self::SystemTime {
        rt::MuonSystemTime::since_unix_epoch(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default(),
        )
    }
}
// SAFETY: selected clocks are monotonic and never derive instants from wall time.
unsafe impl rt::Monotonic for Clock {}
impl rt::Sleep for Clock {
    type Sleep<'a> = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>;
    fn sleep(&self, duration: Duration) -> Self::Sleep<'static> {
        Box::pin(tokio::time::sleep(duration))
    }
}
#[derive(Debug, Clone)]
pub struct Os {
    pub network: InternalConnector,
    pub clock: Clock,
}
impl rt::OperatingSystem for Os {
    type Time = Clock;
    type TcpConnector = InternalConnector;
    type Resolver = InternalConnector;
    fn get_time_capabilities(&self) -> &Clock {
        &self.clock
    }
    fn get_tcp_connector(&self) -> &InternalConnector {
        &self.network
    }
    fn get_resolver(&self) -> &InternalConnector {
        &self.network
    }
}
impl rt::TcpConnect for InternalConnector {
    type Socket = TrackedStream;
    type Err = io::Error;
    async fn tcp_connect(&self, address: SocketAddr) -> io::Result<TrackedStream> {
        self.connect(address).await
    }
}
impl proton_vpn_rcrl::time::OperatingSystem for Os {
    type Clock = Clock;
    type Sleeper = Clock;
    fn get_clock(&self) -> Clock {
        self.clock.clone()
    }
    fn get_sleeper(&self) -> Clock {
        self.clock.clone()
    }
}
impl proton_vpn_local_agent::managed::OperatingSystem for Os {
    type TcpConnector = InternalConnector;
    type Sleeper = Clock;
    fn get_tcp_connector(&self) -> InternalConnector {
        self.network.clone()
    }
    fn get_sleeper(&self) -> Clock {
        self.clock.clone()
    }
}
impl rt::Resolve for InternalConnector {
    type Err = io::Error;
    async fn resolve(&self, host: &str) -> io::Result<Vec<IpAddr>> {
        if let Ok(ip) = host.parse() {
            return Ok(vec![ip]);
        }
        let (v4, v6) =
            futures::future::join(dns_query(self, host, 1), dns_query(self, host, 28)).await;
        let mut ips = Vec::new();
        if let Ok(v4) = v4 {
            ips.extend(v4);
        }
        if let Ok(v6) = v6 {
            ips.extend(v6);
        }
        if ips.is_empty() {
            Err(io::ErrorKind::NotFound.into())
        } else {
            Ok(ips)
        }
    }
}
async fn dns_query(network: &InternalConnector, host: &str, kind: u16) -> io::Result<Vec<IpAddr>> {
    tokio::time::timeout(Duration::from_secs(10), async {
        let mut random = [0; 2];
        getrandom::fill(&mut random).map_err(io::Error::other)?;
        let mut query = vec![random[0], random[1], 1, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        for label in host.trim_end_matches('.').split('.') {
            if label.is_empty() || label.len() > 63 || !label.is_ascii() {
                return Err(io::ErrorKind::InvalidInput.into());
            }
            query.push(label.len() as u8);
            query.extend_from_slice(label.as_bytes());
        }
        if query.len() > 266 {
            return Err(io::ErrorKind::InvalidInput.into());
        }
        query.push(0);
        query.extend_from_slice(&kind.to_be_bytes());
        query.extend_from_slice(&1u16.to_be_bytes());
        let destination = SocketAddr::new(network.addresses().v4(1).into(), 53);
        let socket = network.udp(destination)?;
        socket.send(&query).await?;
        let mut response = vec![0; 4096];
        let n = socket.recv(&mut response).await?;
        response.truncate(n);
        if response.len() < 12
            || response[..2] != random
            || response[2] & 0x80 == 0
            || response[3] & 15 != 0
        {
            return Err(io::ErrorKind::InvalidData.into());
        }
        if response[2] & 2 != 0 {
            let mut tcp = network.connect(destination).await?;
            tcp.write_all(&(query.len() as u16).to_be_bytes()).await?;
            tcp.write_all(&query).await?;
            tcp.flush().await?;
            let mut length = [0; 2];
            tcp.read_exact(&mut length).await?;
            let length = u16::from_be_bytes(length) as usize;
            if !(12..=65535).contains(&length) {
                return Err(io::ErrorKind::InvalidData.into());
            }
            response.resize(length, 0);
            tcp.read_exact(&mut response).await?;
        }
        decode_dns(&response, random, kind)
    })
    .await
    .map_err(|_| io::Error::from(io::ErrorKind::TimedOut))?
}
// RFC 1035 wire decoding. Names are skipped without recursively following
// compression pointers; every offset/record length is checked before slicing.
fn decode_dns(bytes: &[u8], id: [u8; 2], kind: u16) -> io::Result<Vec<IpAddr>> {
    fn bad() -> io::Error {
        io::ErrorKind::InvalidData.into()
    }
    fn name(bytes: &[u8], at: &mut usize) -> io::Result<()> {
        for _ in 0..128 {
            let length = *bytes.get(*at).ok_or_else(bad)?;
            *at += 1;
            if length == 0 {
                return Ok(());
            }
            if length & 0xc0 == 0xc0 {
                let next = *bytes.get(*at).ok_or_else(bad)?;
                *at += 1;
                if (((length & 63) as usize) << 8 | next as usize) >= bytes.len() {
                    return Err(bad());
                }
                return Ok(());
            }
            if length > 63 {
                return Err(bad());
            }
            *at += length as usize;
            if *at > bytes.len() {
                return Err(bad());
            }
        }
        Err(bad())
    }
    if bytes.len() < 12
        || bytes[..2] != id
        || bytes[2] & 0x80 == 0
        || bytes[2] & 2 != 0
        || bytes[3] & 15 != 0
    {
        return Err(bad());
    }
    let questions = u16::from_be_bytes([bytes[4], bytes[5]]);
    let answers = u16::from_be_bytes([bytes[6], bytes[7]]);
    if questions > 16 || answers > 1024 {
        return Err(bad());
    }
    let mut at = 12;
    for _ in 0..questions {
        name(bytes, &mut at)?;
        at += 4;
        if at > bytes.len() {
            return Err(bad());
        }
    }
    let mut addresses = Vec::new();
    for _ in 0..answers {
        name(bytes, &mut at)?;
        let header = bytes.get(at..at + 10).ok_or_else(bad)?;
        let record = u16::from_be_bytes([header[0], header[1]]);
        let class = u16::from_be_bytes([header[2], header[3]]);
        let length = u16::from_be_bytes([header[8], header[9]]) as usize;
        at += 10;
        let data = bytes.get(at..at + length).ok_or_else(bad)?;
        at += length;
        if class == 1 && record == kind {
            match (record, length) {
                (1, 4) => addresses.push(IpAddr::V4(Ipv4Addr::from(
                    <[u8; 4]>::try_from(data).expect("checked"),
                ))),
                (28, 16) => addresses.push(IpAddr::V6(Ipv6Addr::from(
                    <[u8; 16]>::try_from(data).expect("checked"),
                ))),
                _ => {}
            }
        }
    }
    Ok(addresses)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dns_decoder_accepts_compressed_answers_and_rejects_truncation() {
        let answer = [
            0x12, 0x34, 0x81, 0x80, 0, 1, 0, 1, 0, 0, 0, 0, 1, b'a', 0, 0, 1, 0, 1, 0xc0, 12, 0, 1,
            0, 1, 0, 0, 0, 60, 0, 4, 192, 0, 2, 1,
        ];
        assert_eq!(
            decode_dns(&answer, [0x12, 0x34], 1).unwrap(),
            vec!["192.0.2.1".parse::<IpAddr>().unwrap()]
        );
        for n in 0..answer.len() {
            assert!(decode_dns(&answer[..n], [0x12, 0x34], 1).is_err());
        }
        assert!(decode_dns(&answer, [0, 0], 1).is_err());
    }
}
