#![cfg(target_os = "linux")]
//! Opt-in CAP_NET_ADMIN test, run only inside a fresh CI network namespace.
mod common;
use base64::{Engine, engine::general_purpose::STANDARD};
use protun_next::{
    config::WireguardConfig,
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
    os::fd::{AsRawFd, FromRawFd},
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
fn ip(args: &[&str]) {
    assert!(Command::new("ip").args(args).status().unwrap().success());
}
fn echo_ipv4(packet: &mut [u8]) {
    assert_eq!(packet[0] >> 4, 4);
    let header = (packet[0] & 15) as usize * 4;
    assert!(header >= 20 && packet.len() >= header + 8);
    assert_eq!(packet[9], 17);
    for n in 0..4 {
        packet.swap(12 + n, 16 + n);
    }
    for n in 0..2 {
        packet.swap(header + n, header + 2 + n);
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
    request[16..18].copy_from_slice(&((libc::IFF_TUN | libc::IFF_NO_PI) as i16).to_ne_bytes());
    // SAFETY: valid TUN descriptor and 40-byte ifreq storage, private test namespace.
    let configured = unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, request.as_mut_ptr()) };
    assert_eq!(configured, 0);
    ip(&["link", "set", "lo", "up"]);
    ip(&["addr", "add", "10.2.0.2/32", "dev", "pn0"]);
    ip(&["link", "set", "pn0", "mtu", "1280", "up"]);
    ip(&["route", "add", "10.2.0.3/32", "dev", "pn0"]);
    for mode in 0..5 {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = if mode == 0 {
            peer.local_addr().unwrap()
        } else {
            listener.local_addr().unwrap()
        };
        let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let now = linux::boottime().unwrap();
        let (release, wait_release) = std::sync::mpsc::channel();
        let reference = thread::spawn(move || {
            let mut transport = if mode == 0 {
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
                let io: Box<dyn Duplex> = if mode >= 2 {
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
            loop {
                transport.receive(&mut wire);
                let result = match session.decrypt(wire.data(), &mut out) {
                    Ok(result) => result,
                    Err(_) if mode == 4 => {
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
                        echo_ipv4(&mut out[..n]);
                        let Output::Network(n) = session.encrypt(&out[..n], &mut reply).unwrap()
                        else {
                            panic!("echo encrypt")
                        };
                        transport.send(&reply[..n], &mut pool);
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
            STANDARD.encode(Session::public_key(if mode == 4 {
                [11; 32]
            } else {
                [9; 32]
            }))
        );
        let config = WireguardConfig::parse(&text).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = stop.clone();
        // SAFETY: duplicates a valid fd; the worker exclusively owns the duplicate File.
        let fd = unsafe { libc::fcntl(tun.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 3) };
        assert!(fd >= 0);
        // SAFETY: newly duplicated descriptor is owned exactly once.
        let owned = unsafe { File::from_raw_fd(fd) };
        let worker = thread::spawn(move || {
            let result = match mode {
                0 => linux::run_udp(owned.as_raw_fd(), &config, "lo", &worker_stop),
                1 => linux::run_tcp(owned.as_raw_fd(), &config, "lo", &worker_stop),
                2 => linux::run_tls(
                    owned.as_raw_fd(),
                    &config,
                    "lo",
                    tls::client_with_roots("localhost", common::roots()).unwrap(),
                    &worker_stop,
                ),
                _ => linux::run_stealth(
                    owned.as_raw_fd(),
                    &config,
                    "lo",
                    &protun_next::stealth::SniStrategy::Explicit("unrelated.test".into()),
                    &worker_stop,
                ),
            };
            drop(owned);
            result
        });
        let app = UdpSocket::bind("10.2.0.2:0").unwrap();
        app.set_read_timeout(Some(if mode == 4 {
            Duration::from_millis(500)
        } else {
            Duration::from_secs(8)
        }))
        .unwrap();
        app.send_to(b"real kernel TUN and UDP", "10.2.0.3:9000")
            .unwrap();
        let mut response = [0; 128];
        let received = app.recv_from(&mut response);
        stop.store(true, Ordering::Relaxed);
        let counters = worker.join().unwrap().unwrap();
        release.send(()).unwrap();
        reference.join().unwrap();
        if mode == 4 {
            let error = received.unwrap_err();
            assert!(matches!(
                error.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ));
            assert_eq!(
                counters.decrypted_packets, 0,
                "untrusted TLS cannot authenticate a WG peer"
            );
            continue;
        }
        let (n, source) = received.unwrap();
        assert_eq!(&response[..n], b"real kernel TUN and UDP");
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
