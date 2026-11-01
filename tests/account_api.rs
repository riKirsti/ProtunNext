#![cfg(feature = "local-agent")]
//! Public test tokens only. A custom HTTP fixture runs entirely inside two
//! userspace IP stacks; production hosts, credentials and OS sockets are unused.
use futures::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use proton_boringtun::proton_os_interface::{lock::strict::Strict, rand::SeedableRng};
use proton_vpn_netstack::{
    NetworkConfig, NetworkConfigV4, NetworkInterface, NetworkStack, error::DeviceReadError,
    socket::TcpListener,
};
use protun_next::{
    api::{
        account::{Command, Notice, Service},
        cache::MemoryCache,
        internal::InternalNetwork,
        types::*,
    },
    session::Session,
};
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
fn fork() -> zeroize::Zeroizing<ForkSelectorInfo> {
    zeroize::Zeroizing::new(ForkSelectorInfo {
        selector: "PUBLIC-TEST-SELECTOR".into(),
        cookies: Cookies {
            items: vec![Cookie {
                name: "fixture".into(),
                value: "PUBLIC-TEST-COOKIE".into(),
            }],
        },
    })
}
async fn server_requests(mut listener: TcpListener, requests: &AtomicUsize) {
    for stage in 0..2 {
        let (stream, remote) = listener.accept().await.unwrap();
        requests.fetch_add(1, Ordering::Relaxed);
        assert!(remote.port() < 1024);
        let mut stream = futures::io::BufReader::new(stream);
        let mut head = String::new();
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            assert!(stream.read_line(&mut line).await.unwrap() > 0);
            head.push_str(&line);
            assert!(head.len() < 16384);
            if let Some((name, value)) = line.split_once(':')
                && name.eq_ignore_ascii_case("content-length")
            {
                content_length = value.trim().parse::<usize>().unwrap();
            }
            if line == "\r\n" {
                break;
            }
        }
        if stage == 0 {
            assert!(
                head.starts_with("POST /vpn/v1/certificate HTTP/1.1\r\n"),
                "unexpected request: {head}"
            );
            assert!(
                head.to_ascii_lowercase()
                    .contains("authorization: bearer public-test-access")
            );
            assert!(content_length > 0 && content_length < 8192);
            let mut body = vec![0; content_length];
            stream.read_exact(&mut body).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(body["Duration"], "1 hours");
            assert_eq!(body["Renew"], true);
            assert_eq!(body["Features"]["NetShieldLevel"], 3);
            assert_eq!(body["Features"]["PortForwarding"], true);
            assert_eq!(body["Features"]["RandomNAT"], false);
            assert_eq!(body["Features"]["SplitTCP"], true);
            assert!(
                body["ClientPublicKey"]
                    .as_str()
                    .unwrap()
                    .starts_with("-----BEGIN PUBLIC KEY-----")
            );
        } else {
            assert!(
                head.starts_with("GET /auth/v4/sessions/forks/PUBLIC-TEST-SELECTOR HTTP/1.1\r\n"),
                "fork did not reach its published endpoint: {head}"
            );
            assert!(head.contains("fixture=PUBLIC-TEST-COOKIE"));
            assert!(
                !head.contains("PUBLIC-TEST-ACCESS"),
                "rejected in-memory session survived replacement"
            );
        }
        let body = br#"{"Code":10013,"Error":"PUBLIC TEST expired refresh token"}"#;
        let reply = format!(
            "HTTP/1.1 422 Unprocessable Entity\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.get_mut().write_all(reply.as_bytes()).await.unwrap();
        stream.get_mut().write_all(body).await.unwrap();
        // The SDK may close immediately after consuming Content-Length bytes.
        // Event assertions, not a final TCP ACK, prove reply receipt.
        let _ = stream.get_mut().flush().await;
    }
    assert!(
        tokio::time::timeout(Duration::from_millis(250), listener.accept())
            .await
            .is_err(),
        "consumed selector was replayed or the auth loop spun"
    );
}
async fn account_events(service: &Service, cache: &MemoryCache) {
    for stage in 0..2 {
        let (mut error_seen, mut fork_needed) = (false, false);
        while !error_seen || !fork_needed {
            while let Some(notice) = service.try_recv() {
                match notice {
                    Notice::Event(Event::LocalAgentError {
                        error:
                            ErrorEvent::ApiError {
                                http_code: Some(422),
                                proton_code: Some(10013),
                                refresh_token_invalid: true,
                                ..
                            },
                    }) => error_seen = true,
                    Notice::Event(Event::LocalAgentError {
                        error: ErrorEvent::ForkSelectorNeeded,
                    }) => fork_needed = true,
                    Notice::Fatal => {
                        panic!("account runtime failed before handling fixture response")
                    }
                    _ => {}
                }
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        assert!(
            cache.get(CacheKey::ApiSession).is_none(),
            "rejected session must not survive restart"
        );
        service.send(Command::Fork(fork())); // second send deliberately repeats the consumed selector
        if stage == 1 {
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    }
}
async fn pump(router: &mut InternalNetwork, wire: &mut NetworkInterface) {
    let start = Instant::now();
    let mut packet = [0; 65535];
    loop {
        let now = start.elapsed();
        let micros = now.as_micros() as i64;
        for _ in 0..64 {
            let Some(n) = router.read_packet(&mut packet, now).unwrap() else {
                break;
            };
            wire.write_packet(&packet[..n], micros).unwrap();
        }
        for _ in 0..64 {
            match wire.read_packet(&mut packet, micros) {
                Ok(n) => assert!(router.receive(&mut packet[..n], now).unwrap()),
                Err(DeviceReadError::WouldBlock) => break,
                Err(e) => panic!("fixture peer: {e:?}"),
            }
        }
        assert!(
            now < Duration::from_secs(5),
            "private account request/response timed out"
        );
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}
#[test]
fn session_rejection_fork_replacement_cookies_and_duplicate_suppression_use_private_network() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    runtime.block_on(async {
        let wakes = Arc::new(AtomicUsize::new(0));
        let (network, mut router) =
            InternalNetwork::new(Duration::ZERO, CountWake(wakes.clone())).unwrap();
        let stack = NetworkStack::builder()
            .with_time(0)
            .with_config(NetworkConfig::V4(NetworkConfigV4::new(
                "10.2.0.1".parse().unwrap(),
                24,
                None,
            )))
            .with_rng(ChaCha20Rng::from_seed([55; 32]))
            .with_mtu(1420)
            .with_tcp_rx_buffer_size(65536)
            .with_tcp_tx_buffer_size(65536)
            .build::<Strict>();
        let (server, mut wire) = stack.into_sides();
        let listener = server
            .tcp_bind("10.2.0.1:8080".parse::<std::net::SocketAddr>().unwrap())
            .unwrap();
        let cache = Arc::new(MemoryCache::default());
        let auth = muon::auth::Auth::internal(
            "PUBLIC-TEST-USER",
            "PUBLIC-TEST-UID",
            muon::auth::Tokens::access(
                "PUBLIC-TEST-ACCESS",
                "PUBLIC-TEST-REFRESH",
                muon::auth::Scopes::new(["vpn".into()]),
            ),
        );
        cache.put(CacheKey::ApiSession, serde_json::to_vec(&auth).unwrap());
        let mode = ConnectionMode::LocalAgent {
            user_agent: "ProtunNext offline fixture".into(),
            app_version: "windows-vpn@4.1.0".into(),
            settings: LocalAgentSettings {
                netshield_level: Some(NetshieldLevel::AdultAndAdsAndMalwareFilter),
                split_tcp: Some(true),
                port_forwarding: Some(true),
                random_nat: Some(false),
                ..Default::default()
            },
            muon_env: MuonEnv::CustomServers {
                servers: vec!["http://10.2.0.1:8080/".into()],
                api_pins: None,
                ar_pins: None,
            },
            certificate_duration: Some(CertificateDuration::Hours { count: 1 }),
        };
        let peer = PeerInfo {
            peer_id: "offline".into(),
            server_ip: IpAddress("10.2.0.1".parse().unwrap()),
            server_public_key: WgPeerPublicKey(Session::public_key([9; 32])),
            udp_ports: vec![],
            tcp_ports: vec![],
            tls_ports: vec![],
            priority: 0,
            exit_label: None,
            server_name: None,
        };
        let service = Service::start(network, mode, peer, cache.clone()).unwrap();
        let requests = AtomicUsize::new(0);
        let exchange = async {
            futures::join!(
                server_requests(listener, &requests),
                account_events(&service, &cache)
            );
        };
        tokio::select! { _ = exchange => {}, _ = pump(&mut router, &mut wire) => unreachable!() }
        drop(service);
        assert_eq!(requests.load(Ordering::Relaxed), 2);
        assert!(wakes.load(Ordering::Relaxed) > 0);
    });
}

#[test]
fn issued_certificate_is_persisted_and_explicit_invalidation_requests_a_replacement() {
    use base64::{Engine, engine::general_purpose::STANDARD};
    use std::time::{SystemTime, UNIX_EPOCH};
    let pem = |label: &str, der: &[u8]| {
        format!(
            "-----BEGIN {label}-----\n{}\n-----END {label}-----\n",
            STANDARD.encode(der)
        )
    };
    let private = pem(
        "PRIVATE KEY",
        include_bytes!("fixtures/ed25519-private-PUBLIC-TEST-ONLY.der"),
    );
    let certificate_der = include_bytes!("fixtures/ed25519-client-TEST-ONLY.der");
    let certificate = pem("CERTIFICATE", certificate_der);
    let expiry = x509_parser::parse_x509_certificate(certificate_der)
        .unwrap()
        .1
        .validity()
        .not_after
        .timestamp() as u64;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    runtime.block_on(async {
        let (network, mut router) = InternalNetwork::new(Duration::ZERO,
            CountWake(Arc::new(AtomicUsize::new(0)))).unwrap();
        let stack = NetworkStack::builder().with_time(0)
            .with_config(NetworkConfig::V4(NetworkConfigV4::new("10.2.0.1".parse().unwrap(), 24, None)))
            .with_rng(ChaCha20Rng::from_seed([57; 32])).with_mtu(1420)
            .with_tcp_rx_buffer_size(65536).with_tcp_tx_buffer_size(65536).build::<Strict>();
        let (server, mut wire) = stack.into_sides();
        let mut listener = server.tcp_bind("10.2.0.1:8080".parse::<std::net::SocketAddr>().unwrap()).unwrap();
        let cache = Arc::new(MemoryCache::default());
        cache.put(CacheKey::PrivateKey, private.as_bytes().to_vec());
        let public_key = protun_next::api::keys::Identity::load_or_generate(cache.as_ref()).unwrap().public_pem().unwrap();
        let auth = muon::auth::Auth::internal("PUBLIC-TEST-USER", "PUBLIC-TEST-UID",
            muon::auth::Tokens::access("PUBLIC-TEST-ACCESS", "PUBLIC-TEST-REFRESH", muon::auth::Scopes::new(["vpn".into()])));
        cache.put(CacheKey::ApiSession, serde_json::to_vec(&auth).unwrap());
        let mode = ConnectionMode::LocalAgent {
            user_agent: "ProtunNext offline certificate fixture".into(), app_version: "windows-vpn@4.1.0".into(),
            settings: Default::default(), muon_env: MuonEnv::CustomServers {
                servers: vec!["http://10.2.0.1:8080/".into()], api_pins: None, ar_pins: None,
            }, certificate_duration: Some(CertificateDuration::Minutes { count: 90 }),
        };
        let peer = PeerInfo {
            peer_id: "offline".into(), server_ip: IpAddress("10.2.0.1".parse().unwrap()),
            server_public_key: WgPeerPublicKey(Session::public_key([9; 32])), udp_ports: vec![], tcp_ports: vec![],
            tls_ports: vec![], priority: 0, exit_label: None, server_name: None,
        };
        let service = Service::start(network, mode, peer, cache.clone()).unwrap();
        let requests = AtomicUsize::new(0);
        let issued = async {
            for _ in 0..2 {
                let (stream, _) = listener.accept().await.unwrap();
                requests.fetch_add(1, Ordering::Relaxed);
                let mut stream = futures::io::BufReader::new(stream);
                let mut head = String::new();
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    assert!(stream.read_line(&mut line).await.unwrap() > 0);
                    head.push_str(&line);
                    assert!(head.len() < 16384);
                    if let Some((name, value)) = line.split_once(':') && name.eq_ignore_ascii_case("content-length") {
                        length = value.trim().parse::<usize>().unwrap();
                    }
                    if line == "\r\n" { break; }
                }
                assert!(head.starts_with("POST /vpn/v1/certificate HTTP/1.1\r\n"));
                assert!(length > 0 && length < 8192);
                let mut body = vec![0; length];
                stream.read_exact(&mut body).await.unwrap();
                let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
                assert_eq!(body["ClientPublicKey"], public_key);
                assert_eq!(body["Duration"], "90 minutes");
                let now = SystemTime::now();
                let seconds = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert!(expiry > seconds);
                let response = serde_json::to_vec(&serde_json::json!({
                    "Code": 1000, "Certificate": certificate, "ExpirationTime": expiry, "RefreshTime": seconds + 3600,
                })).unwrap();
                let head = format!("HTTP/1.1 200 OK\r\nDate: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    httpdate::fmt_http_date(now), response.len());
                stream.get_mut().write_all(head.as_bytes()).await.unwrap();
                stream.get_mut().write_all(&response).await.unwrap();
                let _ = stream.get_mut().flush().await;
            }
        };
        let observe = async {
            for stage in 0..2 {
                loop {
                    while let Some(notice) = service.try_recv() {
                        assert!(!matches!(notice, Notice::Fatal | Notice::Rekey), "issued-certificate lifecycle failed");
                    }
                    if requests.load(Ordering::Relaxed) > stage && let Some(bytes) = cache.get(CacheKey::Certificate) {
                        let text = String::from_utf8(bytes).unwrap();
                        assert!(text.starts_with("Issue-Date: "));
                        assert!(text.contains("\nRefresh-Date: "));
                        assert!(text.ends_with(&certificate));
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
                if stage == 0 { service.send(Command::Invalidate); }
            }
        };
        let exchange = async { futures::join!(issued, observe); };
        tokio::select! { _ = exchange => {}, _ = pump(&mut router, &mut wire) => unreachable!() }
        assert_eq!(requests.load(Ordering::Relaxed), 2);
        drop(service);
        // This exercises the published certificate API state machine, not a
        // successful LocalAgent mTLS/server-policy session or Proton service.
    });
}
