#![cfg(target_os = "linux")]
//! Opt-in CAP_NET_ADMIN test, run only inside a fresh CI network namespace.
mod common;
use base64::{Engine, engine::general_purpose::STANDARD};
use protun_next::{
    config::WireguardConfig,
    connection::{Connection, State},
    linux,
    packet::{Packet, PacketPool},
    session::{Output, Session},
    stream::{FramedIo, Receive},
    tls,
};
use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    os::fd::{AsFd, AsRawFd},
    process::Command,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
fn ip(args: &[&str]) {
    assert!(Command::new("ip").args(args).status().unwrap().success());
}
fn echo_ipv4(packet: &mut [u8], spoof: bool) {
    assert_eq!(packet[0] >> 4, 4);
    let header = (packet[0] & 15) as usize * 4;
    assert!(header >= 20 && packet.len() >= header + 8);
    assert_eq!(packet[9], 17);
    for n in 0..4 {
        packet.swap(12 + n, 16 + n);
    }
    for n in 0..2 {
        packet.swap(header + n, header + 2 + n);
    }
    if spoof {
        packet[12..16].copy_from_slice(&[10, 2, 0, 4]);
    }
    packet[header + 6..header + 8].fill(0);
    packet[10..12].fill(0);
    let mut sum: u32 = packet[..header]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| u16::from_be_bytes([p[0], p[1]]) as u32)
        .sum();
    while sum > 0xffff {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    packet[10..12].copy_from_slice(&(!(sum as u16)).to_be_bytes());
}
#[test]
#[ignore = "requires a disposable network namespace and CAP_NET_ADMIN; never run on a router"]
fn actual_udp_tcp_tls_engine_tun_roundtrips() {
    assert_eq!(
        std::env::var("PROTUN_NEXT_NETNS_TEST").as_deref(),
        Ok("1"),
        "run through the isolated CI namespace job"
    );
    let tun = File::options()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .unwrap();
    let mut request = [0u8; 40];
    request[..3].copy_from_slice(b"pn0");
    request[16..18].copy_from_slice(&((libc::IFF_TUN | libc::IFF_NO_PI) as i16).to_ne_bytes());
    // SAFETY: valid TUN descriptor and 40-byte ifreq storage, private test namespace.
    let configured = unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, request.as_mut_ptr()) };
    assert_eq!(configured, 0);
    ip(&["link", "set", "lo", "up"]);
    ip(&["addr", "add", "10.2.0.2/32", "dev", "pn0"]);
    ip(&["link", "set", "pn0", "mtu", "1280", "up"]);
    ip(&["route", "add", "10.2.0.3/32", "dev", "pn0"]);
    for mode in 0..8 {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = if mode == 0 {
            peer.local_addr().unwrap()
        } else {
            listener.local_addr().unwrap()
        };
        let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let now = linux::boottime().unwrap();
        let (release, wait_release) = std::sync::mpsc::channel();
        let reference = thread::spawn(move || {
            let mut transport = if mode == 0 {
                Reference::Udp(peer)
            } else {
                let deadline = std::time::Instant::now() + Duration::from_secs(5);
                let tcp = loop {
                    assert!(std::time::Instant::now() < deadline, "accept timeout");
                    match listener.accept() {
                        Ok((tcp, _)) => break tcp,
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            thread::sleep(Duration::from_millis(1))
                        }
                        Err(e) => panic!("accept: {e}"),
                    }
                };
                tcp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
                tcp.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
                tcp.set_nodelay(true).unwrap();
                let io: Box<dyn Duplex> = if mode >= 2 && mode != 7 {
                    Box::new(rustls::StreamOwned::new(
                        rustls::ServerConnection::new(common::server()).unwrap(),
                        tcp,
                    ))
                } else {
                    Box::new(tcp)
                };
                Reference::Stream(FramedIo::new(io, 8).unwrap())
            };
            let mut session =
                Session::new([9; 32], Session::public_key([7; 32]), None, now, wall).unwrap();
            let mut pool = PacketPool::new(8, 65535).unwrap();
            let mut wire = pool.take().unwrap();
            let mut out = [0; 65535];
            let mut reply = [0; 65535];
            loop {
                transport.receive(&mut wire);
                let result = match session.decrypt(wire.data(), &mut out) {
                    Ok(result) => result,
                    Err(_) if mode == 4 => {
                        // The TLS peer has no trusted certificate/name, but the
                        // wrong configured WG key must prevent authentication.
                        wait_release.recv_timeout(Duration::from_secs(10)).unwrap();
                        return;
                    }
                    Err(error) => panic!("reference decrypt: {error}"),
                };
                match result {
                    Output::Network(n) => transport.send(&out[..n], &mut pool),
                    Output::Tunnel(n) => {
                        echo_ipv4(&mut out[..n], mode == 5);
                        let Output::Network(n) = session.encrypt(&out[..n], &mut reply).unwrap()
                        else {
                            panic!("echo encrypt")
                        };
                        transport.send(&reply[..n], &mut pool);
                        if mode == 7 {
                            return;
                        }
                        wait_release.recv_timeout(Duration::from_secs(10)).unwrap();
                        return;
                    }
                    Output::None if mode == 6 => {
                        wait_release.recv_timeout(Duration::from_secs(10)).unwrap();
                        return;
                    }
                    _ => {}
                }
            }
        });
        let text = format!(
            "[Interface]\nPrivateKey={}\nAddress=10.2.0.2/32\nMTU=1280\n[Peer]\nPublicKey={}\nEndpoint={endpoint}\nAllowedIPs=10.2.0.3/32\n",
            STANDARD.encode([7; 32]),
            STANDARD.encode(Session::public_key(if mode == 4 {
                [11; 32]
            } else {
                [9; 32]
            }))
        );
        let mut config = WireguardConfig::parse(&text).unwrap();
        if mode == 6 {
            config.allowed_ips = vec!["10.2.0.4/32".into()];
        }
        let transport = match mode {
            0 => linux::Transport::Udp,
            1 | 7 => linux::Transport::Tcp,
            2 => linux::Transport::Tls(Box::new(
                tls::client_with_roots("localhost", common::roots()).unwrap(),
            )),
            _ => linux::Transport::Stealth(protun_next::stealth::SniStrategy::Explicit(
                "unrelated.test".into(),
            )),
        };
        let mut connection =
            Connection::start(tun.as_fd(), config, "lo".into(), transport).unwrap();
        if mode == 0 {
            connection.replace_tun(None).unwrap();
            let deadline = std::time::Instant::now() + Duration::from_secs(2);
            while connection.snapshot().tun_present {
                assert!(std::time::Instant::now() < deadline, "TUN detach deadline");
                let snapshot = connection.snapshot();
                connection.wait_after(snapshot.sequence, Duration::from_millis(50));
            }
            connection.replace_tun(Some(tun.as_fd())).unwrap();
            while !connection.snapshot().tun_present {
                assert!(
                    std::time::Instant::now() < deadline,
                    "TUN replacement deadline"
                );
                let snapshot = connection.snapshot();
                connection.wait_after(snapshot.sequence, Duration::from_millis(50));
            }
            let invalid = File::open("/dev/null").unwrap();
            assert!(connection.replace_tun(Some(invalid.as_fd())).is_err());
            assert!(
                connection.snapshot().tun_present,
                "invalid replacement must retain working TUN"
            );
        }
        let app = UdpSocket::bind("10.2.0.2:0").unwrap();
        app.set_read_timeout(Some(if (4..=6).contains(&mode) {
            Duration::from_millis(500)
        } else {
            Duration::from_secs(8)
        }))
        .unwrap();
        app.send_to(b"real kernel TUN and UDP", "10.2.0.3:9000")
            .unwrap();
        let mut response = [0; 128];
        let received = app.recv_from(&mut response);
        if mode == 7 {
            let (n, _) = received.unwrap();
            assert_eq!(
                &response[..n],
                b"real kernel TUN and UDP",
                "final validated reply must precede FIN failure"
            );
            let deadline = std::time::Instant::now() + Duration::from_secs(2);
            let mut snapshot = connection.snapshot();
            while !matches!(snapshot.state, State::Failed(_))
                && std::time::Instant::now() < deadline
            {
                snapshot = connection.wait_after(snapshot.sequence, Duration::from_millis(100));
            }
            assert!(matches!(snapshot.state, State::Failed(_)));
            assert!(connection.disconnect_and_wait().is_err());
            reference.join().unwrap();
            continue;
        }
        let request = connection.request_stats();
        let snapshot = connection.wait_for_stats(request, Duration::from_secs(2));
        assert!(
            snapshot.stats_generation >= request,
            "stats request must be sampled"
        );
        let statistics = snapshot
            .statistics
            .as_ref()
            .expect("real session statistics");
        if mode == 4 {
            assert_ne!(snapshot.state, State::TunnelEstablished);
            assert!(statistics.handshake_age.is_none());
            assert_eq!(statistics.received_bytes, 0);
        } else {
            assert_eq!(snapshot.state, State::TunnelEstablished);
            assert!(statistics.handshake_age.is_some());
            if mode == 6 {
                assert_eq!(statistics.sent_bytes, 0);
            } else {
                assert!(statistics.received_bytes > 0 && statistics.sent_bytes > 0);
            }
        }
        let counters = connection.disconnect_and_wait().unwrap();
        assert_eq!(connection.snapshot().state, State::Stopped);
        // Repeated shutdown/join must be safe and preserve final counters.
        assert_eq!(
            connection.disconnect_and_wait().unwrap().tun_reads,
            counters.tun_reads
        );
        release.send(()).unwrap();
        reference.join().unwrap();
        if mode >= 4 {
            let error = received.unwrap_err();
            assert!(matches!(
                error.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ));
            match mode {
                4 => assert_eq!(
                    counters.decrypted_packets, 0,
                    "untrusted TLS cannot authenticate a WG peer"
                ),
                5 => assert!(
                    counters.inbound_filtered > 0,
                    "authenticated wrong-source IP must be rejected"
                ),
                _ => assert!(
                    counters.outbound_filtered > 0,
                    "disallowed destination must not be encrypted"
                ),
            }
            continue;
        }
        let (n, source) = received.unwrap();
        assert_eq!(&response[..n], b"real kernel TUN and UDP");
        assert_eq!(source, "10.2.0.3:9000".parse().unwrap());
        assert!(counters.decrypted_packets >= 1);
        assert!(counters.tun_reads >= 1);
    }
}

