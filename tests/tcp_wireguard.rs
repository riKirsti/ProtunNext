//! Real loopback sockets and actual Noise/WireGuard handshakes. No Proton account.
mod common;
use protun_next::{
    packet::{Packet, PacketPool},
    session::{Output, Session},
    stream::{FramedIo, Receive},
    tls::{self, TlsIo},
};
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
trait Duplex: Read + Write + Send {
    fn buffered_write(&self) -> bool {
        false
    }
}
impl Duplex for TcpStream {}
impl Duplex for rustls::StreamOwned<rustls::ServerConnection, TcpStream> {}
impl Duplex for TlsIo<TcpStream> {
    fn buffered_write(&self) -> bool {
        self.wants_write()
    }
}
fn push(stream: &mut FramedIo<Box<dyn Duplex>>, pool: &mut PacketPool, packet: Packet) {
    assert!(stream.queue(packet).is_ok());
    let deadline = Instant::now() + Duration::from_secs(5);
    while stream.pending() > 0 || stream.io().buffered_write() {
        assert!(Instant::now() < deadline, "write stalled");
        stream.flush(pool).unwrap();
        match stream.io_mut().flush() {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(1))
            }
            result => result.unwrap(),
        }
    }
}
fn receive(stream: &mut FramedIo<Box<dyn Duplex>>, packet: &mut Packet) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        assert!(Instant::now() < deadline, "read stalled");
        match stream.receive(packet).unwrap() {
            Receive::Packet => return,
            Receive::Yield => continue,
            Receive::WouldBlock => thread::sleep(Duration::from_millis(1)),
            other => panic!("unexpected {other:?}"),
        }
    }
}
#[test]
fn authenticated_wireguard_over_real_loopback_tcp() {
    exchange(false);
}
#[test]
fn authenticated_wireguard_over_nonblocking_loopback_tls() {
    exchange(true);
}
fn exchange(use_tls: bool) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let server = thread::spawn(move || {
        let (tcp, _) = listener.accept().unwrap();
        tcp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        tcp.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        tcp.set_nodelay(true).unwrap();
        let io: Box<dyn Duplex> = if use_tls {
            Box::new(rustls::StreamOwned::new(
                rustls::ServerConnection::new(common::server()).unwrap(),
                tcp,
            ))
        } else {
            Box::new(tcp)
        };
        let mut stream = FramedIo::new(io, 8).unwrap();
        let mut pool = PacketPool::new(8, 2048).unwrap();
        let mut session = Session::new(
            [9; 32],
            Session::public_key([7; 32]),
            None,
            Duration::ZERO,
            wall,
        )
        .unwrap();
        for _ in 0..8 {
            let mut incoming = pool.take().unwrap();
            receive(&mut stream, &mut incoming);
            let mut output = pool.take().unwrap();
            let result = session.decrypt(incoming.data(), output.writable()).unwrap();
            pool.recycle(incoming).unwrap();
            match result {
                Output::Network(n) => {
                    output.set_len(n).unwrap();
                    push(&mut stream, &mut pool, output);
                }
                Output::Tunnel(n) => {
                    output.set_len(n).unwrap();
                    assert_eq!(n, 1280);
                    let mut reply = pool.take().unwrap();
                    let Output::Network(n) =
                        session.encrypt(output.data(), reply.writable()).unwrap()
                    else {
                        panic!("reply")
                    };
                    reply.set_len(n).unwrap();
                    pool.recycle(output).unwrap();
                    push(&mut stream, &mut pool, reply);
                    return;
                }
                Output::None => pool.recycle(output).unwrap(),
            }
        }
        panic!("no authenticated application packet");
    });
    let tcp = TcpStream::connect(address).unwrap();
    tcp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    tcp.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
    tcp.set_nodelay(true).unwrap();
    let io: Box<dyn Duplex> = if use_tls {
        tcp.set_nonblocking(true).unwrap();
        Box::new(TlsIo::new(
            tcp,
            tls::client_with_roots("localhost", common::roots()).unwrap(),
        ))
    } else {
        Box::new(tcp)
    };
    let mut stream = FramedIo::new(io, 8).unwrap();
    let mut pool = PacketPool::new(8, 2048).unwrap();
    let mut session = Session::new(
        [7; 32],
        Session::public_key([9; 32]),
        None,
        Duration::ZERO,
        wall,
    )
    .unwrap();
    let mut ip = [0; 1280];
    ip[0] = 0x45;
    ip[2..4].copy_from_slice(&1280u16.to_be_bytes());
    ip[8] = 64;
    ip[9] = 17;
    let mut initiation = pool.take().unwrap();
    let Output::Network(n) = session.encrypt(&ip, initiation.writable()).unwrap() else {
        panic!("initiation")
    };
    initiation.set_len(n).unwrap();
    push(&mut stream, &mut pool, initiation);
    let mut response = pool.take().unwrap();
    receive(&mut stream, &mut response);
    let mut confirmation = pool.take().unwrap();
    let Output::Network(n) = session
        .decrypt(response.data(), confirmation.writable())
        .unwrap()
    else {
        panic!("confirmation")
    };
    confirmation.set_len(n).unwrap();
    pool.recycle(response).unwrap();
    push(&mut stream, &mut pool, confirmation);
    loop {
        let mut queued = pool.take().unwrap();
        match session.drain(queued.writable()).unwrap() {
            Output::Network(n) => {
                queued.set_len(n).unwrap();
                push(&mut stream, &mut pool, queued);
            }
            _ => {
                pool.recycle(queued).unwrap();
                break;
            }
        }
    }
    let mut echoed = pool.take().unwrap();
    receive(&mut stream, &mut echoed);
    let mut plain = [0; 2048];
    let Output::Tunnel(n) = session.decrypt(echoed.data(), &mut plain).unwrap() else {
        panic!("echo")
    };
    assert_eq!(&plain[..n], ip);
    server.join().unwrap();
}
