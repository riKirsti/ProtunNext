//! Isolated-process production-worker probe. Synthetic IPv4 UDP echo workload.
//! The peer and load generator always use a separately built baseline binary.
use crate::{
    config::WireguardConfig,
    connection::Connection,
    linux,
    packet::{Packet, PacketPool, PacketQueue},
    session::{Output, Session},
    stream::{FramedIo, Receive},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    os::fd::{AsFd, AsRawFd},
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
#[path = "../tests/common/mod.rs"]
mod common;
fn emit(v: serde_json::Value) {
    println!("NEXT_JSON {}", v);
}
fn ip(a: &[&str]) {
    assert!(Command::new("ip").args(a).status().unwrap().success());
}
fn cpu() -> f64 {
    let mut t: libc::timespec = unsafe { std::mem::zeroed() };
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut t) },
        0
    );
    t.tv_sec as f64 + t.tv_nsec as f64 / 1e9
}
fn rss() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .unwrap()
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap()
}
fn pin(key: &str) {
    if let Ok(value) = std::env::var(key) {
        let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
        for n in value.split(',') {
            unsafe {
                libc::CPU_SET(n.parse().unwrap(), &mut set);
            }
        }
        assert_eq!(
            unsafe { libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set) },
            0
        );
    }
}
struct OwnedChild(Child);
impl Drop for OwnedChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
static STOP: AtomicBool = AtomicBool::new(false);
extern "C" fn stop(_: libc::c_int) {
    STOP.store(true, Ordering::Relaxed);
}
fn echo(p: &mut [u8]) {
    assert_eq!(p[0] >> 4, 4);
    let h = (p[0] & 15) as usize * 4;
    assert_eq!(p[9], 17);
    for i in 0..4 {
        p.swap(12 + i, 16 + i);
    }
    for i in 0..2 {
        p.swap(h + i, h + 2 + i);
    }
    p[h + 6..h + 8].fill(0);
    p[10..12].fill(0);
    let mut sum: u32 = p[..h]
        .chunks_exact(2)
        .map(|x| u16::from_be_bytes([x[0], x[1]]) as u32)
        .sum();
    while sum > 65535 {
        sum = (sum & 65535) + (sum >> 16);
    }
    p[10..12].copy_from_slice(&(!(sum as u16)).to_be_bytes());
}
trait Duplex: Read + Write + Send {
    fn metadata(&self) -> Option<String> {
        None
    }
}
impl Duplex for TcpStream {}
impl Duplex for rustls::StreamOwned<rustls::ServerConnection, TcpStream> {
    fn metadata(&self) -> Option<String> {
        Some(format!(
            "{:?}/{:?}",
            self.conn.protocol_version(),
            self.conn.negotiated_cipher_suite()
        ))
    }
}
enum Peer {
    Udp {
        socket: UdpSocket,
        tx: crate::linux_batch::Batch,
        rx: crate::linux_batch::Batch,
        first: Option<Vec<u8>>,
    },
    Stream(FramedIo<Box<dyn Duplex>>),
}
impl Peer {
    fn receive(&mut self, p: &mut Packet, pool: &mut PacketPool) -> bool {
        match self {
            Self::Udp {
                socket, rx, first, ..
            } => {
                if let Some(v) = first.take() {
                    p.writable()[..v.len()].copy_from_slice(&v);
                    p.set_len(v.len()).unwrap();
                    true
                } else {
                    matches!(rx.receive(socket, p, pool).unwrap(), Receive::Packet)
                }
            }
            Self::Stream(s) => match s.receive(p) {
                Ok(Receive::Packet) => true,
                Ok(_) => false,
                Err(crate::Error::Io(e))
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock
                            | std::io::ErrorKind::TimedOut
                            | std::io::ErrorKind::Interrupted
                            | std::io::ErrorKind::UnexpectedEof
                    ) =>
                {
                    false
                }
                Err(e) => panic!("peer receive {e}"),
            },
        }
    }
    fn queue(&mut self, data: &[u8], q: &mut PacketQueue, pool: &mut PacketPool) {
        let mut p = pool.take().unwrap();
        p.writable()[..data.len()].copy_from_slice(data);
        p.set_len(data.len()).unwrap();
        match self {
            Self::Udp { .. } => q.push(p).ok().unwrap(),
            Self::Stream(s) => s.queue(p).ok().unwrap(),
        }
    }
    fn flush(&mut self, q: &mut PacketQueue, pool: &mut PacketPool) {
        match self {
            Self::Udp { socket, tx, .. } => {
                while tx.flush(socket, q, pool).unwrap() {
                    thread::yield_now();
                }
            }
            Self::Stream(s) => {
                while s.pending() > 0 {
                    s.flush(pool).unwrap();
                }
                match s.io_mut().flush() {
                    Ok(()) => {}
                    Err(e)
                        if matches!(
                            e.kind(),
                            std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                        ) => {}
                    Err(e)
                        if STOP.load(Ordering::Relaxed)
                            && matches!(
                                e.kind(),
                                std::io::ErrorKind::BrokenPipe
                                    | std::io::ErrorKind::ConnectionReset
                                    | std::io::ErrorKind::NotConnected
                            ) => {}
                    Err(e) => panic!("peer flush {e}"),
                }
            }
        }
    }
}

