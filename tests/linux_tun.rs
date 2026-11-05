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
    let sum = |bytes: &[u8], seed: u32| {
        let mut value = seed;
        for pair in bytes.chunks(2) {
            value += u16::from_be_bytes([pair[0], *pair.get(1).unwrap_or(&0)]) as u32;
        }
        while value > 0xffff {
            value = (value & 0xffff) + (value >> 16);
        }
        value
    };
    assert_eq!(sum(&packet[..header], 0), 0xffff, "outbound IPv4 checksum");
    if packet[header + 6..header + 8] != [0, 0] {
        // Independent RFC 768 pseudoheader check at the receiving peer.
        let pseudo = sum(&packet[12..20], 17 + (packet.len() - header) as u32);
        assert_eq!(
            sum(&packet[header..], pseudo),
            0xffff,
            "outbound UDP checksum"
        );
    }
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
    tun_roundtrips();
}
fn tun_roundtrips() {
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
    let flags = libc::IFF_TUN | libc::IFF_NO_PI;
    request[16..18].copy_from_slice(&(flags as i16).to_ne_bytes());
    // SAFETY: valid TUN descriptor and 40-byte ifreq storage, private test namespace.
    let configured = unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, request.as_mut_ptr()) };
    assert_eq!(configured, 0);
    ip(&["link", "set", "lo", "up"]);
    ip(&["addr", "add", "10.2.0.2/32", "dev", "pn0"]);
    ip(&["link", "set", "pn0", "mtu", "1500", "up"]);
    ip(&["route", "add", "10.2.0.3/32", "dev", "pn0"]);
    for mode in [0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 17, 18, 19] {
        let scenario = match mode {
            16 => 4,
            17 => 5,
            18 => 6,
            19 => 7,
            _ => mode,
        };
        let udp_mode = matches!(mode, 0 | 8 | 9 | 10 | 13 | 15);
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = if udp_mode {
            peer.local_addr().unwrap()
        } else {
            listener.local_addr().unwrap()
        };
        let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let now = linux::boottime().unwrap();
        let (release, wait_release) = std::sync::mpsc::channel();
        let reference = thread::spawn(move || {
            let mut transport = if udp_mode {
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
                let io: Box<dyn Duplex> = if mode >= 2 && !matches!(mode, 7 | 14) {
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
            let mut echoes = if mode == 15 { 3 } else { 1 };
            loop {
                transport.receive(&mut wire);
                let result = match session.decrypt(wire.data(), &mut out) {
                    Ok(result) => result,
                    Err(_) if scenario == 4 => {
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
                        echo_ipv4(&mut out[..n], scenario == 5);
                        let Output::Network(n) = session.encrypt(&out[..n], &mut reply).unwrap()
                        else {
                            panic!("echo encrypt")
                        };
                        transport.send(&reply[..n], &mut pool);
                        echoes -= 1;
                        if echoes > 0 {
                            continue;
                        }
                        if scenario == 7 {
                            return;
                        }
                        wait_release.recv_timeout(Duration::from_secs(10)).unwrap();
                        return;
                    }
                    Output::None if scenario == 6 => {
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
            STANDARD.encode(Session::public_key(if scenario == 4 {
                [11; 32]
            } else {
                [9; 32]
            }))
        );
        let mut config = WireguardConfig::parse(&text).unwrap();
        if scenario == 6 {
            config.allowed_ips = vec!["10.2.0.4/32".into()];
        }
        let transport = match mode {
            0 | 8 | 9 | 10 | 13 | 15 => linux::Transport::Udp,
            1 | 7 | 14 => linux::Transport::Tcp,
            12 => linux::Transport::Tls(Box::new(
                tls::client_with_roots("localhost", common::roots()).unwrap(),
            )),
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
        app.set_read_timeout(Some(if (4..=6).contains(&scenario) {
            Duration::from_millis(500)
        } else {
            Duration::from_secs(8)
        }))
        .unwrap();
        // The caller-owned interface MTU exceeds the worker's initial 1280.
        // A full 1500-byte IP packet must survive without a truncated tail.
        let payload = [0x5a; 1472];
        if mode == 15 {
            let segment_size: libc::c_int = 500;
            assert_eq!(
                // SAFETY: scalar setting on an owned socket in a disposable namespace.
                unsafe {
                    libc::setsockopt(
                        app.as_raw_fd(),
                        libc::SOL_UDP,
                        libc::UDP_SEGMENT,
                        (&segment_size as *const libc::c_int).cast(),
                        size_of::<libc::c_int>() as _,
                    )
                },
                0
            );
        }
        app.send_to(&payload, "10.2.0.3:9000").unwrap();
        let mut response = [0; 1500];
        let received = app.recv_from(&mut response);
        if mode == 15 {
            let (n, from) = received.as_ref().unwrap();
            assert_eq!(*n, 500);
            assert_eq!(&response[..*n], &payload[..500]);
            assert_eq!(*from, "10.2.0.3:9000".parse().unwrap());
            for count in [500, 472] {
                let (n, from) = app.recv_from(&mut response).unwrap();
                assert_eq!(n, count);
                assert_eq!(&response[..n], &payload[..count]);
                assert_eq!(from, "10.2.0.3:9000".parse().unwrap());
            }
        }
        if scenario == 7 {
            let (n, _) = received.unwrap();
            assert_eq!(
                &response[..n],
                payload,
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
        if scenario == 4 {
            assert_ne!(snapshot.state, State::TunnelEstablished);
            assert!(statistics.handshake_age.is_none());
            assert_eq!(statistics.received_bytes, 0);
        } else {
            assert_eq!(snapshot.state, State::TunnelEstablished);
            assert!(statistics.handshake_age.is_some());
            if scenario == 6 {
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
        if (4..=6).contains(&scenario) {
            let error = received.unwrap_err();
            assert!(matches!(
                error.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ));
            match scenario {
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
        if mode == 15 {
            assert_eq!(n, 500);
        } else {
            assert_eq!(&response[..n], payload);
        }
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

#[cfg(feature = "local-agent")]
#[test]
#[ignore = "requires a disposable network namespace and CAP_NET_ADMIN; never run on a router"]
fn tracked_customer_reply_outside_virtual_subnet_survives_the_real_tun_pipeline() {
    use protun_next::api::{cache::MemoryCache, connection as api};
    use std::{
        os::fd::IntoRawFd,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
            mpsc,
        },
    };
    assert_eq!(std::env::var("PROTUN_NEXT_NETNS_TEST").as_deref(), Ok("1"));
    let tun = File::options()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .unwrap();
    let mut request = [0u8; 40];
    request[..7].copy_from_slice(b"pntrack");
    request[16..18].copy_from_slice(&((libc::IFF_TUN | libc::IFF_NO_PI) as i16).to_ne_bytes());
    assert_eq!(
        // SAFETY: owned TUN file and complete ifreq buffer, only in private namespace.
        unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, request.as_mut_ptr()) },
        0
    );
    ip(&["link", "set", "lo", "up"]);
    ip(&["addr", "add", "10.2.0.2/32", "dev", "pntrack"]);
    ip(&["addr", "add", "192.0.2.2/32", "dev", "pntrack"]);
    ip(&["link", "set", "pntrack", "mtu", "1420", "up"]);
    ip(&["route", "add", "192.0.2.100/32", "dev", "pntrack"]);
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_millis(40)))
        .unwrap();
    let endpoint = socket.local_addr().unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let stopped = stop.clone();
    let peer = thread::spawn(move || {
        let mut session = Session::new(
            [9; 32],
            Session::public_key([7; 32]),
            None,
            linux::boottime().unwrap(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        )
        .unwrap();
        let mut input = [0; 65535];
        let mut output = [0; 65535];
        while !stopped.load(Ordering::Relaxed) {
            match socket.recv_from(&mut input) {
                Ok((n, remote)) => match session.decrypt(&input[..n], &mut output).unwrap() {
                    Output::Network(n) => {
                        socket.send_to(&output[..n], remote).unwrap();
                    }
                    // The kernel can also emit IPv6 router solicitation on a
                    // newly created TUN. This reference only echoes IPv4 traffic.
                    Output::Tunnel(n) if n >= 28 && output[0] >> 4 == 4 => {
                        if output[9] == 1 {
                            assert_eq!(output[20], 8);
                            for index in 0..4 {
                                output.swap(12 + index, 16 + index);
                            }
                            output[20] = 0;
                            output[22..24].fill(0);
                            let (pairs, tail) = output[20..n].as_chunks::<2>();
                            let mut sum: u32 = pairs
                                .iter()
                                .map(|p| u32::from(u16::from_be_bytes(*p)))
                                .sum();
                            if let Some(byte) = tail.first() {
                                sum += u32::from(*byte) << 8;
                            }
                            while sum > 65535 {
                                sum = (sum & 65535) + (sum >> 16);
                            }
                            output[22..24].copy_from_slice(&(!(sum as u16)).to_be_bytes());
                        } else {
                            echo_ipv4(&mut output[..n], false);
                        }
                        let Output::Network(n) = session.encrypt(&output[..n], &mut input).unwrap()
                        else {
                            panic!("echo encryption")
                        };
                        socket.send_to(&input[..n], remote).unwrap();
                    }
                    _ => {}
                },
                Err(error)
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                    ) => {}
                Err(error) => panic!("peer: {error}"),
            }
        }
    });
    let (sender, states) = mpsc::channel();
    let connection = api::Connection::unix_connect(
        api::InitialConnectionConfig {
            peers: vec![api::PeerInfo {
                peer_id: "tracked route".into(),
                server_ip: api::IpAddress(endpoint.ip()),
                server_public_key: api::WgPeerPublicKey(Session::public_key([9; 32])),
                udp_ports: vec![endpoint.port()],
                tcp_ports: vec![],
                tls_ports: vec![],
                priority: 0,
                exit_label: None,
                server_name: None,
            }],
            network_available: true,
            pcap_file: None,
            connection_mode: api::ConnectionMode::NoLocalAgent {
                wg_private_key: Some(api::WgClientPrivateKey([7; 32])),
            },
            sni_strategy: api::SniStrategy::Random,
        },
        Some(tun.try_clone().unwrap().into_raw_fd()),
        Box::new(move |state| {
            let _ = sender.send(state);
        }),
        Box::new(|_| {}),
        Some(Box::new(|_| true)),
        Box::new(MemoryCache::default()),
    );
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        let state = states
            .recv_timeout(deadline.saturating_duration_since(std::time::Instant::now()))
            .unwrap();
        if matches!(
            state.connection_state,
            api::ConnectionState::Connected { .. }
        ) {
            break;
        }
    }
    let app = UdpSocket::bind("192.0.2.2:1000").unwrap();
    app.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
    app.send_to(b"tracked customer", "192.0.2.100:9000")
        .unwrap();
    let mut bytes = [0; 64];
    let (n, from) = app.recv_from(&mut bytes).unwrap();
    assert_eq!(&bytes[..n], b"tracked customer");
    assert_eq!(from, "192.0.2.100:9000".parse().unwrap());
    connection.disconnect_and_wait();
    stop.store(true, Ordering::Relaxed);
    peer.join().unwrap();
}

#[cfg(feature = "control-plane")]
#[test]
#[ignore = "requires a disposable network namespace and CAP_NET_ADMIN; never run on a router"]
fn local_http_exists_offline_and_its_tcp_session_survives_worker_replacement() {
    use protun_next::api::{cache::MemoryCache, connection::*};
    use std::{os::fd::IntoRawFd, sync::mpsc};
    assert_eq!(std::env::var("PROTUN_NEXT_NETNS_TEST").as_deref(), Ok("1"));
    let tun = File::options()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .unwrap();
    let mut request = [0u8; 40];
    request[..5].copy_from_slice(b"pnweb");
    request[16..18].copy_from_slice(&((libc::IFF_TUN | libc::IFF_NO_PI) as i16).to_ne_bytes());
    // SAFETY: complete ifreq buffer and owned TUN descriptor in disposable namespace.
    let result = unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, request.as_mut_ptr()) };
    assert_eq!(result, 0);
    ip(&["addr", "add", "10.2.0.2/32", "dev", "pnweb"]);
    ip(&["link", "set", "pnweb", "up"]);
    ip(&["route", "add", "10.2.0.3/32", "dev", "pnweb"]);
    // An isolated on-link destination with no ARP respondent leaves SYN pending.
    // No packets can leave this namespace through the dummy device.
    ip(&["link", "add", "pnuplink", "type", "dummy"]);
    ip(&["addr", "add", "198.51.100.2/24", "dev", "pnuplink"]);
    ip(&["link", "set", "pnuplink", "up"]);
    let (states, receiver) = mpsc::channel();
    let connection = protun_next::api::connection::Connection::unix_connect(
        InitialConnectionConfig {
            peers: vec![],
            connection_mode: ConnectionMode::NoLocalAgent {
                wg_private_key: Some(WgClientPrivateKey([7; 32])),
            },
            network_available: false,
            pcap_file: None,
            sni_strategy: SniStrategy::Random,
        },
        Some(tun.into_raw_fd()),
        Box::new(move |state| {
            let _ = states.send(state);
        }),
        Box::new(|_| {}),
        None,
        Box::new(MemoryCache::default()),
    );
    let wait = |offline: bool| {
        let deadline = std::time::Instant::now() + Duration::from_secs(4);
        loop {
            let state: VpnState = receiver
                .recv_timeout(deadline.saturating_duration_since(std::time::Instant::now()))
                .unwrap();
            if let ConnectionState::Connecting { wait_reasons, .. } = state.connection_state
                && wait_reasons.contains(&PeerConnectionWaitReason::WaitingForNetwork) == offline
            {
                break;
            }
        }
    };
    let open = || {
        let deadline = std::time::Instant::now() + Duration::from_secs(3);
        loop {
            match TcpStream::connect_timeout(
                &"10.2.0.3:80".parse().unwrap(),
                Duration::from_millis(100),
            ) {
                Ok(stream) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(3)))
                        .unwrap();
                    stream
                        .set_write_timeout(Some(Duration::from_secs(3)))
                        .unwrap();
                    break stream;
                }
                Err(error) => assert!(
                    std::time::Instant::now() < deadline,
                    "local service startup: {error}"
                ),
            }
        }
    };
    let finish = |mut stream: TcpStream| {
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: local\r\n\r\n")
            .unwrap();
        let mut reply = Vec::new();
        stream.read_to_end(&mut reply).unwrap();
        assert!(reply.starts_with(b"HTTP/1.1 200 OK\r\n"));
        assert!(reply.ends_with(b"Hello World"));
    };
    wait(true);
    finish(open());
    let persistent_tcp = open();
    connection.update_peers(vec![PeerInfo {
        peer_id: "unreachable TCP in namespace".into(),
        server_ip: IpAddress("198.51.100.100".parse().unwrap()),
        server_public_key: WgPeerPublicKey(Session::public_key([9; 32])),
        udp_ports: vec![],
        tcp_ports: vec![443],
        tls_ports: vec![],
        priority: 0,
        exit_label: None,
        server_name: None,
    }]);
    connection.on_connectivity_change(ConnectivityEvent::Up);
    wait(false);
    finish(persistent_tcp);
    connection.on_connectivity_change(ConnectivityEvent::Down);
    wait(true);
    finish(open());
    connection.disconnect_and_wait();
}
