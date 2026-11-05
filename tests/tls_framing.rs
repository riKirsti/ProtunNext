//! Real TLS sockets with certificate validation; not a Proton Stealth claim.
use protun_next::{
    packet::PacketPool,
    stream::{FramedIo, Receive},
    tls,
};
use rustls::{
    RootCertStore, ServerConfig, ServerConnection, StreamOwned,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};
fn roots() -> RootCertStore {
    let mut r = RootCertStore::empty();
    r.add(CertificateDer::from(
        include_bytes!("fixtures/root-TEST-ONLY.der").to_vec(),
    ))
    .unwrap();
    r
}
fn server() -> Arc<ServerConfig> {
    server_version(rustls::DEFAULT_VERSIONS)
}
fn server_version(versions: &[&'static rustls::SupportedProtocolVersion]) -> Arc<ServerConfig> {
    let cert = CertificateDer::from(include_bytes!("fixtures/server-TEST-ONLY.der").to_vec());
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
        include_bytes!("fixtures/private-PUBLIC-TEST-ONLY.der").to_vec(),
    ));
    Arc::new(
        ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
            .with_protocol_versions(versions)
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .unwrap(),
    )
}

fn unbuffered_client_with_roots(
    name: &str,
    roots: RootCertStore,
) -> protun_next::Result<rustls::client::UnbufferedClientConnection> {
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .unwrap()
    .with_root_certificates(roots)
    .with_no_client_auth();
    rustls::client::UnbufferedClientConnection::new(
        Arc::new(config),
        name.to_owned().try_into().unwrap(),
    )
    .map_err(|e| protun_next::Error::Protocol(e.to_string()))
}

#[test]
fn unbuffered_tls12_and_tls13_preserve_bytes_with_short_writes_and_small_reads() {
    for version in [&rustls::version::TLS12, &rustls::version::TLS13] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (tcp, _) = listener.accept().unwrap();
            let mut stream = StreamOwned::new(
                ServerConnection::new(server_version(&[version])).unwrap(),
                bound_socket(tcp),
            );
            let mut body = vec![0; 80_000];
            stream.read_exact(&mut body).unwrap();
            assert!(body.iter().enumerate().all(|(n, b)| *b == (n % 251) as u8));
            if version.version == rustls::ProtocolVersion::TLSv1_3 {
                stream.conn.refresh_traffic_keys().unwrap();
            }
            stream.write_all(&body).unwrap();
            stream.conn.send_close_notify();
            stream.flush().unwrap();
            let mut after_close = [0];
            stream.read_exact(&mut after_close).unwrap();
            assert_eq!(after_close, [b'X']);
        });
        let tcp = bound_socket(TcpStream::connect(address).unwrap());
        tcp.set_nonblocking(true).unwrap();
        let mut stream = protun_next::tls_unbuffered::UnbufferedTlsIo::new(
            Pressured(tcp, false),
            unbuffered_client_with_roots("localhost", roots()).unwrap(),
        );
        let body: Vec<u8> = (0..80_000).map(|n| (n % 251) as u8).collect();
        let mut accepted = 0;
        let mut received = Vec::new();
        let mut requested_update = false;
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while received.len() < body.len() {
            assert!(
                std::time::Instant::now() < deadline,
                "unbuffered TLS stalled: accepted {accepted}, received {}",
                received.len()
            );
            if version.version == rustls::ProtocolVersion::TLSv1_3
                && accepted > 20_000
                && !requested_update
            {
                stream.refresh_traffic_keys().unwrap();
                requested_update = true;
            }
            if accepted < body.len() {
                let end = (accepted + 24_000).min(body.len());
                let middle = (accepted + 900).min(end);
                match stream.write_vectored(&[
                    std::io::IoSlice::new(&body[accepted..middle]),
                    std::io::IoSlice::new(&body[middle..end]),
                ]) {
                    Ok(0) => panic!("zero write"),
                    Ok(n) => accepted += n,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                    Err(e) => panic!("write: {e}"),
                }
            }
            match stream.flush() {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => panic!("flush: {e}"),
            }
            let mut output = [0; 1013];
            match stream.read(&mut output) {
                Ok(0) => panic!("early EOF"),
                Ok(n) => received.extend_from_slice(&output[..n]),
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
                    ) => {}
                Err(e) => panic!("read: {e}"),
            }
            thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(received, body);
        assert_eq!(accepted, body.len());
        if version.version == rustls::ProtocolVersion::TLSv1_2 {
            assert!(stream.refresh_traffic_keys().is_err());
        } else {
            assert!(requested_update);
        }
        let mut byte = [0];
        loop {
            match stream.read(&mut byte) {
                Ok(0) => break,
                Ok(_) => panic!("unexpected trailing plaintext"),
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
                    ) => {}
                Err(e) => panic!("close_notify: {e}"),
            }
            assert!(std::time::Instant::now() < deadline);
            thread::sleep(Duration::from_millis(1));
        }
        // Receiving close_notify closes only the peer's write direction.
        loop {
            match stream.write(b"X") {
                Ok(1) => break,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                result => panic!("half-close write: {result:?}"),
            }
            assert!(std::time::Instant::now() < deadline);
        }
        loop {
            match stream.queue_close_notify() {
                Ok(()) => break,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                result => panic!("client close_notify: {result:?}"),
            }
            assert!(std::time::Instant::now() < deadline);
        }
        while let Err(e) = stream.flush() {
            assert_eq!(e.kind(), std::io::ErrorKind::WouldBlock);
            assert!(std::time::Instant::now() < deadline);
        }
        handle.join().unwrap();
    }
}
fn bound_socket(tcp: TcpStream) -> TcpStream {
    tcp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    tcp.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
    tcp.set_nodelay(true).unwrap();
    tcp
}