fn peer(proto: &str, window: usize) {
    pin("NEXT_PEER_CPUS");
    unsafe {
        libc::signal(libc::SIGTERM, stop as libc::sighandler_t);
    }
    let udp = proto == "udp";
    let u = UdpSocket::bind("127.0.0.1:0").unwrap();
    u.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    println!(
        "NEXT_READY {}",
        if udp {
            u.local_addr().unwrap()
        } else {
            listener.local_addr().unwrap()
        }
    );
    std::io::stdout().flush().unwrap();
    let mut peer = if udp {
        let mut v = vec![0; 65535];
        let (n, from) = u.recv_from(&mut v).unwrap();
        v.truncate(n);
        u.connect(from).unwrap();
        u.set_nonblocking(true).unwrap();
        let tx = crate::linux_batch::Batch::new(&u, true);
        let rx = crate::linux_batch::Batch::new(&u, true);
        Peer::Udp {
            socket: u,
            tx,
            rx,
            first: Some(v),
        }
    } else {
        let (tcp, _) = listener.accept().unwrap();
        tcp.set_nodelay(true).unwrap();
        tcp.set_nonblocking(true).unwrap();
        let io: Box<dyn Duplex> = if proto == "stealth" {
            Box::new(rustls::StreamOwned::new(
                rustls::ServerConnection::new(common::server()).unwrap(),
                tcp,
            ))
        } else {
            Box::new(tcp)
        };
        Peer::Stream(FramedIo::new(io, 64).unwrap())
    };
    let now = linux::boottime().unwrap();
    let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut s = Session::new([9; 32], Session::public_key([7; 32]), None, now, wall).unwrap();
    let mut pool = PacketPool::new(132, 65535).unwrap();
    let mut p = pool.take().unwrap();
    let mut out = vec![0; 65535];
    let mut wire = vec![0; 65535];
    let mut q = PacketQueue::new(64).unwrap();
    let start = Instant::now();
    let c = cpu();
    let mut count = 0u64;
    let mut handshakes = 0u64;
    let mut timer = Instant::now();
    while !STOP.load(Ordering::Relaxed) {
        assert!(start.elapsed() < Duration::from_secs(900));
        if timer.elapsed() > Duration::from_secs(1) {
            if let Output::Network(n) = s.advance(linux::boottime().unwrap(), &mut out).unwrap() {
                peer.queue(&out[..n], &mut q, &mut pool);
            }
            timer = Instant::now();
        }
        let mut got = false;
        for _ in 0..32 {
            if !peer.receive(&mut p, &mut pool) {
                break;
            }
            got = true;
            match s.decrypt(p.data(), &mut out).unwrap() {
                Output::Network(n) => {
                    if n == 92 {
                        handshakes += 1;
                    }
                    peer.queue(&out[..n], &mut q, &mut pool);
                    peer.flush(&mut q, &mut pool);
                }
                Output::Tunnel(n) => {
                    echo(&mut out[..n]);
                    let Output::Network(n) = s.encrypt(&out[..n], &mut wire).unwrap() else {
                        panic!()
                    };
                    peer.queue(&wire[..n], &mut q, &mut pool);
                    count += 1;
                    if count % 16 == 0 || window == 1 {
                        peer.flush(&mut q, &mut pool);
                    }
                }
                _ => {}
            }
        }
        peer.flush(&mut q, &mut pool);
        if !got {
            thread::yield_now();
        }
    }
    let tls = match &mut peer {
        Peer::Stream(s) => s.io_mut().metadata(),
        _ => None,
    };
    emit(
        serde_json::json!({"role":"peer","cpu_seconds":cpu()-c,"rss_kib":rss(),"packets":count,"handshakes":handshakes,"tls":tls}),
    );
}
fn wait_file(key: &str) {
    if let Ok(path) = std::env::var(key) {
        let end = Instant::now() + Duration::from_secs(30);
        while !std::path::Path::new(&path).exists() {
            assert!(Instant::now() < end, "synchronization timeout");
            thread::sleep(Duration::from_millis(10));
        }
    }
}
fn load(size: usize, window: usize, seconds: f64, flows: usize) {
    pin("NEXT_LOAD_CPUS");
    assert!((8..=2048).contains(&size));
    assert!((1..=64).contains(&window));
    assert!(flows > 0 && flows <= window && window % flows == 0);
    let sockets: Vec<_> = (0..flows)
        .map(|_| {
            let a = UdpSocket::bind("10.2.0.2:0").unwrap();
            a.connect("10.2.0.3:9000").unwrap();
            a.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            a
        })
        .collect();
    let mut body = vec![0x5a; size];
    let mut recv = vec![0; 65535];
    let cold = Instant::now();
    sockets[0].send(&body).unwrap();
    let n = sockets[0].recv(&mut recv).unwrap();
    assert_eq!(&recv[..n], body);
    println!("NEXT_READY {}", cold.elapsed().as_secs_f64() * 1000.0);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_exact(&mut [0; 1]).unwrap();
    let c = cpu();
    let start = Instant::now();
    let mut packets = 0usize;
    let mut latencies = Vec::new();
    while start.elapsed().as_secs_f64() < seconds {
        let t = Instant::now();
        for j in 0..window {
            body[..8].copy_from_slice(&((packets + j) as u64).to_le_bytes());
            sockets[j % flows].send(&body).unwrap();
        }
        let mut seen = 0u64;
        for j in 0..window {
            let n = sockets[j % flows].recv(&mut recv).unwrap();
            assert_eq!(n, size);
            let seq = u64::from_le_bytes(recv[..8].try_into().unwrap()) as usize;
            assert!(seq >= packets && seq < packets + window);
            let bit = 1u64 << (seq - packets);
            assert_eq!(seen & bit, 0, "duplicate response");
            seen |= bit;
            assert!(recv[8..n].iter().all(|b| *b == 0x5a));
        }
        packets += window;
        if window == 1 {
            latencies.push(t.elapsed().as_secs_f64() * 1e6);
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    latencies.sort_by(f64::total_cmp);
    emit(
        serde_json::json!({"role":"load","bytes":packets*size,"packets":packets,"seconds":elapsed,"Mbps":packets as f64*size as f64*8.0/elapsed/1e6,"cpu_seconds":cpu()-c,"rss_kib":rss(),"p99_us":if latencies.is_empty(){None}else{Some(latencies[(latencies.len()-1)*99/100])}}),
    );
}
fn client(peer_bin: &str, proto: &str, size: usize, window: usize, seconds: f64, flows: usize) {
    // Start the fixed peer before restricting this process and its future worker.
    let mut peer = OwnedChild(
        Command::new(peer_bin)
            .args(["peer", proto, &window.to_string()])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let mut peer_out = BufReader::new(peer.0.stdout.take().unwrap());
    let mut line = String::new();
    peer_out.read_line(&mut line).unwrap();
    let endpoint = line
        .trim()
        .strip_prefix("NEXT_READY ")
        .expect("peer did not become ready")
        .to_owned();
    pin("NEXT_CLIENT_CPUS");
    let tun = File::options()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .unwrap();
    let mut req = [0u8; 40];
    req[..4].copy_from_slice(b"pnh1");
    req[16..18].copy_from_slice(&((libc::IFF_TUN | libc::IFF_NO_PI) as i16).to_ne_bytes());
    assert_eq!(
        unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, req.as_mut_ptr()) },
        0
    );
    ip(&["link", "set", "lo", "up"]);
    ip(&["addr", "add", "10.2.0.2/32", "dev", "pnh1"]);
    ip(&["link", "set", "pnh1", "mtu", "9000", "up"]);
    ip(&["route", "add", "10.2.0.3/32", "dev", "pnh1"]);
    let text = format!(
        "[Interface]\nPrivateKey={}\nAddress=10.2.0.2/32\nMTU=1280\n[Peer]\nPublicKey={}\nEndpoint={endpoint}\nAllowedIPs=10.2.0.3/32\n",
        STANDARD.encode([7; 32]),
        STANDARD.encode(Session::public_key([9; 32]))
    );
    let conf = WireguardConfig::parse(&text).unwrap();
    let tr = match proto {
        "udp" => linux::Transport::Udp,
        "tcp" => linux::Transport::Tcp,
        "stealth" => {
            linux::Transport::Stealth(crate::stealth::SniStrategy::Explicit("cover.test".into()))
        }
        _ => panic!(),
    };
    let mut con = Connection::start(tun.as_fd(), conf, "lo".into(), tr).unwrap();
    let mut generator = OwnedChild(
        Command::new(peer_bin)
            .args([
                "load",
                &size.to_string(),
                &window.to_string(),
                &seconds.to_string(),
                &flows.to_string(),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let mut load_out = BufReader::new(generator.0.stdout.take().unwrap());
    line.clear();
    load_out.read_line(&mut line).unwrap();
    let cold: f64 = line
        .trim()
        .strip_prefix("NEXT_READY ")
        .expect("load did not become ready")
        .parse()
        .unwrap();
    if let Ok(path) = std::env::var("NEXT_PROFILE_READY") {
        std::fs::write(path, std::process::id().to_string()).unwrap();
        wait_file("NEXT_PROFILE_GO");
    }
    let c = cpu();
    let start = Instant::now();
    generator.0.stdin.take().unwrap().write_all(b"G").unwrap();
    let mut rest = String::new();
    load_out.read_to_string(&mut rest).unwrap();
    assert!(generator.0.wait().unwrap().success());
    let used = cpu() - c;
    let elapsed = start.elapsed().as_secs_f64();
    let before = con.snapshot();
    con.disconnect();
    // Tell the controlled peer to stop before waiting for transport teardown.
    unsafe {
        libc::kill(peer.0.id() as libc::pid_t, libc::SIGTERM);
    }
    let counters = con.disconnect_and_wait().unwrap();
    assert_eq!(counters.invalid_packets, 0);
    if proto == "udp" && window > 1 {
        assert!(
            counters.io.gso_datagrams > 0,
            "requested workload failed to exercise GSO"
        );
    }
    unsafe {
        libc::kill(peer.0.id() as libc::pid_t, libc::SIGTERM);
    };
    let mut details = String::new();
    peer_out.read_to_string(&mut details).unwrap();
    assert!(peer.0.wait().unwrap().success());
    let metric: serde_json::Value = serde_json::from_str(
        rest.lines()
            .find_map(|l| l.strip_prefix("NEXT_JSON "))
            .unwrap(),
    )
    .unwrap();
    let bytes = metric["bytes"].as_u64().unwrap();
    let peer_metric: serde_json::Value = serde_json::from_str(
        details
            .lines()
            .find_map(|l| l.strip_prefix("NEXT_JSON "))
            .unwrap(),
    )
    .unwrap();
    emit(
        serde_json::json!({"effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build"),"role":"client","protocol":proto,"size":size,"window":window,"flows":flows,"seconds":elapsed,"client_cpu_seconds":used,"client_cpu_ns_per_byte":used*1e9/bytes as f64,"client_rss_kib":rss(),"load":metric,"peer":peer_metric,"cold_ms":cold,"counters":counters,"state":format!("{:?}",before.state),"kernel":std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap().trim(),"arch":std::env::consts::ARCH,"scope":"IPv4 UDP echo through actual Connection/TUN/WireGuard worker; peer and generator are separate processes built from frozen main. Process CPU includes synchronous system time, not all asynchronous kernel work. Not a TCP/HTTPS, NIC or Proton WAN acceptance test."}),
    );
}
pub fn run() {
    let a: Vec<_> = std::env::args().collect();
    if a[1] == "identity" {
        println!(
            "{}",
            serde_json::json!({"effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build")})
        );
        return;
    }
    match a[1].as_str() {
        "peer" => peer(&a[2], a[3].parse().unwrap()),
        "load" => load(
            a[2].parse().unwrap(),
            a[3].parse().unwrap(),
            a[4].parse().unwrap(),
            a[5].parse().unwrap(),
        ),
        "client" => client(
            &a[2],
            &a[3],
            a[4].parse().unwrap(),
            a[5].parse().unwrap(),
            a[6].parse().unwrap(),
            a[7].parse().unwrap(),
        ),
        _ => panic!("mode"),
    }
}
