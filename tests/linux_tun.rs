#![cfg(target_os = "linux")]
//! Opt-in CAP_NET_ADMIN test, run only inside a fresh CI network namespace.
use base64::{Engine, engine::general_purpose::STANDARD};
use protun_next::{
    config::WireguardConfig,
    linux,
    session::{Output, Session},
};
use std::{
    fs::File,
    net::UdpSocket,
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
fn actual_udp_engine_tun_roundtrip() {
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
    let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    let endpoint = peer.local_addr().unwrap();
    let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now = linux::boottime().unwrap();
    let reference = thread::spawn(move || {
        let mut session =
            Session::new([9; 32], Session::public_key([7; 32]), None, now, wall).unwrap();
        let mut wire = [0; 65535];
        let mut out = [0; 65535];
        let mut reply = [0; 65535];
        loop {
            let (n, address) = peer.recv_from(&mut wire).unwrap();
            let result = session.decrypt(&wire[..n], &mut out).unwrap();
            match result {
                Output::Network(n) => {
                    peer.send_to(&out[..n], address).unwrap();
                }
                Output::Tunnel(n) => {
                    echo_ipv4(&mut out[..n]);
                    let Output::Network(n) = session.encrypt(&out[..n], &mut reply).unwrap() else {
                        panic!("echo encrypt")
                    };
                    peer.send_to(&reply[..n], address).unwrap();
                    return;
                }
                _ => {}
            }
        }
    });
    let text = format!(
        "[Interface]\nPrivateKey={}\nAddress=10.2.0.2/32\nMTU=1280\n[Peer]\nPublicKey={}\nEndpoint={endpoint}\nAllowedIPs=10.2.0.3/32\n",
        STANDARD.encode([7; 32]),
        STANDARD.encode(Session::public_key([9; 32]))
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
        let result = linux::run_udp(owned.as_raw_fd(), &config, "lo", &worker_stop);
        drop(owned);
        result
    });
    let app = UdpSocket::bind("10.2.0.2:0").unwrap();
    app.set_read_timeout(Some(Duration::from_secs(8))).unwrap();
    app.send_to(b"real kernel TUN and UDP", "10.2.0.3:9000")
        .unwrap();
    let mut response = [0; 128];
    let received = app.recv_from(&mut response);
    stop.store(true, Ordering::Relaxed);
    let counters = worker.join().unwrap().unwrap();
    reference.join().unwrap();
    let (n, source) = received.unwrap();
    assert_eq!(&response[..n], b"real kernel TUN and UDP");
    assert_eq!(source, "10.2.0.3:9000".parse().unwrap());
    assert!(counters.decrypted_packets >= 1);
    assert!(counters.tun_reads >= 1);
}
