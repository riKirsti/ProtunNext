#![cfg(windows)]
//! Real loopback Winsock/Mio/Noise/TLS, without loading a driver, creating an
//! adapter, changing routes/DNS, or using any external server or account.
mod common;
use base64::{Engine, engine::general_purpose::STANDARD};
use protun_next::{
    config::WireguardConfig,
    connection::State,
    packet::PacketPool,
    session::{Output, Session},
    stream::{Flush, FramedIo, Receive},
    transport::Transport,
    windows,
};
use std::{
    io::{Read, Write},
    net::{TcpListener, UdpSocket},
    sync::mpsc,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
trait Duplex: Read + Write {}
impl<T: Read + Write> Duplex for T {}

#[test]
fn windows_native_udp_tcp_tls_and_stealth_exchange_real_noise_without_a_tun() {
    for mode in 0..8 {
        let udp_mode = matches!(mode, 0 | 6);
        let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
        udp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let tcp = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = if udp_mode {
            udp.local_addr().unwrap()
        } else {
            tcp.local_addr().unwrap()
        };
        let (release, hold) = mpsc::channel();
        let reference = thread::spawn(move || {
            let mut crypto = Session::new(
                [9; 32],
                Session::public_key([7; 32]),
                None,
                windows::boottime().unwrap(),
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            )
            .unwrap();
            let mut input = [0; 65535];
            let mut output = [0; 65535];
            if udp_mode {
                let (n, remote) = udp.recv_from(&mut input).unwrap();
                let Output::Network(n) = crypto.decrypt(&input[..n], &mut output).unwrap() else {
                    panic!("WG response")
                };
                udp.send_to(&output[..n], remote).unwrap();
                hold.recv_timeout(Duration::from_secs(10)).unwrap();
            } else {
                let (socket, _) = tcp.accept().unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                socket
                    .set_write_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let socket: Box<dyn Duplex> = if (2..=5).contains(&mode) {
                    Box::new(rustls::StreamOwned::new(
                        rustls::ServerConnection::new(common::server()).unwrap(),
                        socket,
                    ))
                } else {
                    Box::new(socket)
                };
                let mut framed = FramedIo::new(socket, 8).unwrap();
                let mut pool = PacketPool::new(8, 65535).unwrap();
                let mut packet = pool.take().unwrap();
                while framed.receive(&mut packet).unwrap() != Receive::Packet {}
                let Output::Network(n) = crypto.decrypt(packet.data(), &mut output).unwrap() else {
                    panic!("WG response")
                };
                packet.writable()[..n].copy_from_slice(&output[..n]);
                packet.set_len(n).unwrap();
                framed
                    .queue(packet)
                    .unwrap_or_else(|_| panic!("queue handshake response"));
                while framed.flush(&mut pool).unwrap() != Flush::Empty {}
                hold.recv_timeout(Duration::from_secs(10)).unwrap();
            }
        });
        let config = WireguardConfig::parse(&format!(
            "[Interface]\nPrivateKey={}\nAddress=10.2.0.2/32\n[Peer]\nPublicKey={}\nEndpoint={endpoint}\nAllowedIPs=0.0.0.0/0\n",
            STANDARD.encode([7;32]), STANDARD.encode(Session::public_key([9;32])))).unwrap();
        let transport = match mode {
            0 | 6 => Transport::Udp,
            1 | 7 => Transport::Tcp,
            5 => Transport::UnbufferedTls(Box::new(
                protun_next::tls::unbuffered_client_with_roots("localhost", common::roots())
                    .unwrap(),
            )),
            2 => Transport::Tls(Box::new(
                protun_next::tls::client_with_roots("localhost", common::roots()).unwrap(),
            )),
            _ => Transport::Stealth(protun_next::stealth::SniStrategy::Explicit(
                "unrelated.test".into(),
            )),
        };
        let transport = if mode >= 4 {
            transport.with_performance(protun_next::performance::Options {
                udp_backend: protun_next::performance::UdpBackend::Batch,
                compact_pool: true,
                in_place_crypto: cfg!(feature = "in-place-crypto"),
                unbuffered_tls: mode == 4,
                ..Default::default()
            })
        } else {
            transport
        };
        let mut worker = windows::start_without_tun(config, transport).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let state = worker.snapshot();
            if state.state == State::TunnelEstablished {
                break;
            }
            assert!(
                !matches!(state.state, State::Failed(_)),
                "mode {mode}: {:?}",
                state.state
            );
            assert!(
                Instant::now() < deadline,
                "mode {mode}: native handshake timeout"
            );
            worker.wait_after(state.sequence, Duration::from_millis(100));
        }
        let request = worker.request_stats();
        let sample = worker.wait_for_stats(request, Duration::from_secs(2));
        assert!(sample.statistics.unwrap().handshake_age.is_some());
        worker.disconnect_and_wait().unwrap();
        release.send(()).unwrap();
        reference.join().unwrap();
    }
}
