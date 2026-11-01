#![cfg(feature = "local-agent")]
use futures::{AsyncReadExt, AsyncWriteExt};
use proton_boringtun::proton_os_interface::{lock::strict::Strict, rand::SeedableRng};
use proton_vpn_netstack::{NetworkConfig, NetworkConfigV4, NetworkStack, error::DeviceReadError};
use protun_next::api::internal::InternalNetwork;
use rand_chacha::ChaCha20Rng;
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::Wake,
    time::{Duration, Instant},
};
struct CountWake(Arc<AtomicUsize>);
impl Wake for CountWake {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}
fn stack(
    address: &str,
) -> (
    proton_vpn_netstack::TransportFactory,
    proton_vpn_netstack::NetworkInterface,
) {
    NetworkStack::builder()
        .with_time(0)
        .with_config(NetworkConfig::V4(NetworkConfigV4::new(
            address.parse().unwrap(),
            24,
            None,
        )))
        .with_rng(ChaCha20Rng::from_seed([42; 32]))
        .with_mtu(1420)
        .with_tcp_rx_buffer_size(65536)
        .with_tcp_tx_buffer_size(65536)
        .build::<Strict>()
        .into_sides()
}

#[test]
fn customer_and_private_socket_with_identical_tuple_receive_their_own_udp_replies() {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let (control, mut router) =
                InternalNetwork::new(Duration::ZERO, CountWake(Arc::new(AtomicUsize::new(0))))
                    .unwrap();
            let (customer, mut customer_wire) = stack("10.2.0.2");
            let (server, mut peer_wire) = stack("10.2.0.1");
            let address: std::net::SocketAddr = "10.2.0.1:53".parse().unwrap();
            let private = control.udp(address).unwrap(); // first privileged wire port: 4
            let application = customer
                .udp_bind("10.2.0.2:4".parse::<std::net::SocketAddr>().unwrap())
                .unwrap()
                .connect(address);
            let echo = server.udp_bind(address).unwrap();
            let exchanges = async {
                let a = async {
                    private.send(b"private DNS").await.unwrap();
                    let mut bytes = [0; 32];
                    let n = private.recv(&mut bytes).await.unwrap();
                    assert_eq!(&bytes[..n], b"private DNS");
                };
                let b = async {
                    application.send(b"customer DNS").await.unwrap();
                    let mut bytes = [0; 32];
                    let n = application.recv(&mut bytes).await.unwrap();
                    assert_eq!(&bytes[..n], b"customer DNS");
                };
                let c = async {
                    let mut ports = std::collections::HashSet::new();
                    for _ in 0..2 {
                        let mut bytes = [0; 32];
                        let (n, from) = echo.recv_from(&mut bytes).await.unwrap();
                        ports.insert(from.port());
                        echo.sendto(&bytes[..n], from.into()).await.unwrap();
                    }
                    assert_eq!(ports, std::collections::HashSet::from([4, 1000]));
                };
                futures::join!(a, b, c);
            };
            let pump = async {
                let start = Instant::now();
                let mut bytes = [0; 65535];
                loop {
                    let now = start.elapsed();
                    let micros = now.as_micros() as i64;
                    for _ in 0..64 {
                        let Some(n) = router.read_packet(&mut bytes, now).unwrap() else {
                            break;
                        };
                        peer_wire.write_packet(&bytes[..n], micros).unwrap();
                    }
                    for _ in 0..64 {
                        match customer_wire.read_packet(&mut bytes, micros) {
                            Ok(n) => {
                                router.customer_outbound(&mut bytes[..n]).unwrap();
                                peer_wire.write_packet(&bytes[..n], micros).unwrap();
                            }
                            Err(DeviceReadError::WouldBlock) => break,
                            Err(e) => panic!("customer read: {e:?}"),
                        }
                    }
                    for _ in 0..64 {
                        match peer_wire.read_packet(&mut bytes, micros) {
                            Ok(n) => {
                                if !router.receive(&mut bytes[..n], now).unwrap() {
                                    customer_wire.write_packet(&bytes[..n], micros).unwrap();
                                }
                            }
                            Err(DeviceReadError::WouldBlock) => break,
                            Err(e) => panic!("peer read: {e:?}"),
                        }
                    }
                    assert!(
                        now < Duration::from_secs(3),
                        "colliding flow exchange timed out"
                    );
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            };
            tokio::select! { _=exchanges=>{}, _=pump=>unreachable!() }
        });
}

