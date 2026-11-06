//! Kernel TCP download fixture, injected identically into isolated source archives.
//! Outer sockets remain in the client's disposable namespace. The peer unshares
//! AFTER creating them, so its inner TCP server and TUN live in a second stack.
//! No veth, forwarding, persistent namespace, host route or sysctl is required.
use super::*;
use std::sync::{Arc, Barrier};

const BLOCK: usize = 65536;
const WARM_BLOCKS: u64 = 64;

pub(super) fn dispatch(args: &[String]) -> bool {
    match args.get(1).map(String::as_str) {
        Some("download-client") => client(
            &args[2],
            &args[3],
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
            &args[6],
            &args[7],
        ),
        Some("download-peer") => peer(
            &args[2],
            args[3].parse().unwrap(),
            args[4].parse().unwrap(),
            &args[5],
            &args[6],
        ),
        Some("download-load") => load(
            args[2].parse().unwrap(),
            args[3].parse().unwrap(),
            &args[4],
            &args[5],
        ),
        _ => return false,
    }
    true
}

fn addresses(family: &str, peer: bool) -> (&'static str, &'static str, &'static str) {
    match (family, peer) {
        ("v4", false) => ("10.2.0.2/32", "10.2.0.3/32", "10.2.0.3:9000"),
        ("v4", true) => ("10.2.0.3/32", "10.2.0.2/32", "10.2.0.3:9000"),
        ("v6", false) => ("fd42::2/128", "fd42::3/128", "[fd42::3]:9000"),
        ("v6", true) => ("fd42::3/128", "fd42::2/128", "[fd42::3]:9000"),
        _ => panic!("IP family"),
    }
}

fn tun(family: &str, peer: bool) -> File {
    let tun = File::options()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .unwrap();
    let mut req = [0u8; 40];
    req[..4].copy_from_slice(b"pndl");
    req[16..18].copy_from_slice(&((libc::IFF_TUN | libc::IFF_NO_PI) as i16).to_ne_bytes());
    // SAFETY: live TUN fd and correctly sized mutable ifreq storage.
    assert_eq!(
        unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, req.as_mut_ptr()) },
        0
    );
    let (local, remote, _) = addresses(family, peer);
    ip(&["link", "set", "lo", "up"]);
    // BusyBox ip does not implement nodad. TUN is IFF_NOARP, for which the
    // Linux IPv6 stack already skips duplicate-address detection.
    ip(&["addr", "add", local, "dev", "pndl"]);
    ip(&["link", "set", "pndl", "mtu", "1280", "up"]);
    ip(&["route", "add", remote, "dev", "pndl"]);
    if family == "v6" {
        // Address activation is asynchronous even when NOARP skips DAD.
        let address = format!("[{}]:0", local.split('/').next().unwrap());
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            match TcpListener::bind(&address) {
                Ok(_) => break,
                Err(e) if e.kind() == std::io::ErrorKind::AddrNotAvailable => {
                    assert!(Instant::now() < deadline, "IPv6 address did not activate");
                    thread::sleep(Duration::from_millis(1));
                }
                Err(e) => panic!("IPv6 readiness {e}"),
            }
        }
    }
    tun
}

