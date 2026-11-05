//! Bounded outer-TLS opportunity screen, not the ProtunNext packet worker.
//! Both backends use the same independent Rustls server and public test keys.
use crate::{common, ktls};
#[path = "../../../src/tls_unbuffered.rs"]
mod unbuffered;
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    process::{Command, Stdio},
    sync::Arc,
    time::{Duration, Instant},
};
fn cpu() -> f64 {
    let mut t = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut t) },
        0
    );
    t.tv_sec as f64 + t.tv_nsec as f64 * 1e-9
}
fn pin(key: &str) {
    let Some(text) = std::env::var_os(key) else {
        return;
    };
    let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
    for cpu in text.to_str().unwrap().split(',') {
        unsafe { libc::CPU_SET(cpu.parse().unwrap(), &mut set) }
    }
    assert_eq!(
        unsafe { libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set) },
        0
    );
}
enum Client {
    User(unbuffered::UnbufferedTlsIo<TcpStream>),
    Kernel(ktls::Ktls),
}
impl Read for Client {
    fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::User(s) => s.read(b),
            Self::Kernel(s) => s.read(b),
        }
    }
}
impl Write for Client {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        match self {
            Self::User(s) => s.write(b),
            Self::Kernel(s) => s.write(b),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::User(s) => s.flush(),
            Self::Kernel(s) => s.flush(),
        }
    }
}
impl Client {
    fn update(&mut self) -> io::Result<()> {
        match self {
            Self::User(s) => s.refresh_traffic_keys(),
            Self::Kernel(s) => s.key_update(),
        }
    }
    fn close(&mut self) -> io::Result<()> {
        match self {
            Self::User(s) => s.queue_close_notify(),
            Self::Kernel(s) => s.close_notify(),
        }
    }
}
fn peer(size: usize) -> io::Result<()> {
    pin("NEXT_TLS_PEER_CPUS");
    let listener = TcpListener::bind("127.0.0.1:0")?;
    println!("TLS_READY {}", listener.local_addr()?);
    io::stdout().flush()?;
    let (tcp, _) = listener.accept()?;
    tcp.set_nodelay(true)?;
    tcp.set_read_timeout(Some(Duration::from_secs(10)))?;
    tcp.set_write_timeout(Some(Duration::from_secs(10)))?;
    let mut s = rustls::StreamOwned::new(
        rustls::ServerConnection::new(common::server()).unwrap(),
        tcp,
    );
    let mut b = vec![0; size];
    let start = cpu();
    let mut packets = 0u64;
    loop {
        match s.read_exact(&mut b) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
        if b.iter().any(|&b| b != 0x5a) {
            return Err(io::ErrorKind::InvalidData.into());
        }
        s.write_all(&b)?;
        s.flush()?;
        packets += 1;
        if packets % 100_000 == 0 {
            s.conn.refresh_traffic_keys().map_err(io::Error::other)?;
        }
    }
    s.conn.send_close_notify();
    s.flush()?;
    println!(
        "TLS_PEER {}",
        serde_json::json!({"cpu_seconds":cpu()-start,"packets":packets})
    );
    Ok(())
}
fn client(backend: &str, size: usize, window: usize) -> io::Result<()> {
    assert!([1280, 16384, 65536].contains(&size));
    assert!(window == 1 || (window == 8 && size <= 16384));
    let mut peer = Command::new(std::env::current_exe()?)
        .args(["bench-peer", &size.to_string()])
        .stdout(Stdio::piped())
        .spawn()?;
    let result = (|| {
        let mut out = BufReader::new(peer.stdout.take().unwrap());
        let mut line = String::new();
        out.read_line(&mut line)?;
        let endpoint = line
            .trim()
            .strip_prefix("TLS_READY ")
            .ok_or(io::ErrorKind::InvalidData)?;
        pin("NEXT_TLS_CLIENT_CPUS");
        let mut config = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_root_certificates(common::roots())
        .with_no_client_auth();
        config.enable_secret_extraction = true;
        let tcp = TcpStream::connect(endpoint)?;
        tcp.set_nodelay(true)?;
        tcp.set_read_timeout(Some(Duration::from_secs(10)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(10)))?;
        let mut s = if backend == "ktls" {
            Client::Kernel(ktls::Ktls::connect(tcp, Arc::new(config))?)
        } else {
            assert_eq!(backend, "unbuffered");
            Client::User(unbuffered::UnbufferedTlsIo::new(
                tcp,
                rustls::client::UnbufferedClientConnection::new(
                    Arc::new(config),
                    "localhost".try_into().unwrap(),
                )
                .unwrap(),
            ))
        };
        let body = vec![0x5a; size];
        let mut got = vec![0; size];
        // Authenticate and exchange one complete message before timing.
        s.write_all(&body)?;
        s.flush()?;
        s.read_exact(&mut got)?;
        assert_eq!(got, body);
        let start = Instant::now();
        let c = cpu();
        let mut packets = 0u64;
        // Hard fixture caps keep this incomplete adapter far below AES-GCM
        // record limits; this is NOT long-lived connection limit enforcement.
        while start.elapsed() < Duration::from_secs(3)
            && packets < 1_000_000
            && packets * (size as u64) < 4 * 1024 * 1024 * 1024
        {
            for _ in 0..window {
                s.write_all(&body)?;
            }
            s.flush()?;
            for _ in 0..window {
                s.read_exact(&mut got)?;
                if got != body {
                    return Err(io::ErrorKind::InvalidData.into());
                }
            }
            packets += window as u64;
            if packets % 100_000 == 0 {
                s.update()?;
            }
        }
        let used = cpu() - c;
        let seconds = start.elapsed().as_secs_f64();
        let bytes = packets * size as u64;
        s.close()?;
        s.flush()?;
        assert_eq!(s.read(&mut [0; 1])?, 0);
        let mut details = String::new();
        out.read_to_string(&mut details)?;
        assert!(peer.wait()?.success());
        println!(
            "TLS_SPEED {}",
            serde_json::json!({"backend":backend,"size":size,"window":window,
            "bytes":bytes,"seconds":seconds,"Mbps":bytes as f64*8.0/seconds/1e6,
            "client_cpu_seconds":used,"client_cpu_ns_per_byte":used*1e9/bytes as f64,
            "kernel":std::fs::read_to_string("/proc/sys/kernel/osrelease")?.trim(),
            "peer":details.trim(),"scope":"bounded blocking outer TLS component, separate fixed Rustls peer; not the packet worker, WireGuard or Proton WAN"})
        );
        Ok(())
    })();
    // Only the child created here is terminated on a failed fixture.
    if result.is_err() {
        let _ = peer.kill();
        let _ = peer.wait();
    }
    result
}
pub fn run(args: &[String]) -> bool {
    let result = match args.get(1).map(String::as_str) {
        Some("bench-peer") => peer(args[2].parse().unwrap()),
        Some("bench-client") => {
            client(&args[2], args[3].parse().unwrap(), args[4].parse().unwrap())
        }
        _ => return false,
    };
    if let Err(e) = result {
        eprintln!("TLS_SPEED_FAILED {e:?}");
        std::process::exit(1)
    }
    true
}