#[test]
fn private_tcp_and_dns_datagrams_cross_real_ip_stack_with_port_translation() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    runtime.block_on(async {
        let wakes = Arc::new(AtomicUsize::new(0));
        let (client, mut wire) =
            InternalNetwork::new(Duration::ZERO, CountWake(wakes.clone())).unwrap();
        let stack = NetworkStack::builder()
            .with_time(0)
            .with_config(NetworkConfig::V4(NetworkConfigV4::new(
                "10.2.0.1".parse().unwrap(),
                24,
                None,
            )))
            .with_rng(ChaCha20Rng::from_seed([43; 32]))
            .with_mtu(1420)
            .with_tcp_rx_buffer_size(65536)
            .with_tcp_tx_buffer_size(65536)
            .build::<Strict>();
        let (server, mut peer) = stack.into_sides();
        let mut listener = server
            .tcp_bind("10.2.0.1:65433".parse::<std::net::SocketAddr>().unwrap())
            .unwrap();
        let dns = server
            .udp_bind("10.2.0.1:53".parse::<std::net::SocketAddr>().unwrap())
            .unwrap();
        let exchange = async {
            let client_tcp = async {
                let mut stream = client
                    .connect("10.2.0.1:65433".parse().unwrap())
                    .await
                    .unwrap();
                stream.write_all(b"private control TCP").await.unwrap();
                stream.flush().await.unwrap();
                let mut reply = [0; 19];
                stream.read_exact(&mut reply).await.unwrap();
                assert_eq!(&reply, b"private control TCP");
                tokio::time::sleep(Duration::from_millis(60)).await;
            };
            let server_tcp = async {
                let (mut stream, remote) = listener.accept().await.unwrap();
                assert!(
                    remote.port() < 1024,
                    "wire source must use a privileged control port"
                );
                let mut bytes = [0; 19];
                stream.read_exact(&mut bytes).await.unwrap();
                stream.write_all(&bytes).await.unwrap();
                stream.flush().await.unwrap();
                // Retain the socket until the client receives the buffered reply.
                tokio::time::sleep(Duration::from_millis(40)).await;
            };
            let client_udp = async {
                let socket = client.udp("10.2.0.1:53".parse().unwrap()).unwrap();
                socket.send(b"DNS request").await.unwrap();
                let mut reply = [0; 1024];
                let n = socket.recv(&mut reply).await.unwrap();
                assert_eq!(&reply[..n], &[0x5a; 1024]);
            };
            let server_udp = async {
                let mut bytes = [0; 32];
                let (n, from) = dns.recv_from(&mut bytes).await.unwrap();
                assert_eq!(&bytes[..n], b"DNS request");
                assert!(from.port() < 1024);
                dns.sendto(&[0x5a; 1024], from.into()).await.unwrap();
            };
            futures::join!(client_tcp, server_tcp, client_udp, server_udp);
        };
        let pump = async {
            let started = Instant::now();
            let mut packet = [0; 65535];
            let mut fragments_delivered = 0;
            loop {
                let now = started.elapsed();
                for _ in 0..64 {
                    let Some(n) = wire.read_packet(&mut packet, now).unwrap() else {
                        break;
                    };
                    peer.write_packet(&packet[..n], now.as_micros() as i64)
                        .unwrap();
                }
                for _ in 0..64 {
                    match peer.read_packet(&mut packet, now.as_micros() as i64) {
                        Ok(n) => {
                            assert!(InternalNetwork::might_receive(&packet[..n]));
                            if packet[9] == 17 && n > 200 {
                                // A real UDP reply from the server IP stack is
                                // fragmented on its wire path. All fragments
                                // must reach the private reassembler, never TUN.
                                for mut fragment in ipv4_fragments(&packet[..n], 200) {
                                    assert!(InternalNetwork::might_receive(&fragment));
                                    assert!(wire.receive(&mut fragment, now).unwrap());
                                    fragments_delivered += 1;
                                }
                            } else {
                                assert!(wire.receive(&mut packet[..n], now).unwrap());
                            }
                        }
                        Err(DeviceReadError::WouldBlock) => break,
                        Err(error) => panic!("peer read: {error:?}"),
                    }
                }
                assert!(
                    started.elapsed() < Duration::from_secs(3),
                    "in-memory control exchange timed out after {fragments_delivered} fragments"
                );
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        };
        tokio::select! { _ = exchange => {}, _ = pump => unreachable!() }
        assert!(
            wakes.load(Ordering::Relaxed) > 0,
            "control socket writes must wake the packet loop"
        );
    });
}