struct CountWrites {
    socket: TcpStream,
    writes: usize,
}
impl Read for CountWrites {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        self.socket.read(bytes)
    }
}
impl Write for CountWrites {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.writes += 1;
        self.socket.write(bytes)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.socket.flush()
    }
}

#[test]
fn unbuffered_tls_batches_available_records_without_waiting_for_more_plaintext() {
    for version in [&rustls::version::TLS12, &rustls::version::TLS13] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (ready, wait_ready) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let (tcp, _) = listener.accept().unwrap();
            let mut peer = StreamOwned::new(
                ServerConnection::new(server_version(&[version])).unwrap(),
                bound_socket(tcp),
            );
            let mut hello = [0; 5];
            peer.read_exact(&mut hello).unwrap();
            assert_eq!(&hello, b"start");
            ready.send(()).unwrap();
            let mut body = vec![0; 16 * 16384];
            peer.read_exact(&mut body).unwrap();
            assert!(body.iter().enumerate().all(|(n, b)| *b == (n % 251) as u8));
            peer.write_all(b"OK").unwrap();
            peer.flush().unwrap();
            let mut sparse = [0; 1];
            peer.read_exact(&mut sparse).unwrap();
            assert_eq!(&sparse, b"X");
        });
        let mut client = protun_next::tls_unbuffered::UnbufferedTlsIo::new(
            CountWrites {
                socket: bound_socket(TcpStream::connect(address).unwrap()),
                writes: 0,
            },
            unbuffered_client_with_roots("localhost", roots()).unwrap(),
        );
        client.write_all(b"start").unwrap();
        client.flush().unwrap();
        wait_ready.recv_timeout(Duration::from_secs(3)).unwrap();
        client.socket_mut().writes = 0;
        let body: Vec<u8> = (0..16 * 16384).map(|n| (n % 251) as u8).collect();
        for chunk in body.chunks(16384) {
            client.write_all(chunk).unwrap();
        }
        client.flush().unwrap();
        let calls = client.socket().writes;
        let mut ack = [0; 2];
        client.read_exact(&mut ack).unwrap();
        assert_eq!(&ack, b"OK");
        assert_eq!(
            client
                .write_vectored(&[std::io::IoSlice::new(b"X")])
                .unwrap(),
            1
        );
        client.flush().unwrap();
        handle.join().unwrap();
        eprintln!(
            "TLS {:?}: 16 queued records used {calls} socket writes",
            version.version
        );
        assert!(
            calls <= 8,
            "available TLS records should share socket writes, got {calls}"
        );
    }
}

