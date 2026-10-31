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
    let cert = CertificateDer::from(include_bytes!("fixtures/server-TEST-ONLY.der").to_vec());
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
        include_bytes!("fixtures/private-PUBLIC-TEST-ONLY.der").to_vec(),
    ));
    Arc::new(
        ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .unwrap(),
    )
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