#[test]
fn alternate_subnet_private_dns_and_local_http_use_the_selected_network() {
    use protun_next::api::{account::Service, cache::MemoryCache, types::*};
    use protun_next::{addressing::Addresses, session::Session};
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let addresses = Addresses::new(37).unwrap();
            let (private, mut router) = InternalNetwork::with_addresses(
                Duration::ZERO,
                CountWake(Arc::new(AtomicUsize::new(0))),
                addresses,
            )
            .unwrap();
            let (customer, mut app_wire) = stack("10.37.0.2");
            let (server, mut peer_wire) = stack("10.37.0.1");
            let dns_address = std::net::SocketAddr::new(addresses.v4(1).into(), 53);
            let dns = server.udp_bind(dns_address).unwrap();
            let service = Service::start(
                private.clone(),
                ConnectionMode::NoLocalAgent {
                    wg_private_key: Some(WgClientPrivateKey([7; 32])),
                },
                PeerInfo {
                    peer_id: "private fixture".into(),
                    server_ip: IpAddress(addresses.v4(1).into()),
                    server_public_key: WgPeerPublicKey(Session::public_key([9; 32])),
                    udp_ports: vec![],
                    tcp_ports: vec![],
                    tls_ports: vec![],
                    priority: 0,
                    exit_label: None,
                    server_name: None,
                },
                Arc::new(MemoryCache::default()),
            )
            .unwrap();
            let exchanges = async {
                let query = async {
                    let socket = private.udp(dns_address).unwrap();
                    socket.send(b"selected private DNS").await.unwrap();
                    let mut bytes = [0; 64];
                    let n = socket.recv(&mut bytes).await.unwrap();
                    assert_eq!(&bytes[..n], b"selected private DNS");
                };
                let response = async {
                    let mut bytes = [0; 64];
                    let (n, from) = dns.recv_from(&mut bytes).await.unwrap();
                    assert_eq!(from.ip(), addresses.v4(2));
                    dns.sendto(&bytes[..n], from.into()).await.unwrap();
                };
                let http = async {
                    let deadline = Instant::now() + Duration::from_secs(3);
                    // The service starts on its own thread; an early RST may precede
                    // listener creation. Retry startup without assuming a sleep is readiness.
                    let mut stream = loop {
                        let pending = customer.tcp_connect(addresses.http()).unwrap();
                        match proton_vpn_netstack::socket::AsyncTcpConnect::Pending(pending).await {
                            Ok(stream) => break stream,
                            Err(_) => {
                                assert!(Instant::now() < deadline, "local HTTP listener startup");
                                tokio::time::sleep(Duration::from_millis(1)).await;
                            }
                        }
                    };
                    stream
                        .write_all(b"GET / HTTP/1.1\r\nHost: local\r\n\r\n")
                        .await
                        .unwrap();
                    stream.flush().await.unwrap();
                    let mut reply = Vec::new();
                    stream.read_to_end(&mut reply).await.unwrap();
                    assert!(reply.starts_with(b"HTTP/1.1 200 OK\r\n"));
                    assert!(reply.ends_with(b"Hello World"));
                };
                futures::join!(query, response, http);
            };
            let pump = async {
                let start = Instant::now();
                let mut bytes = [0; 65535];
                loop {
                    let now = start.elapsed();
                    let micros = now.as_micros() as i64;
                    for _ in 0..64 {
                        match app_wire.read_packet(&mut bytes, micros) {
                            Ok(n) => assert!(router.receive_local(&bytes[..n], now).unwrap()),
                            Err(DeviceReadError::WouldBlock) => break,
                            Err(error) => panic!("customer: {error:?}"),
                        }
                    }
                    for _ in 0..64 {
                        let Some(n) = router.read_packet(&mut bytes, now).unwrap() else {
                            break;
                        };
                        if protun_next::api::internal::local_service_source_with(
                            &bytes[..n],
                            addresses,
                        ) {
                            app_wire.write_packet(&bytes[..n], micros).unwrap();
                        } else {
                            assert_eq!(&bytes[12..16], &addresses.v4(2).octets());
                            peer_wire.write_packet(&bytes[..n], micros).unwrap();
                        }
                    }
                    for _ in 0..64 {
                        match peer_wire.read_packet(&mut bytes, micros) {
                            Ok(n) => assert!(router.receive(&mut bytes[..n], now).unwrap()),
                            Err(DeviceReadError::WouldBlock) => break,
                            Err(error) => panic!("peer: {error:?}"),
                        }
                    }
                    assert!(
                        now < Duration::from_secs(4),
                        "alternate subnet exchange timed out"
                    );
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            };
            tokio::select! { _ = exchanges => {}, _ = pump => unreachable!() }
            drop(service);
        });
}

