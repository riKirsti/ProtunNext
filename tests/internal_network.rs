#![cfg(feature = "local-agent")]
use futures::{AsyncReadExt, AsyncWriteExt};
use proton_boringtun::proton_os_interface::{lock::strict::Strict, rand::SeedableRng};
use proton_vpn_netstack::{NetworkConfig, NetworkConfigV4, NetworkStack, error::DeviceReadError};
use protun_next::api::internal::InternalNetwork;
use rand_chacha::ChaCha20Rng;
use std::{
    sync::Arc,
    task::Wake,
    time::{Duration, Instant},
};
struct Noop;
impl Wake for Noop {
    fn wake(self: Arc<Self>) {}
}

#[test]
fn private_tcp_and_dns_datagrams_cross_real_ip_stack_with_port_translation() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    runtime.block_on(async {
        let (client, mut wire) = InternalNetwork::new(Duration::ZERO, Noop).unwrap();
        let stack = NetworkStack::builder()
            .with_time(0)
            .with_config(NetworkConfig::V4(NetworkConfigV4::new(
                "10.2.0.1".parse().unwrap(),
                24,
                None,
            )))
            .with_rng(ChaCha20Rng::from_seed([43; 32]))
            .with_mtu(1420)
            .with_tcp_rx_buffer_size(65536)
            .with_tcp_tx_buffer_size(65536)
            .build::<Strict>();
        let (server, mut peer) = stack.into_sides();
        let mut listener = server
            .tcp_bind("10.2.0.1:65433".parse::<std::net::SocketAddr>().unwrap())
            .unwrap();
        let dns = server
            .udp_bind("10.2.0.1:53".parse::<std::net::SocketAddr>().unwrap())
            .unwrap();
        let exchange = async {
            let client_tcp = async {
                let mut stream = client
                    .connect("10.2.0.1:65433".parse().unwrap())
                    .await
                    .unwrap();
                stream.write_all(b"private control TCP").await.unwrap();
                stream.flush().await.unwrap();
                let mut reply = [0; 19];
                stream.read_exact(&mut reply).await.unwrap();
                assert_eq!(&reply, b"private control TCP");
                tokio::time::sleep(Duration::from_millis(60)).await;
            };
            let server_tcp = async {
                let (mut stream, remote) = listener.accept().await.unwrap();
                assert!(
                    remote.port() < 1024,
                    "wire source must use a privileged control port"
                );
                let mut bytes = [0; 19];
                stream.read_exact(&mut bytes).await.unwrap();
                stream.write_all(&bytes).await.unwrap();
                stream.flush().await.unwrap();
                // Retain the socket until the client receives the buffered reply.
                tokio::time::sleep(Duration::from_millis(40)).await;
            };
            let client_udp = async {
                let socket = client.udp("10.2.0.1:53".parse().unwrap()).unwrap();
                socket.send(b"DNS request").await.unwrap();
                let mut reply = [0; 32];
                let n = socket.recv(&mut reply).await.unwrap();
                assert_eq!(&reply[..n], b"DNS response");
            };
            let server_udp = async {
                let mut bytes = [0; 32];
                let (n, from) = dns.recv_from(&mut bytes).await.unwrap();
                assert_eq!(&bytes[..n], b"DNS request");
                assert!(from.port() < 1024);
                dns.sendto(b"DNS response", from.into()).await.unwrap();
            };
            futures::join!(client_tcp, server_tcp, client_udp, server_udp);
        };
        let pump = async {
            let started = Instant::now();
            let mut packet = [0; 65535];
            loop {
                let now = started.elapsed();
                for _ in 0..64 {
                    let Some(n) = wire.read_packet(&mut packet, now).unwrap() else {
                        break;
                    };
                    peer.write_packet(&packet[..n], now.as_micros() as i64)
                        .unwrap();
                }
                for _ in 0..64 {
                    match peer.read_packet(&mut packet, now.as_micros() as i64) {
                        Ok(n) => {
                            assert!(InternalNetwork::might_receive(&packet[..n]));
                            assert!(wire.receive(&mut packet[..n], now).unwrap());
                        }
                        Err(DeviceReadError::WouldBlock) => break,
                        Err(error) => panic!("peer read: {error:?}"),
                    }
                }
                assert!(
                    started.elapsed() < Duration::from_secs(3),
                    "in-memory control exchange timed out"
                );
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        };
        tokio::select! { _ = exchange => {}, _ = pump => unreachable!() }
    });
}
