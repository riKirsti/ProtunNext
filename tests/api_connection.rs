#![cfg(target_os = "linux")]
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
#[test]
fn compatibility_supervisor_handshakes_without_tun_retains_peer_and_recovers_network() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_millis(40)))
        .unwrap();
    let address = socket.local_addr().unwrap();
    let stopped = Arc::new(AtomicBool::new(false));
    let stop = stopped.clone();
    let peer_thread = thread::spawn(move || {
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
                Ok((n, remote)) => {
                    if let Output::Network(n) = peer.decrypt(&bytes[..n], &mut out).unwrap() {
                        socket.send_to(&out[..n], remote).unwrap();
                    }
                }
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) => {}
                Err(error) => panic!("peer: {error}"),
            }
        }
    });
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
    wait_for(&event_rx, |e| matches!(e, Event::ConnectionStats { .. }));
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