#[test]
fn unbuffered_tls_drains_full_ack_queue_while_download_records_remain_unread() {
    download_ack_loop(
        |tcp| {
            protun_next::tls_unbuffered::UnbufferedTlsIo::new(
                tcp,
                unbuffered_client_with_roots("localhost", roots()).unwrap(),
            )
        },
        |io| io.wants_write(),
    );
}
#[test]
fn selected_tls_drains_full_ack_queue_while_download_records_remain_unread() {
    download_ack_loop(
        |tcp| tls::TlsIo::new(tcp, tls::client_with_roots("localhost", roots()).unwrap()),
        |io| io.wants_write(),
    );
}
fn download_ack_loop<S: Read + Write>(
    make: impl FnOnce(TcpStream) -> S,
    wants_write: impl Fn(&S) -> bool,
) {
    const PACKETS: usize = 4096;
    const BURST: usize = 256;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let handle = thread::spawn(move || {
        let (tcp, _) = listener.accept().unwrap();
        let mut peer = FramedIo::new(
            StreamOwned::new(ServerConnection::new(server()).unwrap(), bound_socket(tcp)),
            64,
        )
        .unwrap();
        let mut hello = [0; 5];
        peer.io_mut().read_exact(&mut hello).unwrap();
        assert_eq!(&hello, b"start");
        let mut pool = PacketPool::new(2, 2048).unwrap();
        let mut encoder = protun_next::framing::Encoder::default();
        let mut wire = Vec::with_capacity(BURST * 1300);
        for base in (0..PACKETS).step_by(BURST) {
            wire.clear();
            for sequence in base..base + BURST {
                let mut packet = pool.take().unwrap();
                packet.writable()[..1280].fill(0x37);
                packet.writable()[..8].copy_from_slice(&[4, 0, 0, 0, 0, 0, 0, 0]);
                packet.writable()[8..16].copy_from_slice(&(sequence as u64).to_le_bytes());
                packet.set_len(1280).unwrap();
                encoder.frame(&mut packet).unwrap();
                wire.extend_from_slice(packet.data());
                pool.recycle(packet).unwrap();
            }
            // Several TLS records are available while the client queues its ACKs.
            peer.io_mut().write_all(&wire).unwrap();
            peer.io_mut().flush().unwrap();
            for sequence in base..base + BURST {
                let mut ack = pool.take().unwrap();
                assert_eq!(peer.receive(&mut ack).unwrap(), Receive::Packet);
                assert_eq!(ack.data().len(), 32);
                assert_eq!(&ack.data()[8..16], &(sequence as u64).to_le_bytes());
                pool.recycle(ack).unwrap();
            }
        }
    });
    let tcp = bound_socket(TcpStream::connect(address).unwrap());
    let control = tcp.try_clone().unwrap();
    let mut tls = make(tcp);
    tls.write_all(b"start").unwrap();
    tls.flush().unwrap();
    control.set_nonblocking(true).unwrap();
    let mut stream = FramedIo::new(tls, 64).unwrap();
    let mut pool = PacketPool::new(132, 2048).unwrap();
    let mut received = 0;
    let deadline = std::time::Instant::now() + Duration::from_secs(4);
    while received < PACKETS || stream.pending() != 0 || wants_write(stream.io()) {
        assert!(
            std::time::Instant::now() < deadline,
            "download/ACK deadlock: {received} packets received, {} ACKs queued",
            stream.pending()
        );
        // Same reserved-output budget used by the packet worker. Filling it must
        // not make draining depend on consuming another incoming TLS record.
        while received < PACKETS && stream.pending() < 63 {
            let mut packet = pool.take().unwrap();
            match stream.receive(&mut packet).unwrap() {
                Receive::Packet => {
                    assert_eq!(packet.data().len(), 1280);
                    assert_eq!(&packet.data()[8..16], &(received as u64).to_le_bytes());
                    assert!(packet.data()[16..].iter().all(|b| *b == 0x37));
                    pool.recycle(packet).unwrap();
                    let mut ack = pool.take().unwrap();
                    ack.writable()[..32].fill(0);
                    ack.writable()[0] = 4;
                    ack.writable()[8..16].copy_from_slice(&(received as u64).to_le_bytes());
                    ack.set_len(32).unwrap();
                    assert!(stream.queue(ack).is_ok());
                    received += 1;
                }
                Receive::WouldBlock | Receive::Yield => {
                    pool.recycle(packet).unwrap();
                    break;
                }
                Receive::Eof => panic!("download ended early"),
            }
        }
        for _ in 0..64 {
            if stream.flush(&mut pool).unwrap() != protun_next::stream::Flush::Progress {
                break;
            }
        }
        match stream.io_mut().flush() {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => panic!("TLS flush: {e}"),
        }
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

#[test]
fn framed_bytes_over_authenticated_tls() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let handle = thread::spawn(move || {
        let (tcp, _) = listener.accept().unwrap();
        let tls = StreamOwned::new(ServerConnection::new(server()).unwrap(), bound_socket(tcp));
        let mut stream = FramedIo::new(tls, 2).unwrap();
        let mut pool = PacketPool::new(2, 2048).unwrap();
        let mut packet = pool.take().unwrap();
        assert_eq!(stream.receive(&mut packet).unwrap(), Receive::Packet);
        assert_eq!(packet.data().len(), 1280);
        assert_eq!(&packet.data()[..4], &[4, 0, 0, 0]);
        assert!(stream.queue(packet).is_ok());
        while stream.pending() > 0 {
            stream.flush(&mut pool).unwrap();
        }
    });
    let tcp = bound_socket(TcpStream::connect(address).unwrap());
    let tls = tls::TlsIo::new(tcp, tls::client_with_roots("localhost", roots()).unwrap());
    let mut stream = FramedIo::new(tls, 2).unwrap();
    let mut pool = PacketPool::new(2, 2048).unwrap();
    let mut packet = pool.take().unwrap();
    packet.writable()[..1280].fill(0x5a);
    packet.writable()[..4].copy_from_slice(&[4, 0, 0, 0]);
    packet.set_len(1280).unwrap();
    assert!(stream.queue(packet).is_ok());
    while stream.pending() > 0 {
        stream.flush(&mut pool).unwrap();
    }
    let mut reply = pool.take().unwrap();
    assert_eq!(stream.receive(&mut reply).unwrap(), Receive::Packet);
    assert_eq!(reply.data().len(), 1280);
    assert_eq!(&reply.data()[..4], &[4, 0, 0, 0]);
    assert!(reply.data()[4..].iter().all(|b| *b == 0x5a));
    handle.join().unwrap();
}

#[test]
fn unbuffered_tls_writes_preserve_a_partially_read_plaintext_record() {
    for version in [&rustls::version::TLS12, &rustls::version::TLS13] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (tcp, _) = listener.accept().unwrap();
            let mut peer = StreamOwned::new(
                ServerConnection::new(server_version(&[version])).unwrap(),
                bound_socket(tcp),
            );
            let mut start = [0; 5];
            peer.read_exact(&mut start).unwrap();
            assert_eq!(&start, b"start");
            let body: Vec<u8> = (0..1024).map(|n| (n % 251) as u8).collect();
            peer.write_all(&body).unwrap();
            peer.flush().unwrap();
            let mut ack = [0; 3];
            peer.read_exact(&mut ack).unwrap();
            assert_eq!(&ack, b"ack");
        });
        let mut client = protun_next::tls_unbuffered::UnbufferedTlsIo::new(
            bound_socket(TcpStream::connect(address).unwrap()),
            unbuffered_client_with_roots("localhost", roots()).unwrap(),
        );
        client.write_all(b"start").unwrap();
        client.flush().unwrap();
        let mut body = [0; 1024];
        client.read_exact(&mut body[..1]).unwrap();
        // The remaining 1023 plaintext bytes must neither block this ACK nor
        // disappear when the writer temporarily borrows the pending buffer.
        client.write_all(b"ack").unwrap();
        client.flush().unwrap();
        client.read_exact(&mut body[1..]).unwrap();
        assert!(body.iter().enumerate().all(|(n, b)| *b == (n % 251) as u8));
        handle.join().unwrap();
    }
}
#[test]
fn wrong_server_name_is_rejected() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let handle = thread::spawn(move || {
        let (tcp, _) = listener.accept().unwrap();
        let mut stream =
            StreamOwned::new(ServerConnection::new(server()).unwrap(), bound_socket(tcp));
        let mut b = [0; 1];
        assert!(stream.read(&mut b).is_err());
    });
    let tcp = bound_socket(TcpStream::connect(address).unwrap());
    let mut stream = tls::TlsIo::new(
        tcp,
        tls::client_with_roots("wrong.invalid", roots()).unwrap(),
    );
    assert!(
        stream
            .write_all(b"must not be accepted")
            .and_then(|_| stream.read(&mut [0; 1]))
            .is_err()
    );
    drop(stream);
    handle.join().unwrap();
}
#[test]
fn unbuffered_rejects_wrong_names_and_untrusted_certificates() {
    for trusted in [true, false] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (tcp, _) = listener.accept().unwrap();
            let mut server =
                StreamOwned::new(ServerConnection::new(server()).unwrap(), bound_socket(tcp));
            assert!(
                server.read(&mut [0; 1]).is_err(),
                "unverified plaintext must not reach peer"
            );
        });
        let tcp = bound_socket(TcpStream::connect(address).unwrap());
        let mut client = protun_next::tls_unbuffered::UnbufferedTlsIo::new(
            tcp,
            unbuffered_client_with_roots(
                if trusted {
                    "wrong.invalid"
                } else {
                    "localhost"
                },
                if trusted {
                    roots()
                } else {
                    RootCertStore::empty()
                },
            )
            .unwrap(),
        );
        assert!(client.write_all(b"must not be sent").is_err());
        drop(client);
        handle.join().unwrap();
    }
}

