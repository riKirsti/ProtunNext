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
            tls::unbuffered_client_with_roots("localhost", roots()).unwrap(),
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
    let tls = StreamOwned::new(tls::client_with_roots("localhost", roots()).unwrap(), tcp);
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
    let mut stream = StreamOwned::new(
        tls::client_with_roots("wrong.invalid", roots()).unwrap(),
        tcp,
    );
    assert!(stream.write_all(b"must not be accepted").is_err());
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
            tls::unbuffered_client_with_roots(
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
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (ready, wait_ready) = std::sync::mpsc::channel();
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
        stream.conn.refresh_traffic_keys().unwrap();
        stream.write_all(b"inbound").unwrap();
        stream.flush().unwrap();
        ready.send(()).unwrap();
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
    let mut stream = protun_next::tls_unbuffered::UnbufferedTlsIo::new(
        WriteGate {
            socket: tcp,
            blocked: blocked.clone(),
        },
        tls::unbuffered_client_with_roots("localhost", roots()).unwrap(),
    );
    stream.write_all(b"hello").unwrap();
    stream.flush().unwrap();
    wait_ready.recv_timeout(Duration::from_secs(3)).unwrap();
    stream.socket_mut().socket.set_nonblocking(true).unwrap();
    blocked.store(true, std::sync::atomic::Ordering::Release);
    assert_eq!(stream.write(b"pending").unwrap(), 7);
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
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let handle = thread::spawn(move || {
        let (tcp, _) = listener.accept().unwrap();
        let mut stream =
            StreamOwned::new(ServerConnection::new(server()).unwrap(), bound_socket(tcp));
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
    let slices = [
        std::io::IoSlice::new(&body[..20_000]),
        std::io::IoSlice::new(&body[20_000..50_000]),
        std::io::IoSlice::new(&body[50_000..]),
    ];
    let mut accepted = stream.write_vectored(&slices).unwrap();
    assert!(
        accepted > 20_000 && accepted < body.len(),
        "must consume multiple slices within the bounded pre-handshake buffer"
    );
    assert_eq!(
        stream.write(&body[accepted..]).unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
    let mut reply = vec![0; body.len()];
    let mut received = 0;
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while accepted < body.len() || received < body.len() {
        assert!(
            std::time::Instant::now() < deadline,
            "TLS stalled under short writes"
        );
        if accepted < body.len() {
            match stream.write(&body[accepted..]) {
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
