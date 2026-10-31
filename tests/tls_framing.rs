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