// Real TCP/TLS with forced short writes and intermittent backpressure. This
// tests accepted-byte ownership across rustls's bounded plaintext/TLS buffers.
struct Pressured(TcpStream, bool);
impl Read for Pressured {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(output)
    }
}
impl Write for Pressured {
    fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
        self.1 = !self.1;
        if self.1 {
            return Err(std::io::ErrorKind::WouldBlock.into());
        }
        self.0.write(&input[..input.len().min(257)])
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct WriteGate {
    socket: TcpStream,
    blocked: Arc<std::sync::atomic::AtomicBool>,
}
impl Read for WriteGate {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        self.socket.read(output)
    }
}
impl Write for WriteGate {
    fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
        if self.blocked.load(std::sync::atomic::Ordering::Acquire) {
            return Err(std::io::ErrorKind::WouldBlock.into());
        }
        self.socket.write(input)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.socket.flush()
    }
}
#[test]
fn unbuffered_tls_reads_progress_while_ciphertext_writes_are_blocked() {
    blocked_tls_reads(|socket| {
        protun_next::tls_unbuffered::UnbufferedTlsIo::new(
            socket,
            unbuffered_client_with_roots("localhost", roots()).unwrap(),
        )
    });
}
#[test]
fn selected_tls_reads_progress_while_ciphertext_writes_are_blocked() {
    blocked_tls_reads(|socket| {
        tls::TlsIo::new(
            socket,
            tls::client_with_roots("localhost", roots()).unwrap(),
        )
    });
}
fn blocked_tls_reads<S: Read + Write>(make: impl FnOnce(WriteGate) -> S) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (ready, wait_ready) = std::sync::mpsc::channel();
    let (start_inbound, wait_start_inbound) = std::sync::mpsc::channel();
    let (release, wait_release) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let (tcp, _) = listener.accept().unwrap();
        let mut stream = StreamOwned::new(
            ServerConnection::new(server_version(&[&rustls::version::TLS13])).unwrap(),
            bound_socket(tcp),
        );
        let mut hello = [0; 5];
        stream.read_exact(&mut hello).unwrap();
        assert_eq!(&hello, b"hello");
        ready.send(()).unwrap();
        wait_start_inbound
            .recv_timeout(Duration::from_secs(3))
            .unwrap();
        stream.conn.refresh_traffic_keys().unwrap();
        stream.write_all(b"inbound").unwrap();
        stream.flush().unwrap();
        wait_release.recv_timeout(Duration::from_secs(5)).unwrap();
        let mut pending = [0; 7];
        stream.read_exact(&mut pending).unwrap();
        assert_eq!(&pending, b"pending");
        let mut next = [0; 4];
        stream.read_exact(&mut next).unwrap();
        assert_eq!(&next, b"next");
    });
    let blocked = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let tcp = bound_socket(TcpStream::connect(address).unwrap());
    let control = tcp.try_clone().unwrap();
    let mut stream = make(WriteGate {
        socket: tcp,
        blocked: blocked.clone(),
    });
    stream.write_all(b"hello").unwrap();
    stream.flush().unwrap();
    control.set_nonblocking(true).unwrap();
    // Buffered rustls may accept "hello" before verification. Drive the
    // handshake until the peer receives it, before blocking ciphertext writes.
    // The peer withholds application data, so this cannot pre-consume the read
    // whose independence from blocked output is under test.
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    while wait_ready.try_recv().is_err() {
        assert!(std::time::Instant::now() < deadline, "handshake deadline");
        match stream.read(&mut [0; 1]) {
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
                ) => {}
            other => panic!("unexpected pre-traffic read: {other:?}"),
        }
        match stream.flush() {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            other => panic!("handshake flush: {other:?}"),
        }
        thread::sleep(Duration::from_millis(1));
    }
    blocked.store(true, std::sync::atomic::Ordering::Release);
    assert_eq!(stream.write(b"pending").unwrap(), 7);
    start_inbound.send(()).unwrap();
    let mut incoming = [0; 64];
    let mut received = 0;
    let deadline = std::time::Instant::now() + Duration::from_secs(1);
    while std::time::Instant::now() < deadline {
        match stream.read(&mut incoming) {
            Ok(n) => {
                received = n;
                break;
            }
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
                ) => {}
            Err(e) => panic!("read: {e}"),
        }
        thread::sleep(Duration::from_millis(1));
    }
    blocked.store(false, std::sync::atomic::Ordering::Release);
    stream.flush().unwrap();
    stream.write_all(b"next").unwrap();
    stream.flush().unwrap();
    release.send(()).unwrap();
    handle.join().unwrap();
    assert_eq!(
        &incoming[..received],
        b"inbound",
        "outgoing backpressure must not block independent TLS reception"
    );
}
#[test]
fn vectored_tls_retains_accepted_bytes_across_full_buffers_and_short_socket_writes() {
    for version in [&rustls::version::TLS12, &rustls::version::TLS13] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (tcp, _) = listener.accept().unwrap();
            let mut stream = StreamOwned::new(
                ServerConnection::new(server_version(&[version])).unwrap(),
                bound_socket(tcp),
            );
            let mut body = vec![0; 80_000];
            stream.read_exact(&mut body).unwrap();
            assert!(
                body.iter()
                    .enumerate()
                    .all(|(n, byte)| *byte == (n % 251) as u8)
            );
            stream.write_all(&body).unwrap();
            stream.flush().unwrap();
        });
        let tcp = bound_socket(TcpStream::connect(address).unwrap());
        tcp.set_nonblocking(true).unwrap();
        let mut stream = tls::TlsIo::new(
            Pressured(tcp, false),
            tls::client_with_roots("localhost", roots()).unwrap(),
        );
        let body: Vec<u8> = (0..80_000).map(|n| (n % 251) as u8).collect();
        // The unbuffered adapter can yield during the handshake rather than owning
        // a pre-handshake plaintext queue. Keep retrying the unaccepted suffix only.
        let mut accepted = 0;
        let mut reply = vec![0; body.len()];
        let mut received = 0;
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while accepted < body.len() || received < body.len() {
            assert!(
                std::time::Instant::now() < deadline,
                "TLS stalled under short writes"
            );
            if accepted < body.len() {
                let middle = (accepted + 900).min(body.len());
                match stream.write_vectored(&[
                    std::io::IoSlice::new(&body[accepted..middle]),
                    std::io::IoSlice::new(&body[middle..]),
                ]) {
                    Ok(n) => accepted += n,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                    Err(e) => panic!("write: {e}"),
                }
            }
            match stream.flush() {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => panic!("flush: {e}"),
            }
            match stream.read(&mut reply[received..]) {
                Ok(0) => panic!("early EOF"),
                Ok(n) => received += n,
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => panic!("read: {e}"),
            }
            thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(reply, body);
        handle.join().unwrap();
    }
}

