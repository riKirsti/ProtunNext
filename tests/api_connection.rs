#![cfg(unix)]
use protun_next::{
    api::{cache::MemoryCache, connection::*},
    linux,
    session::{Output, Session},
};
use std::{
    net::UdpSocket,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
fn wait_for<T>(receiver: &mpsc::Receiver<T>, predicate: impl Fn(&T) -> bool) -> T {
    let deadline = Instant::now() + Duration::from_secs(4);
    loop {
        let state = receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .expect("callback deadline");
        if predicate(&state) {
            return state;
        }
    }
}
fn echo_probe(packet: &mut [u8]) {
    assert_eq!(packet[0], 0x45);
    assert_eq!(packet[9], 1);
    assert_eq!(packet[20], 8);
    for at in 0..4 {
        packet.swap(12 + at, 16 + at);
    }
    packet[20] = 0;
    packet[22..24].fill(0);
    packet[10..12].fill(0);
    fn checksum(bytes: &[u8]) -> u16 {
        let (pairs, tail) = bytes.as_chunks::<2>();
        let mut sum: u32 = pairs.iter().map(|p| u16::from_be_bytes(*p) as u32).sum();
        if let Some(byte) = tail.first() {
            sum += (*byte as u32) << 8;
        }
        while sum > 65535 {
            sum = (sum & 65535) + (sum >> 16);
        }
        !(sum as u16)
    }
    let ip = checksum(&packet[..20]);
    packet[10..12].copy_from_slice(&ip.to_be_bytes());
    let icmp = checksum(&packet[20..]);
    packet[22..24].copy_from_slice(&icmp.to_be_bytes());
}
#[test]
fn compatibility_supervisor_handshakes_without_tun_retains_peer_and_recovers_network() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_millis(40)))
        .unwrap();
    let address = socket.local_addr().unwrap();
    let stopped = Arc::new(AtomicBool::new(false));
    let peer_thread = reference_peer(socket, stopped.clone(), true);
    let mut peer = PeerInfo {
        peer_id: "first".into(),
        server_ip: IpAddress(address.ip()),
        server_public_key: WgPeerPublicKey(Session::public_key([9; 32])),
        udp_ports: vec![address.port()],
        tcp_ports: vec![],
        tls_ports: vec![],
        priority: 0,
        exit_label: None,
        server_name: None,
    };
    let (states, state_rx) = mpsc::channel();
    let (events, event_rx) = mpsc::channel();
    let protected = Arc::new(AtomicUsize::new(0));
    let protected_count = protected.clone();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("capture.pcap");
    let capture = PcapFileInfo {
        file: PcapFile::Path {
            path: path.to_str().unwrap().into(),
            mode: FileWriteMode::Overwrite,
        },
        max_bytes: None,
    };
    let started = Instant::now();
    let connection = Connection::unix_connect(
        InitialConnectionConfig {
            peers: vec![peer.clone()],
            network_available: true,
            pcap_file: Some(capture),
            connection_mode: ConnectionMode::NoLocalAgent {
                wg_private_key: Some(WgClientPrivateKey([7; 32])),
            },
            sni_strategy: SniStrategy::Random,
        },
        None,
        Box::new(move |state| {
            states.send(state).unwrap();
        }),
        Box::new(move |event| {
            events.send(event).unwrap();
        }),
        Some(Box::new(move |_| {
            protected_count.fetch_add(1, Ordering::Relaxed);
            true
        })),
        Box::new(MemoryCache::default()),
    );
    let state = wait_for(&state_rx, |s| {
        matches!(s.connection_state, ConnectionState::Connected { .. })
    });
    assert!(matches!(
        state.interface_state,
        InterfaceState::Down { last_error: None }
    ));
    let first_size = std::fs::metadata(&path).unwrap().len();
    assert!(first_size > 24);
    peer.peer_id = "retained".into();
    connection.update_peers(vec![peer]);
    wait_for(
        &state_rx,
        |s| matches!(&s.connection_state, ConnectionState::Connected { peer, .. } if peer.peer_id == "retained"),
    );
    assert_eq!(
        protected.load(Ordering::Relaxed),
        1,
        "metadata update must retain socket/session"
    );
    connection.request_stats();
    let Event::ConnectionStats {
        mono_timestamp_ms: first_timestamp,
        ..
    } = wait_for(&event_rx, |e| matches!(e, Event::ConnectionStats { .. }))
    else {
        unreachable!()
    };
    assert!(
        first_timestamp >= 0 && first_timestamp as u128 <= started.elapsed().as_millis() + 100,
        "API timestamp must be relative to the connection, not OS uptime"
    );
    thread::sleep(Duration::from_millis(80));
    connection.on_connectivity_change(ConnectivityEvent::Down);
    wait_for(
        &state_rx,
        |s| matches!(&s.connection_state, ConnectionState::Connecting { wait_reasons, .. } if !wait_reasons.is_empty()),
    );
    connection.on_connectivity_change(ConnectivityEvent::Up);
    wait_for(&state_rx, |s| {
        matches!(s.connection_state, ConnectionState::Connected { .. })
    });
    assert_eq!(protected.load(Ordering::Relaxed), 2);
    connection.request_stats();
    let Event::ConnectionStats {
        mono_timestamp_ms: second_timestamp,
        ..
    } = wait_for(&event_rx, |e| matches!(e, Event::ConnectionStats { .. }))
    else {
        unreachable!()
    };
    assert!(
        second_timestamp >= first_timestamp + 80,
        "recreated worker must retain the connection's timestamp origin"
    );
    assert!(second_timestamp as u128 <= started.elapsed().as_millis() + 100);
    assert!(
        std::fs::metadata(&path).unwrap().len() > first_size,
        "failover must preserve rather than truncate capture"
    );
    connection.disconnect_and_wait();
    connection.disconnect_and_wait();
    wait_for(&state_rx, |s| {
        matches!(s.connection_state, ConnectionState::Disconnected { .. })
    });
    wait_for(&event_rx, |e| {
        matches!(
            e,
            Event::PacketCaptureStopped {
                reason: CaptureStopReason::Disconnected { .. }
            }
        )
    });
    stopped.store(true, Ordering::Relaxed);
    peer_thread.join().unwrap();
}

