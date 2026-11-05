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
    io::{BufRead, BufReader, IoSlice, Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    os::fd::{AsFd, AsRawFd},
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
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
static PEER_CPU_START: AtomicU64 = AtomicU64::new(0);
static PEER_WALL_START: AtomicU64 = AtomicU64::new(0);
fn monotonic_ns() -> u64 {
    let mut t: libc::timespec = unsafe { std::mem::zeroed() };
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t) },
        0
    );
    t.tv_sec as u64 * 1_000_000_000 + t.tv_nsec as u64
}
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
    fn fd(&self) -> i32;
    fn pending_write(&self) -> bool {
        false
    }
    fn metadata(&self) -> Option<String> {
        None
    }
}
impl Duplex for TcpStream {
    fn fd(&self) -> i32 {
        self.as_raw_fd()
    }
}
#[derive(Default)]
struct RecordSizes {
    header: [u8; 5],
    used: usize,
    remaining: usize,
    encrypted: [u64; 3],
}
impl RecordSizes {
    fn feed(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            if self.remaining > 0 {
                let n = self.remaining.min(bytes.len());
                self.remaining -= n;
                bytes = &bytes[n..];
                continue;
            }
            let n = (5 - self.used).min(bytes.len());
            self.header[self.used..self.used + n].copy_from_slice(&bytes[..n]);
            self.used += n;
            bytes = &bytes[n..];
            if self.used == 5 {
                let n = u16::from_be_bytes([self.header[3], self.header[4]]) as usize;
                assert!(n <= 18432);
                if self.header[0] == 23 {
                    self.encrypted[if n <= 2048 {
                        0
                    } else if n <= 8192 {
                        1
                    } else {
                        2
                    }] += 1;
                }
                self.remaining = n;
                self.used = 0;
            }
        }
    }
}
struct PeerWire {
    socket: TcpStream,
    records: RecordSizes,
}
impl Read for PeerWire {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        self.socket.read(b)
    }
}
impl Write for PeerWire {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        let n = self.socket.write(b)?;
        self.records.feed(&b[..n]);
        Ok(n)
    }
    fn write_vectored(&mut self, b: &[IoSlice<'_>]) -> std::io::Result<usize> {
        let n = self.socket.write_vectored(b)?;
        let mut rest = n;
        for part in b {
            let used = rest.min(part.len());
            self.records.feed(&part[..used]);
            rest -= used;
            if rest == 0 {
                break;
            }
        }
        Ok(n)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.socket.flush()
    }
}
struct PeerTls {
    inner: rustls::StreamOwned<rustls::ServerConnection, PeerWire>,
    vectored: bool,
}
impl PeerTls {
    fn new(socket: TcpStream, vectored: bool) -> Self {
        Self {
            inner: rustls::StreamOwned::new(
                rustls::ServerConnection::new(common::server()).unwrap(),
                PeerWire {
                    socket,
                    records: RecordSizes::default(),
                },
            ),
            vectored,
        }
    }
}
impl Read for PeerTls {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(b)
    }
}
impl Write for PeerTls {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.inner.write(b)
    }
    fn write_vectored(&mut self, b: &[IoSlice<'_>]) -> std::io::Result<usize> {
        if self.vectored {
            rustls::Stream::new(&mut self.inner.conn, &mut self.inner.sock).write_vectored(b)
        } else {
            self.inner.write_vectored(b)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
impl Duplex for PeerTls {
    fn fd(&self) -> i32 {
        self.inner.sock.socket.as_raw_fd()
    }
    fn pending_write(&self) -> bool {
        self.inner.conn.wants_write()
    }
    fn metadata(&self) -> Option<String> {
        Some(serde_json::json!({"version":format!("{:?}",self.inner.conn.protocol_version()),"suite":format!("{:?}",self.inner.conn.negotiated_cipher_suite()),"vectored":self.vectored,"encrypted_record_buckets_le_2k_le_8k_gt_8k":self.inner.sock.records.encrypted}).to_string())
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
    fn receive(&mut self, p: &mut Packet, pool: &mut PacketPool) -> Receive {
        match self {
            Self::Udp {
                socket, rx, first, ..
            } => {
                if let Some(v) = first.take() {
                    p.writable()[..v.len()].copy_from_slice(&v);
                    p.set_len(v.len()).unwrap();
                    Receive::Packet
                } else {
                    rx.receive(socket, p, pool).unwrap()
                }
            }
            Self::Stream(stream) => match stream.receive(p) {
                Ok(r) => r,
                Err(crate::Error::Io(e))
                    if STOP.load(Ordering::Relaxed)
                        && matches!(
                            e.kind(),
                            std::io::ErrorKind::UnexpectedEof
                                | std::io::ErrorKind::ConnectionReset
                                | std::io::ErrorKind::BrokenPipe
                        ) =>
                {
                    Receive::Eof
                }
                Err(e) => panic!("peer receive {e}"),
            },
        }
    }
    fn wait(&self, pending: bool) {
        let fd = match self {
            Self::Udp { socket, .. } => socket.as_raw_fd(),
            Self::Stream(s) => s.io().fd(),
        };
        let tls_pending = matches!(self,Self::Stream(s) if s.io().pending_write() || s.pending()>0);
        let mut poll = libc::pollfd {
            fd,
            events: libc::POLLIN
                | if pending || tls_pending {
                    libc::POLLOUT
                } else {
                    0
                },
            revents: 0,
        };
        let n = unsafe { libc::poll(&mut poll, 1, 100) };
        assert!(
            n >= 0 || std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
        );
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
    fn flush(&mut self, q: &mut PacketQueue, pool: &mut PacketPool) -> bool {
        match self {
            Self::Udp { socket, tx, .. } => tx.flush(socket, q, pool).unwrap(),
            Self::Stream(s) => {
                for _ in 0..64 {
                    match s.flush(pool).unwrap() {
                        crate::stream::Flush::Progress => {}
                        _ => break,
                    }
                }
                match s.io_mut().flush() {
                    Ok(()) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return true,
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
                s.pending() > 0 || s.io().pending_write()
            }
        }
    }
}

fn peer(proto: &str, window: usize) {
    pin("NEXT_PEER_CPUS");
    unsafe {
        libc::signal(libc::SIGTERM, stop as *const () as libc::sighandler_t);
    }
    thread::spawn(|| {
        if std::io::stdin().read_exact(&mut [0; 1]).is_ok() {
            PEER_CPU_START.store((cpu() * 1e9) as u64, Ordering::Release);
            PEER_WALL_START.store(monotonic_ns(), Ordering::Release);
        }
    });
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
            Box::new(PeerTls::new(
                tcp,
                std::env::var("NEXT_PEER_VECTORED").as_deref() != Ok("0"),
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
            match peer.receive(&mut p, &mut pool) {
                Receive::Packet => {}
                Receive::WouldBlock => break,
                Receive::Yield => continue,
                Receive::Eof => {
                    assert!(STOP.load(Ordering::Relaxed), "peer unexpected EOF");
                    break;
                }
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
        let pending = peer.flush(&mut q, &mut pool);
        if !got || pending {
            peer.wait(pending);
        }
    }
    let tls = match &mut peer {
        Peer::Stream(s) => s.io_mut().metadata(),
        _ => None,
    };
    emit(
        serde_json::json!({"effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build"),"role":"peer","cpu_seconds":(cpu()*1e9-PEER_CPU_START.load(Ordering::Acquire) as f64)/1e9,"seconds":(monotonic_ns()-PEER_WALL_START.load(Ordering::Acquire)) as f64/1e9,"timed":PEER_CPU_START.load(Ordering::Acquire)!=0,"rss_kib":rss(),"packets":count,"handshakes":handshakes,"tls":tls}),
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
    // Warm the actual data path (including buffer promotion and crypto workers).
    let shape = std::env::var("NEXT_LOAD_SHAPE").unwrap_or_else(|_| "burst".into());
    let warm = std::env::var("NEXT_WARM_SECONDS")
        .ok()
        .map(|x| x.parse().unwrap())
        .unwrap_or(1.0);
    let _ = pump(&sockets, &mut body, &mut recv, window, warm, &shape, false);
    println!("NEXT_WARM");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_exact(&mut [0; 1]).unwrap();
    let c = cpu();
    let start = Instant::now();
    let (packets, mut latencies) = pump(
        &sockets,
        &mut body,
        &mut recv,
        window,
        seconds,
        &shape,
        window == 1,
    );
    let elapsed = start.elapsed().as_secs_f64();
    let used = cpu() - c;
    // Tell the parent immediately: sorting/serializing latency data is outside
    // BOTH the generator and client measurement intervals.
    println!("NEXT_DONE");
    std::io::stdout().flush().unwrap();
    latencies.sort_by(f64::total_cmp);
    emit(
        serde_json::json!({"effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build"),"role":"load","shape":shape,"bytes":packets*size,"packets":packets,"seconds":elapsed,"Mbps":packets as f64*size as f64*8.0/elapsed/1e6,"cpu_seconds":used,"rss_kib":rss(),"p99_us":if latencies.is_empty(){None}else{Some(latencies[(latencies.len()-1)*99/100])}}),
    );
}
fn pump(
    sockets: &[UdpSocket],
    body: &mut [u8],
    recv: &mut [u8],
    window: usize,
    seconds: f64,
    shape: &str,
    latency: bool,
) -> (usize, Vec<f64>) {
    assert!(seconds.is_finite() && seconds > 0.0 && seconds <= 30.0);
    assert!(shape == "burst" || shape == "sliding");
    let mut latencies = Vec::with_capacity(if latency { 1_000_000 } else { 0 });
    let start = Instant::now();
    let mut packets = 0;
    if shape == "burst" || window == 1 {
        while start.elapsed().as_secs_f64() < seconds {
            let t = Instant::now();
            for j in 0..window {
                body[..8].copy_from_slice(&((packets + j) as u64).to_le_bytes());
                sockets[j % sockets.len()].send(body).unwrap();
            }
            let mut seen = 0u64;
            for j in 0..window {
                let n = sockets[j % sockets.len()].recv(recv).unwrap();
                assert_eq!(n, body.len());
                let seq = u64::from_le_bytes(recv[..8].try_into().unwrap()) as usize;
                assert!(seq >= packets && seq < packets + window);
                let bit = 1u64 << (seq - packets);
                assert_eq!(seen & bit, 0, "duplicate response");
                seen |= bit;
                assert!(recv[8..n].iter().all(|&b| b == 0x5a));
            }
            packets += window;
            if let Ok(rate) = std::env::var("NEXT_RATE_PPS") {
                let rate: f64 = rate.parse().unwrap();
                assert!(rate.is_finite() && rate > 0.0);
                let due = Duration::from_secs_f64(packets as f64 / rate);
                if let Some(delay) = due.checked_sub(start.elapsed()) {
                    thread::sleep(delay);
                }
            }
            if latency {
                assert!(
                    latencies.len() < latencies.capacity(),
                    "latency storage exhausted"
                );
                latencies.push(t.elapsed().as_secs_f64() * 1e6);
            }
        }
    } else {
        let mut pending = std::collections::HashMap::with_capacity(window * 2);
        let mut sequence = 0u64;
        for socket in sockets {
            socket.set_nonblocking(true).unwrap();
        }
        for j in 0..window {
            body[..8].copy_from_slice(&sequence.to_le_bytes());
            sockets[j % sockets.len()].send(body).unwrap();
            assert!(pending.insert(sequence, j % sockets.len()).is_none());
            sequence += 1;
        }
        let mut poll: Vec<_> = sockets
            .iter()
            .map(|s| libc::pollfd {
                fd: s.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            })
            .collect();
        let mut last = Instant::now();
        while !pending.is_empty() {
            let mut progress = false;
            for (flow, socket) in sockets.iter().enumerate() {
                for _ in 0..64 {
                    let n = match socket.recv(recv) {
                        Ok(n) => n,
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(e) => panic!("generator {e}"),
                    };
                    assert_eq!(n, body.len());
                    let seq = u64::from_le_bytes(recv[..8].try_into().unwrap());
                    assert_eq!(
                        pending.remove(&seq),
                        Some(flow),
                        "unknown, duplicate or wrong-flow response"
                    );
                    assert!(recv[8..n].iter().all(|&b| b == 0x5a));
                    packets += 1;
                    progress = true;
                    last = Instant::now();
                    if start.elapsed().as_secs_f64() < seconds {
                        body[..8].copy_from_slice(&sequence.to_le_bytes());
                        socket.send(body).unwrap();
                        assert!(pending.insert(sequence, flow).is_none());
                        sequence += 1;
                    }
                }
            }
            assert!(
                last.elapsed() < Duration::from_secs(5),
                "sliding window stalled"
            );
            if !progress {
                let n = unsafe { libc::poll(poll.as_mut_ptr(), poll.len() as _, 100) };
                assert!(
                    n >= 0
                        || std::io::Error::last_os_error().kind()
                            == std::io::ErrorKind::Interrupted
                );
            }
        }
        for socket in sockets {
            socket.set_nonblocking(false).unwrap();
        }
    }
    (packets, latencies)
}

fn client(peer_bin: &str, proto: &str, size: usize, window: usize, seconds: f64, flows: usize) {
    // Start the fixed peer before restricting this process and its future worker.
    let mut peer = OwnedChild(
        Command::new(peer_bin)
            .args(["peer", proto, &window.to_string()])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
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
    generator.0.stdin.as_mut().unwrap().write_all(b"W").unwrap();
    line.clear();
    load_out.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "NEXT_WARM");
    if let Ok(path) = std::env::var("NEXT_PROFILE_READY") {
        std::fs::write(path, std::process::id().to_string()).unwrap();
        wait_file("NEXT_PROFILE_GO");
    }
    peer.0.stdin.take().unwrap().write_all(b"G").unwrap();
    let c = cpu();
    let start = Instant::now();
    generator.0.stdin.take().unwrap().write_all(b"G").unwrap();
    line.clear();
    load_out.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "NEXT_DONE");
    let used = cpu() - c;
    let elapsed = start.elapsed().as_secs_f64();
    let mut rest = String::new();
    load_out.read_to_string(&mut rest).unwrap();
    assert!(generator.0.wait().unwrap().success());
    let before = con.snapshot();
    // Stop the controlled peer before closing the transport: teardown EOF is
    // not a measurement failure after all application replies were validated.
    unsafe {
        libc::kill(peer.0.id() as libc::pid_t, libc::SIGTERM);
    }
    con.disconnect();
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
        serde_json::json!({"effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build"),"role":"client","protocol":proto,"size":size,"window":window,"flows":flows,"seconds":elapsed,"client_cpu_seconds":used,"client_cpu_ns_per_byte":used*1e9/bytes as f64,"client_rss_kib":rss(),"load":metric,"peer":peer_metric,"cold_ms":cold,"counters":counters,"state":format!("{:?}",before.state),"kernel":std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap().trim(),"arch":std::env::consts::ARCH,"scope":"IPv4 UDP echo through actual Connection/TUN/WireGuard worker; peer and generator are fixed separate processes; their effective source identities are recorded. Process CPU includes synchronous system time, not all asynchronous kernel work. Not a TCP/HTTPS, NIC or Proton WAN acceptance test."}),
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

#[cfg(test)]
mod audit_tests {
    use super::*;
    #[test]
    fn actual_pinned_streamowned_fallback_and_corrected_peer_batch_differ() {
        for vectored in [false, true] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let endpoint = listener.local_addr().unwrap();
            let server = thread::spawn(move || {
                let (tcp, _) = listener.accept().unwrap();
                tcp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
                tcp.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
                let mut peer = PeerTls::new(tcp, vectored);
                let n = peer
                    .write_vectored(&[IoSlice::new(b"abc"), IoSlice::new(b"def")])
                    .unwrap();
                assert_eq!(n, if vectored { 6 } else { 3 });
                peer.inner.conn.send_close_notify();
                peer.flush().unwrap();
            });
            let config = rustls::ClientConfig::builder_with_provider(std::sync::Arc::new(
                rustls::crypto::ring::default_provider(),
            ))
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_root_certificates(common::roots())
            .with_no_client_auth();
            let tcp = TcpStream::connect(endpoint).unwrap();
            tcp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            let mut client = rustls::StreamOwned::new(
                rustls::ClientConnection::new(
                    std::sync::Arc::new(config),
                    "localhost".try_into().unwrap(),
                )
                .unwrap(),
                tcp,
            );
            let mut got = Vec::new();
            client.read_to_end(&mut got).unwrap();
            assert_eq!(
                &got,
                if vectored {
                    b"abcdef".as_slice()
                } else {
                    b"abc".as_slice()
                }
            );
            server.join().unwrap();
        }
    }
    #[test]
    fn record_meter_handles_fragmented_and_vectored_headers_without_payload_copies() {
        let mut meter = RecordSizes::default();
        let bytes = [23, 3, 3, 0, 3, 1, 2, 3, 23, 3, 3, 0, 1, 7];
        for b in bytes {
            meter.feed(&[b]);
        }
        assert_eq!(meter.encrypted, [2, 0, 0]);
        assert_eq!(meter.remaining, 0);
        assert_eq!(meter.used, 0);
    }
}