fn namespace() -> String {
    std::fs::read_link("/proc/self/ns/net")
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

fn template() -> Vec<u8> {
    (0..BLOCK)
        .map(|n| ((n * 31 + n / 251) % 251) as u8)
        .collect()
}

fn send_body(io: &mut dyn Duplex, blocks: u64, flow: u64) {
    let mut body = template();
    for sequence in 0..blocks {
        body[..8].copy_from_slice(&sequence.to_be_bytes());
        body[8..16].copy_from_slice(&flow.to_be_bytes());
        io.write_all(&body).unwrap();
    }
    io.flush().unwrap();
}

fn receive_body(io: &mut dyn Duplex, blocks: u64, flow: u64) {
    let expected = template();
    let mut body = vec![0; BLOCK];
    for sequence in 0..blocks {
        io.read_exact(&mut body).unwrap();
        assert_eq!(&body[..8], sequence.to_be_bytes());
        assert_eq!(&body[8..16], flow.to_be_bytes());
        assert_eq!(&body[16..], &expected[16..], "download payload mismatch");
    }
}

fn request(io: &mut dyn Duplex, flow: usize) {
    // Deliberately small HTTP/1.1 fixture; no external HTTP implementation or DNS.
    write!(
        io,
        "GET /{flow} HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n"
    )
    .unwrap();
    io.flush().unwrap();
}

fn header(io: &mut dyn Duplex) -> String {
    let mut value = Vec::new();
    while !value.ends_with(b"\r\n\r\n") {
        assert!(value.len() < 4096, "oversized HTTP header");
        let mut byte = [0];
        io.read_exact(&mut byte).unwrap();
        value.push(byte[0]);
    }
    String::from_utf8(value).unwrap()
}

struct InnerTls(rustls::StreamOwned<rustls::ClientConnection, TcpStream>);
impl Read for InnerTls {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(b)
    }
}
impl Write for InnerTls {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0.write(b)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
impl Duplex for InnerTls {
    fn fd(&self) -> i32 {
        self.0.sock.as_raw_fd()
    }
}

fn load(flows: usize, blocks: u64, family: &str, application: &str) {
    pin("NEXT_LOAD_CPUS");
    let barrier = Arc::new(Barrier::new(flows + 1));
    let mut workers = Vec::new();
    for flow in 0..flows {
        let barrier = barrier.clone();
        let endpoint = addresses(family, false).2;
        let https = application == "https";
        workers.push(thread::spawn(move || {
            let socket =
                TcpStream::connect_timeout(&endpoint.parse().unwrap(), Duration::from_secs(15))
                    .unwrap();
            socket.set_nodelay(true).unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(30)))
                .unwrap();
            socket
                .set_write_timeout(Some(Duration::from_secs(30)))
                .unwrap();
            let mut io: Box<dyn Duplex> = if https {
                let config = rustls::ClientConfig::builder_with_provider(Arc::new(
                    rustls::crypto::ring::default_provider(),
                ))
                .with_safe_default_protocol_versions()
                .unwrap()
                .with_root_certificates(common::roots())
                .with_no_client_auth();
                Box::new(InnerTls(rustls::StreamOwned::new(
                    rustls::ClientConnection::new(
                        Arc::new(config),
                        "localhost".try_into().unwrap(),
                    )
                    .unwrap(),
                    socket,
                )))
            } else {
                Box::new(socket)
            };
            request(&mut *io, flow);
            let response = header(&mut *io);
            assert_eq!(
                response,
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
                    WARM_BLOCKS * BLOCK as u64
                )
            );
            receive_body(&mut *io, WARM_BLOCKS, flow as u64);
            barrier.wait();
            barrier.wait();
            // Server sends measured bytes only after the second HTTP request, preventing
            // prefilled receive buffers from inflating the measured download.
            request(&mut *io, flow);
            assert_eq!(
                header(&mut *io),
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    blocks * BLOCK as u64
                )
            );
            receive_body(&mut *io, blocks, flow as u64);
            barrier.wait();
            // Keep the verified connection open until the client timer stops.
            barrier.wait();
        }));
    }
    barrier.wait();
    println!("NEXT_WARM");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_exact(&mut [0]).unwrap();
    let cpu_start = cpu();
    let start = Instant::now();
    barrier.wait();
    barrier.wait();
    let elapsed = start.elapsed().as_secs_f64();
    let used = cpu() - cpu_start;
    println!("NEXT_DONE");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_exact(&mut [0]).unwrap();
    barrier.wait();
    for worker in workers {
        worker.join().unwrap();
    }
    emit(
        serde_json::json!({"role":"load", "effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build"), "bytes":blocks*BLOCK as u64*flows as u64,"seconds":elapsed,"cpu_seconds":used,"rss_kib":rss(),"verified":true,"namespace":namespace()}),
    );
}

