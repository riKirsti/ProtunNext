//! The upstream local connection-test endpoint, never reachable from the tunnel.
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use proton_vpn_netstack::TransportFactory;
use std::{io, time::Duration};
/// A no-account build still supplies the upstream local connection-test service.
#[cfg(not(feature = "local-agent"))]
pub(crate) struct Service {
    stop: Option<tokio::sync::oneshot::Sender<()>>,
    join: Option<std::thread::JoinHandle<()>>,
}
#[cfg(not(feature = "local-agent"))]
impl Service {
    pub(crate) fn start(network: super::internal::InternalConnector) -> crate::Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()?;
        let (stop, stopped) = tokio::sync::oneshot::channel();
        let join = std::thread::Builder::new()
            .name("protun-next-local".into())
            .spawn(move || {
                runtime.block_on(async {
                    tokio::select! {
                        _ = stopped => {},
                        _ = run(network.factory().clone(), network.addresses().http()) => {},
                    }
                });
            })?;
        Ok(Self {
            stop: Some(stop),
            join: Some(join),
        })
    }
}
#[cfg(not(feature = "local-agent"))]
impl Drop for Service {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}
pub(crate) async fn run(factory: TransportFactory, address: std::net::SocketAddr) {
    let Ok(mut listener) = factory.tcp_bind(address) else {
        log::warn!("local connection-test listener could not bind");
        return;
    };
    while let Ok((mut stream, _)) = listener.accept().await {
        let _ = serve(&mut stream).await;
    }
}
async fn serve(stream: &mut (impl AsyncRead + AsyncWrite + Unpin)) -> io::Result<()> {
    // Read a bounded request head; a partial/slow request still gets the
    // upstream's fixed response within one second. There is no proxying.
    let _ = tokio::time::timeout(Duration::from_secs(1), async {
        let mut window = [0u8; 4];
        let mut used = 0;
        let mut buf = [0u8; 256];
        while used < 8192 {
            let maximum = buf.len().min(8192 - used);
            let n = stream.read(&mut buf[..maximum]).await?;
            if n == 0 {
                break;
            }
            used += n;
            for byte in &buf[..n] {
                window.rotate_left(1);
                window[3] = *byte;
                if window == *b"\r\n\r\n" {
                    return Ok::<(), io::Error>(());
                }
            }
        }
        Ok(())
    })
    .await;
    stream
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\nConnection: close\r\n\r\nHello World")
        .await?;
    stream.flush().await?;
    stream.close().await
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };
    struct Duplex {
        input: futures::io::Cursor<Vec<u8>>,
        output: Vec<u8>,
    }
    impl AsyncRead for Duplex {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bytes: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut self.input).poll_read(cx, bytes)
        }
    }
    impl AsyncWrite for Duplex {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context<'_>,
            bytes: &[u8],
        ) -> Poll<io::Result<usize>> {
            self.output.extend_from_slice(bytes);
            Poll::Ready(Ok(bytes.len()))
        }
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }
    #[test]
    fn fixed_local_response_accepts_any_target_and_bounds_request_reading() {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(async {
                for request in [
                    b"GET /anything HTTP/1.1\r\nHost: 10.2.0.3\r\n\r\n".to_vec(),
                    vec![b'X'; 16384],
                ] {
                    let mut stream = Duplex {
                        input: futures::io::Cursor::new(request),
                        output: vec![],
                    };
                    serve(&mut stream).await.unwrap();
                    assert!(stream.input.position() <= 8192);
                    assert!(stream.output.ends_with(b"\r\n\r\nHello World"));
                }
            });
    }
}