fn reference_peer(
    socket: UdpSocket,
    stop: Arc<AtomicBool>,
    answer_probe: bool,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut peer = Session::new(
            [9; 32],
            Session::public_key([7; 32]),
            None,
            linux::boottime().unwrap(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        )
        .unwrap();
        let mut bytes = [0; 65535];
        let mut out = [0; 65535];
        while !stop.load(Ordering::Relaxed) {
            match socket.recv_from(&mut bytes) {
                Ok((n, remote)) => match peer.decrypt(&bytes[..n], &mut out).unwrap() {
                    Output::Network(n) => {
                        socket.send_to(&out[..n], remote).unwrap();
                    }
                    Output::Tunnel(n) if answer_probe => {
                        echo_probe(&mut out[..n]);
                        let Output::Network(n) = peer.encrypt(&out[..n], &mut bytes).unwrap()
                        else {
                            panic!("probe response encryption")
                        };
                        socket.send_to(&bytes[..n], remote).unwrap();
                    }
                    _ => {}
                },
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) => {}
                Err(error) => panic!("peer: {error}"),
            }
        }
    })
}
#[test]
fn handshake_without_decrypted_data_fails_over_to_a_working_data_path() {
    let first = UdpSocket::bind("127.0.0.1:0").unwrap();
    let second = UdpSocket::bind("127.0.0.1:0").unwrap();
    first
        .set_read_timeout(Some(Duration::from_millis(40)))
        .unwrap();
    second
        .set_read_timeout(Some(Duration::from_millis(40)))
        .unwrap();
    let failed_port = first.local_addr().unwrap().port();
    let good_port = second.local_addr().unwrap().port();
    let stop = Arc::new(AtomicBool::new(false));
    let bad = reference_peer(first, stop.clone(), false);
    let good = reference_peer(second, stop.clone(), true);
    let peer = PeerInfo {
        peer_id: "probe-failover".into(),
        server_ip: IpAddress("127.0.0.1".parse().unwrap()),
        server_public_key: WgPeerPublicKey(Session::public_key([9; 32])),
        udp_ports: vec![failed_port, good_port],
        tcp_ports: vec![],
        tls_ports: vec![],
        priority: 0,
        exit_label: None,
        server_name: None,
    };
    let (sender, states) = mpsc::channel();
    let connection = Connection::unix_connect(
        InitialConnectionConfig {
            peers: vec![peer],
            network_available: true,
            pcap_file: None,
            connection_mode: ConnectionMode::NoLocalAgent {
                wg_private_key: Some(WgClientPrivateKey([7; 32])),
            },
            sni_strategy: SniStrategy::Random,
        },
        None,
        Box::new(move |state| {
            sender.send(state).unwrap();
        }),
        Box::new(|_: Event| {}),
        None,
        Box::new(MemoryCache::default()),
    );
    let deadline = Instant::now() + Duration::from_secs(12);
    loop {
        let state = states
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .expect("probe failover deadline");
        if let ConnectionState::Connected { peer, .. } = state.connection_state {
            assert_eq!(
                peer.port, good_port,
                "handshake-only peer must never be reported connected"
            );
            break;
        }
    }
    connection.disconnect_and_wait();
    stop.store(true, Ordering::Relaxed);
    bad.join().unwrap();
    good.join().unwrap();
}
#[test]
fn disconnect_callback_can_reenter_shutdown_while_another_thread_joins() {
    let slot = Arc::new(std::sync::OnceLock::<std::sync::Weak<Connection>>::new());
    let from_callback = slot.clone();
    let connection = Arc::new(Connection::unix_connect(
        InitialConnectionConfig {
            peers: vec![],
            network_available: false,
            pcap_file: None,
            connection_mode: ConnectionMode::NoLocalAgent {
                wg_private_key: Some(WgClientPrivateKey([7; 32])),
            },
            sni_strategy: SniStrategy::Random,
        },
        None,
        Box::new(move |state: VpnState| {
            if matches!(state.connection_state, ConnectionState::Disconnected { .. })
                && let Some(connection) = from_callback.get().and_then(std::sync::Weak::upgrade)
            {
                connection.disconnect_and_wait();
            }
        }),
        Box::new(|_: Event| {}),
        None,
        Box::new(MemoryCache::default()),
    ));
    slot.set(Arc::downgrade(&connection)).unwrap();
    let joining = connection.clone();
    let (done, wait) = mpsc::channel();
    let thread = thread::spawn(move || {
        joining.disconnect_and_wait();
        done.send(()).unwrap();
    });
    wait.recv_timeout(Duration::from_secs(2))
        .expect("callback shutdown deadlocked with join mutex");
    thread.join().unwrap();
}