fn peer(proto: &str, flows: usize, blocks: u64, family: &str, application: &str) {
    pin("NEXT_PEER_CPUS");
    // Create outer sockets in the parent stack BEFORE creating the inner stack.
    let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
    let tcp = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = if proto == "udp" {
        udp.local_addr().unwrap()
    } else {
        tcp.local_addr().unwrap()
    };
    let outer_namespace = namespace();
    // SAFETY: single-threaded child, disposable network namespace, checked result.
    assert_eq!(unsafe { libc::unshare(libc::CLONE_NEWNET) }, 0);
    let mut device = tun(family, true);
    let listener = TcpListener::bind(addresses(family, true).2).unwrap();
    let https = application == "https";
    thread::spawn(move || {
        let mut workers = Vec::new();
        for _ in 0..flows {
            let (socket, _) = listener.accept().unwrap();
            workers.push(thread::spawn(move || {
                socket.set_nodelay(true).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(30)))
                    .unwrap();
                socket
                    .set_write_timeout(Some(Duration::from_secs(30)))
                    .unwrap();
                let mut io: Box<dyn Duplex> = if https {
                    Box::new(PeerTls::new(socket, true))
                } else {
                    Box::new(socket)
                };
                let request = header(&mut *io);
                let flow: u64 = request
                    .strip_prefix("GET /")
                    .unwrap()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                assert!(flow < flows as u64);
                write!(
                    io,
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
                    WARM_BLOCKS * BLOCK as u64
                )
                .unwrap();
                send_body(&mut *io, WARM_BLOCKS, flow);
                assert_eq!(header(&mut *io), request);
                write!(
                    io,
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    blocks * BLOCK as u64
                )
                .unwrap();
                send_body(&mut *io, blocks, flow);
            }));
        }
        for worker in workers {
            worker.join().unwrap();
        }
    });
    // SAFETY: simple handlers only update lock-free atomics.
    unsafe {
        libc::signal(libc::SIGTERM, stop as *const () as libc::sighandler_t);
        libc::signal(libc::SIGUSR1, drain as *const () as libc::sighandler_t);
    }
    println!("NEXT_READY {endpoint}");
    std::io::stdout().flush().unwrap();
    let mut network = if proto == "udp" {
        udp.set_read_timeout(Some(Duration::from_secs(15))).unwrap();
        let mut first = vec![0; 65535];
        let (n, from) = udp.recv_from(&mut first).unwrap();
        first.truncate(n);
        udp.connect(from).unwrap();
        udp.set_nonblocking(true).unwrap();
        let tx = crate::linux_batch::Batch::new(&udp, true);
        let rx = crate::linux_batch::Batch::new(&udp, true);
        Peer::Udp {
            socket: udp,
            tx,
            rx,
            first: Some(first),
        }
    } else {
        let (socket, _) = tcp.accept().unwrap();
        socket.set_nodelay(true).unwrap();
        socket.set_nonblocking(true).unwrap();
        let io: Box<dyn Duplex> = if proto == "stealth" {
            Box::new(PeerTls::new(socket, true))
        } else {
            Box::new(socket)
        };
        Peer::Stream(FramedIo::new(io, 64).unwrap())
    };
    // SAFETY: live TUN descriptor, preserve flags and enable nonblocking I/O.
    let flags = unsafe { libc::fcntl(device.as_raw_fd(), libc::F_GETFL) };
    assert!(flags >= 0);
    assert_eq!(
        unsafe { libc::fcntl(device.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) },
        0
    );
    let mut session = Session::new(
        [9; 32],
        Session::public_key([7; 32]),
        None,
        linux::boottime().unwrap(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
    )
    .unwrap();
    let mut pool = PacketPool::new(132, 65535).unwrap();
    let mut packet = pool.take().unwrap();
    let mut queue = PacketQueue::new(64).unwrap();
    let mut plain = vec![0; 65535];
    let mut wire = vec![0; 65535];
    let mut pending_tun: Option<usize> = None;
    let mut timing = None;
    let mut measured = None;
    let mut timer = Instant::now();
    let mut handshakes = 0;
    thread::spawn(|| {
        let mut input = std::io::stdin();
        let mut marker = [0];
        while input.read_exact(&mut marker).is_ok() {
            if marker[0] == b'G' {
                PEER_CPU_START.store((cpu() * 1e9) as u64, Ordering::Release);
                PEER_WALL_START.store(monotonic_ns(), Ordering::Release);
            } else {
                DRAIN.store(true, Ordering::Release);
            }
        }
    });
    while !STOP.load(Ordering::Relaxed) {
        if timing.is_none() && PEER_WALL_START.load(Ordering::Acquire) > 0 {
            timing = Some((
                PEER_CPU_START.load(Ordering::Acquire),
                PEER_WALL_START.load(Ordering::Acquire),
            ));
        }
        if DRAIN.load(Ordering::Acquire) && measured.is_none() {
            measured = timing.map(|(c, t)| {
                (
                    (cpu() * 1e9 - c as f64) / 1e9,
                    (monotonic_ns() - t) as f64 / 1e9,
                )
            });
            println!("NEXT_DRAIN");
            std::io::stdout().flush().unwrap();
        }
        if timer.elapsed() >= Duration::from_secs(1)
            && queue.len()
                + match &network {
                    Peer::Stream(s) => s.pending(),
                    _ => 0,
                }
                < 60
        {
            if let Output::Network(n) = session
                .advance(linux::boottime().unwrap(), &mut wire)
                .unwrap()
            {
                network.queue(&wire[..n], &mut queue, &mut pool);
            }
            timer = Instant::now();
        }
        let mut progress = false;
        for _ in 0..32 {
            if let Some(n) = pending_tun {
                match device.write(&plain[..n]) {
                    Ok(written) => {
                        assert_eq!(written, n);
                        pending_tun = None;
                        progress = true;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(e) => panic!("peer TUN write {e}"),
                }
            }
            if queue.len()
                + match &network {
                    Peer::Stream(s) => s.pending(),
                    _ => 0,
                }
                >= 60
            {
                break;
            }
            match network.receive(&mut packet, &mut pool) {
                Receive::WouldBlock => break,
                Receive::Yield => continue,
                Receive::Eof => {
                    assert!(DRAIN.load(Ordering::Acquire));
                    STOP.store(true, Ordering::Relaxed);
                    break;
                }
                Receive::Packet => {}
            }
            progress = true;
            match session.decrypt(packet.data(), &mut plain).unwrap() {
                Output::Network(n) => {
                    if n == 92 {
                        handshakes += 1;
                    }
                    network.queue(&plain[..n], &mut queue, &mut pool);
                }
                Output::Tunnel(n) => pending_tun = Some(n),
                Output::None => {}
            }
        }
        for _ in 0..32 {
            if queue.len()
                + match &network {
                    Peer::Stream(s) => s.pending(),
                    _ => 0,
                }
                >= 60
            {
                break;
            }
            match device.read(&mut wire[..65000]) {
                Ok(n) => {
                    assert!(n > 0);
                    let Output::Network(n) =
                        session.encrypt(&wire[..n], packet.writable()).unwrap()
                    else {
                        panic!("peer encrypt")
                    };
                    network.queue(&packet.writable()[..n], &mut queue, &mut pool);
                    progress = true;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => panic!("peer TUN read {e}"),
            }
        }
        let pending = network.flush(&mut queue, &mut pool);
        if !progress || pending {
            let outer = match &network {
                Peer::Udp { socket, .. } => socket.as_raw_fd(),
                Peer::Stream(s) => s.io().fd(),
            };
            let mut polls = [
                libc::pollfd {
                    fd: outer,
                    events: libc::POLLIN | if pending { libc::POLLOUT } else { 0 },
                    revents: 0,
                },
                libc::pollfd {
                    fd: device.as_raw_fd(),
                    events: libc::POLLIN
                        | if pending_tun.is_some() {
                            libc::POLLOUT
                        } else {
                            0
                        },
                    revents: 0,
                },
            ];
            // SAFETY: two initialized pollfd entries, live owned descriptors.
            let result = unsafe { libc::poll(polls.as_mut_ptr(), 2, 10) };
            assert!(
                result >= 0
                    || std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
            );
        }
    }
    let (used, seconds) = measured.expect("peer timing not stopped");
    let tls = match &network {
        Peer::Stream(s) => s.io().metadata(),
        _ => None,
    };
    emit(
        serde_json::json!({"role":"peer","effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build"),"cpu_seconds":used,"seconds":seconds,"rss_kib":rss(),"outer_namespace":outer_namespace,"namespace":namespace(),"handshakes":handshakes,"tls":tls}),
    );
}

fn client(binary: &str, proto: &str, flows: usize, blocks: u64, family: &str, application: &str) {
    assert!((1..=8).contains(&flows) && blocks > 0 && blocks <= 65536);
    assert!(matches!(application, "http" | "https"));
    assert_ne!(
        namespace(),
        std::fs::read_link("/proc/1/ns/net")
            .unwrap()
            .to_string_lossy(),
        "run in an unshared namespace"
    );
    let mut peer = OwnedChild(
        Command::new(binary)
            .args([
                "download-peer",
                proto,
                &flows.to_string(),
                &blocks.to_string(),
                family,
                application,
            ])
            .stdin(Stdio::piped())
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
        .expect("peer readiness");
    pin("NEXT_CLIENT_CPUS");
    let device = tun(family, false);
    let (local, remote, _) = addresses(family, false);
    let config=WireguardConfig::parse(&format!("[Interface]\nPrivateKey={}\nAddress={local}\nMTU=1280\n[Peer]\nPublicKey={}\nEndpoint={endpoint}\nAllowedIPs={remote}\n",STANDARD.encode([7;32]),STANDARD.encode(Session::public_key([9;32])))).unwrap();
    let transport = match proto {
        "udp" => linux::Transport::Udp,
        "tcp" => linux::Transport::Tcp,
        "stealth" => {
            linux::Transport::Stealth(crate::stealth::SniStrategy::Explicit("cover.test".into()))
        }
        _ => panic!("protocol"),
    };
    let mut connection = Connection::start(device.as_fd(), config, "lo".into(), transport).unwrap();
    let mut generator = OwnedChild(
        Command::new(binary)
            .args([
                "download-load",
                &flows.to_string(),
                &blocks.to_string(),
                family,
                application,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let mut load_out = BufReader::new(generator.0.stdout.take().unwrap());
    line.clear();
    load_out.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "NEXT_WARM");
    if let Ok(path) = std::env::var("NEXT_PROFILE_READY") {
        std::fs::write(path, std::process::id().to_string()).unwrap();
        wait_file("NEXT_PROFILE_GO");
    }
    let before = connection.wait_for_stats(connection.request_stats(), Duration::from_secs(2));
    peer.0.stdin.as_mut().unwrap().write_all(b"G").unwrap();
    let start = Instant::now();
    let cpu_start = cpu();
    generator.0.stdin.as_mut().unwrap().write_all(b"G").unwrap();
    line.clear();
    load_out.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "NEXT_DONE");
    let used = cpu() - cpu_start;
    let seconds = start.elapsed().as_secs_f64();
    peer.0.stdin.take().unwrap().write_all(b"D").unwrap();
    line.clear();
    peer_out.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "NEXT_DRAIN");
    let after = connection.wait_for_stats(connection.request_stats(), Duration::from_secs(2));
    generator.0.stdin.take().unwrap().write_all(b"D").unwrap();
    let mut load_text = String::new();
    load_out.read_to_string(&mut load_text).unwrap();
    assert!(generator.0.wait().unwrap().success());
    connection.disconnect();
    let counters = connection.disconnect_and_wait().unwrap();
    // SAFETY: signal only our owned, still-live child; socket stays open until client disconnects.
    unsafe {
        libc::kill(peer.0.id() as libc::pid_t, libc::SIGTERM);
    }
    let mut peer_text = String::new();
    peer_out.read_to_string(&mut peer_text).unwrap();
    assert!(peer.0.wait().unwrap().success());
    let metric = |text: &str| -> serde_json::Value {
        serde_json::from_str(
            text.lines()
                .find_map(|s| s.strip_prefix("NEXT_JSON "))
                .unwrap(),
        )
        .unwrap()
    };
    emit(
        serde_json::json!({"role":"download-client","effective_sources_sha256":option_env!("NEXT_BUILD_ID").unwrap_or("unstamped-local-build"),"protocol":proto,"flows":flows,"blocks_per_flow":blocks,"family":family,"application":application,"seconds":seconds,"client_cpu_seconds":used,"client_rss_kib":rss(),"load":metric(&load_text),"peer":metric(&peer_text),"before":before,"after":after,"counters":counters,"namespace":namespace(),"kernel":std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap().trim(),"arch":std::env::consts::ARCH,"scope":"Kernel TCP HTTP/HTTPS, two inner network stacks, production Connection/TUN worker; outer sockets on loopback. No physical NIC, RTT/loss or Proton WAN acceptance. CPU excludes asynchronous kernel work; RSS excludes kernel/auxiliary memory."}),
    );
    assert_eq!(
        counters.invalid_packets, 0,
        "rejected packets; full metrics retained above"
    );
    assert_eq!(counters.outer_eof, 0);
    assert_eq!(counters.outer_io_errors, 0);
}
