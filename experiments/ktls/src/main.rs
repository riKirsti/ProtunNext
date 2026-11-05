//! Fixed small kTLS interoperability cases. No account, router or speed claim.
#[path = "../../../tests/common/mod.rs"]
mod common;
mod ktls;
mod speed;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};
fn run(tls13: bool) -> std::io::Result<serde_json::Value> {
    let version = if tls13 {
        &rustls::version::TLS13
    } else {
        &rustls::version::TLS12
    };
    let mut config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_protocol_versions(&[version])
    .unwrap()
    .with_root_certificates(common::roots())
    .with_no_client_auth();
    config.enable_secret_extraction = true;
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let peer = thread::spawn(move || -> std::io::Result<()> {
        let (tcp, _) = listener.accept()?;
        tcp.set_read_timeout(Some(Duration::from_secs(4)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(4)))?;
        let mut stream = rustls::StreamOwned::new(
            rustls::ServerConnection::new(common::server()).unwrap(),
            tcp,
        );
        for turn in 0..5 {
            let mut length = [0; 4];
            stream.read_exact(&mut length)?;
            let n = u32::from_be_bytes(length) as usize;
            if n > 65535 {
                return Err(std::io::ErrorKind::InvalidData.into());
            }
            let mut body = vec![0; n];
            stream.read_exact(&mut body)?;
            assert!(
                body.iter()
                    .enumerate()
                    .all(|(i, b)| *b == ((i + turn) % 251) as u8)
            );
            if tls13 && turn == 2 {
                stream.conn.refresh_traffic_keys().unwrap();
            }
            stream.write_all(&length)?;
            stream.write_all(&body)?;
            stream.flush()?;
        }
        stream.conn.send_close_notify();
        stream.flush()?;
        assert_eq!(stream.read(&mut [0; 1])?, 0);
        Ok(())
    });
    let result = (|| {
        let tcp = TcpStream::connect(address)?;
        tcp.set_nodelay(true)?;
        tcp.set_read_timeout(Some(Duration::from_secs(4)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(4)))?;
        let mut client = ktls::Ktls::connect(tcp, Arc::new(config))?;
        for (turn, n) in [0usize, 1, 1280, 16384, 65535].into_iter().enumerate() {
            if tls13 && turn == 3 {
                client.key_update()?;
            }
            let body: Vec<_> = (0..n).map(|i| ((i + turn) % 251) as u8).collect();
            client.write_all(&(n as u32).to_be_bytes())?;
            client.write_all(&body)?;
            client.flush()?;
            let mut length = [0; 4];
            client.read_exact(&mut length)?;
            assert_eq!(u32::from_be_bytes(length) as usize, n);
            let mut got = vec![0; n];
            client.read_exact(&mut got)?;
            assert_eq!(got, body);
        }
        let mut end = Vec::new();
        client.read_to_end(&mut end)?;
        assert!(end.is_empty());
        client.close_notify()?;
        client.flush()?;
        if tls13 {
            assert!(client.rx_updates >= 1 && client.tx_updates >= 1);
            assert!(client.tickets >= 1);
        }
        Ok(
            serde_json::json!({"tls13":tls13,"suite":client.suite(),"rx_key_updates":client.rx_updates,"tx_key_updates":client.tx_updates,"tickets":client.tickets,"status":"passed","scope":"bounded blocking loopback kTLS interop, not production worker or performance acceptance"}),
        )
    })();
    let peer_result = peer.join().unwrap();
    match result {
        Ok(v) => {
            peer_result?;
            Ok(v)
        }
        Err(e) => Err(e),
    }
}
fn main() {
    if speed::run(&std::env::args().collect::<Vec<_>>()) {
        return;
    }
    let tls13 = std::env::args().nth(1).as_deref() != Some("tls12");
    if !tls13 {
        println!(
            "KTLS_JSON {}",
            serde_json::json!({"tls13":false,"status":"userspace_required","scope":"TLS 1.2 kTLS handoff is outside this prototype; production rustls TLS 1.2 remains unchanged"})
        );
        return;
    }
    match run(tls13) {
        Ok(v) => println!("KTLS_JSON {}", v),
        Err(e) => {
            let rekey = e
                .get_ref()
                .and_then(|e| e.downcast_ref::<ktls::RekeyUnavailable>());
            println!(
                "KTLS_JSON {}",
                serde_json::json!({"tls13":tls13,"status":if rekey.is_some(){"unsupported_key_update"}else{"failed"},"errno":rekey.map(|e|e.0).or(e.raw_os_error()),"error":e.to_string()})
            );
            std::process::exit(1);
        }
    }
}