trait Duplex: Read + Write + Send {}
impl Duplex for TcpStream {}
impl Duplex for rustls::StreamOwned<rustls::ServerConnection, TcpStream> {}
enum Reference {
    Udp(UdpSocket),
    Stream(FramedIo<Box<dyn Duplex>>),
}
impl Reference {
    fn receive(&mut self, packet: &mut Packet) {
        match self {
            Self::Udp(socket) => {
                let (n, source) = socket.recv_from(packet.writable()).unwrap();
                socket.connect(source).unwrap();
                packet.set_len(n).unwrap();
            }
            Self::Stream(stream) => loop {
                match stream.receive(packet).unwrap() {
                    Receive::Packet => break,
                    Receive::Yield => continue,
                    result => panic!("reference stream: {result:?}"),
                }
            },
        }
    }
    fn send(&mut self, bytes: &[u8], pool: &mut PacketPool) {
        match self {
            Self::Udp(socket) => {
                assert_eq!(socket.send(bytes).unwrap(), bytes.len());
            }
            Self::Stream(stream) => {
                let mut packet = pool.take().unwrap();
                packet.writable()[..bytes.len()].copy_from_slice(bytes);
                packet.set_len(bytes.len()).unwrap();
                assert!(stream.queue(packet).is_ok());
                while stream.pending() > 0 {
                    stream.flush(pool).unwrap();
                }
                stream.io_mut().flush().unwrap();
            }
        }
    }
}