#[test]
fn selected_tls_half_close_and_key_update_preserve_bytes() {
    for version in [&rustls::version::TLS12, &rustls::version::TLS13] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (tcp, _) = listener.accept().unwrap();
            let mut peer = StreamOwned::new(
                ServerConnection::new(server_version(&[version])).unwrap(),
                bound_socket(tcp),
            );
            let mut hello = [0; 5];
            peer.read_exact(&mut hello).unwrap();
            assert_eq!(&hello, b"hello");
            if version.version == rustls::ProtocolVersion::TLSv1_3 {
                peer.conn.refresh_traffic_keys().unwrap();
            }
            peer.write_all(b"inbound").unwrap();
            peer.conn.send_close_notify();
            peer.flush().unwrap();
            let mut ack = [0; 3];
            peer.read_exact(&mut ack).unwrap();
            assert_eq!(&ack, b"ack");
            assert_eq!(peer.read(&mut [0; 1]).unwrap(), 0);
        });
        let mut client = tls::TlsIo::new(
            bound_socket(TcpStream::connect(address).unwrap()),
            tls::client_with_roots("localhost", roots()).unwrap(),
        );
        client.write_all(b"hello").unwrap();
        client.flush().unwrap();
        let mut body = Vec::new();
        client.read_to_end(&mut body).unwrap();
        assert_eq!(body, b"inbound");
        if version.version == rustls::ProtocolVersion::TLSv1_3 {
            client.refresh_traffic_keys().unwrap();
        } else {
            assert!(client.refresh_traffic_keys().is_err());
        }
        client.write_all(b"ack").unwrap();
        client.queue_close_notify().unwrap();
        client.flush().unwrap();
        client.queue_close_notify().unwrap();
        assert_eq!(
            client.write(b"after close").unwrap_err().kind(),
            std::io::ErrorKind::BrokenPipe
        );
        assert!(client.refresh_traffic_keys().is_err());
        handle.join().unwrap();
    }
}