#[test]
fn peer_initiated_reserved_port_is_private_and_its_reply_retains_the_wire_tuple() {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let (private, mut router) =
                InternalNetwork::new(Duration::ZERO, CountWake(Arc::new(AtomicUsize::new(0))))
                    .unwrap();
            let (peer, mut peer_wire) = stack("10.2.0.1");
            let listener = private
                .factory()
                .udp_bind("10.2.0.2:1000".parse::<std::net::SocketAddr>().unwrap())
                .unwrap();
            let client = peer
                .udp_bind("10.2.0.1:7777".parse::<std::net::SocketAddr>().unwrap())
                .unwrap()
                .connect("10.2.0.2:1000".parse::<std::net::SocketAddr>().unwrap());
            let exchanges = async {
                let send = async {
                    client.send(b"peer initiated").await.unwrap();
                    let mut bytes = [0; 32];
                    let n = client.recv(&mut bytes).await.unwrap();
                    assert_eq!(&bytes[..n], b"peer initiated");
                };
                let echo = async {
                    let mut bytes = [0; 32];
                    let (n, from) = listener.recv_from(&mut bytes).await.unwrap();
                    assert_eq!(
                        from,
                        "10.2.0.1:7777".parse::<std::net::SocketAddr>().unwrap()
                    );
                    listener.sendto(&bytes[..n], from.into()).await.unwrap();
                };
                futures::join!(send, echo);
            };
            let pump = async {
                let start = Instant::now();
                let mut bytes = [0; 65535];
                loop {
                    let now = start.elapsed();
                    let micros = now.as_micros() as i64;
                    for _ in 0..64 {
                        match peer_wire.read_packet(&mut bytes, micros) {
                            Ok(n) => assert_eq!(
                                router.receive_routed(&mut bytes[..n], now).unwrap(),
                                protun_next::api::internal::PacketDisposition::Private
                            ),
                            Err(DeviceReadError::WouldBlock) => break,
                            Err(error) => panic!("peer: {error:?}"),
                        }
                    }
                    for _ in 0..64 {
                        let Some(n) = router.read_packet(&mut bytes, now).unwrap() else {
                            break;
                        };
                        assert_eq!(&bytes[20..24], &[0x03, 0xe8, 0x1e, 0x61]); // 1000 -> 7777
                        peer_wire.write_packet(&bytes[..n], micros).unwrap();
                    }
                    assert!(
                        now < Duration::from_secs(3),
                        "private inbound flow timed out"
                    );
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            };
            tokio::select! { _ = exchanges => {}, _ = pump => unreachable!() }
        });
}

fn ipv4_fragments(packet: &[u8], payload_size: usize) -> Vec<Vec<u8>> {
    assert_eq!(packet[0], 0x45);
    assert_eq!(payload_size % 8, 0);
    let payload = &packet[20..];
    payload
        .chunks(payload_size)
        .enumerate()
        .map(|(index, chunk)| {
            let mut fragment = packet[..20].to_vec();
            fragment.extend_from_slice(chunk);
            let length = fragment.len() as u16;
            fragment[2..4].copy_from_slice(&length.to_be_bytes());
            let offset = index * payload_size;
            let flags = (offset / 8) as u16
                | if offset + chunk.len() < payload.len() {
                    0x2000
                } else {
                    0
                };
            fragment[6..8].copy_from_slice(&flags.to_be_bytes());
            fragment[10..12].fill(0);
            let mut checksum: u32 = fragment[..20]
                .as_chunks::<2>()
                .0
                .iter()
                .map(|word| u32::from(u16::from_be_bytes([word[0], word[1]])))
                .sum();
            while checksum > 0xffff {
                checksum = (checksum & 0xffff) + (checksum >> 16);
            }
            fragment[10..12].copy_from_slice(&(!(checksum as u16)).to_be_bytes());
            fragment
        })
        .collect()
}
