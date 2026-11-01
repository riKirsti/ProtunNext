//! Reactor-facing transports. A single bounded packet worker serves UDP/TCP/TLS.
use crate::{
    Error, Result,
    packet::{Packet, PacketPool, PacketQueue},
    stream::{Flush, FramedIo, Receive},
    tls::TlsIo,
};
#[cfg(unix)]
use mio::unix::SourceFd;
use mio::{Interest, Token};
use std::io::{self, IoSlice, Read, Write};
pub(crate) const COMPLETION: Token = Token(4);

#[cfg(windows)]
use mio::net::{TcpStream, UdpSocket};
#[cfg(unix)]
use std::{
    net::{TcpStream, UdpSocket},
    os::fd::AsRawFd,
};

pub(crate) trait Network {
    fn options(&self) -> crate::performance::Options {
        Default::default()
    }
    fn diagnostics(&self) -> crate::performance::IoCounters {
        Default::default()
    }
    fn retry_delay(&self) -> Option<std::time::Duration> {
        None
    }
    fn max_ip_packet(&self) -> usize {
        65_503 // 65,535-byte WG datagram minus data header and AEAD tag.
    }
    #[cfg(unix)]
    fn fd(&self) -> i32;
    #[cfg(unix)]
    fn register(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()> {
        registry.register(&mut SourceFd(&self.fd()), token, interest)
    }
    #[cfg(unix)]
    fn reregister(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()> {
        registry.reregister(&mut SourceFd(&self.fd()), token, interest)
    }
    #[cfg(windows)]
    fn register(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()>;
    #[cfg(windows)]
    fn reregister(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()>;
    fn receive(&mut self, packet: &mut Packet, pool: &mut PacketPool) -> Result<Receive>;
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool>;
    fn pending(&self) -> usize {
        0
    }
    fn wants_write(&self) -> bool {
        false
    }
    fn handshaking(&self) -> bool {
        false
    }
}
pub(crate) struct Datagram {
    socket: UdpSocket,
    options: crate::performance::Options,
    counters: crate::performance::IoCounters,
    retry_at: Option<std::time::Instant>,
    #[cfg(target_os = "linux")]
    batch: Option<crate::linux_batch::Batch>,
    #[cfg(all(target_os = "linux", feature = "io-uring"))]
    uring: Option<crate::uring_rx::Receiver>,
}
impl Datagram {
    pub(crate) fn new(socket: UdpSocket, options: crate::performance::Options) -> Self {
        #[cfg(target_os = "linux")]
        let mut batch = (options.udp_backend != crate::performance::UdpBackend::Portable)
            .then(|| crate::linux_batch::Batch::new(&socket, options));
        #[cfg(all(target_os = "linux", feature = "io-uring"))]
        let uring = if options.udp_backend == crate::performance::UdpBackend::IoUring {
            // RecvMulti lacks GRO control messages: preserve their boundaries by
            // selecting the metadata-aware batch path when offload is requested.
            match (!options.udp_offload)
                .then(|| crate::uring_rx::Receiver::new(&socket, options.udp_batch_size))
            {
                Some(Ok(receiver)) => Some(receiver),
                _ => {
                    batch
                        .as_mut()
                        .expect("batched fallback")
                        .counters
                        .kernel_fallbacks += 1;
                    None
                }
            }
        } else {
            None
        };
        #[cfg(all(target_os = "linux", not(feature = "io-uring")))]
        if options.udp_backend == crate::performance::UdpBackend::IoUring {
            batch
                .as_mut()
                .expect("batched fallback")
                .counters
                .kernel_fallbacks += 1;
        }
        let mut counters = crate::performance::IoCounters::default();
        if !cfg!(target_os = "linux")
            && options.udp_backend != crate::performance::UdpBackend::Portable
        {
            counters.kernel_fallbacks += 1;
        }
        Self {
            socket,
            options,
            counters,
            retry_at: None,
            #[cfg(target_os = "linux")]
            batch,
            #[cfg(all(target_os = "linux", feature = "io-uring"))]
            uring,
        }
    }
}
impl Network for Datagram {
    fn options(&self) -> crate::performance::Options {
        self.options
    }
    fn diagnostics(&self) -> crate::performance::IoCounters {
        #[cfg(target_os = "linux")]
        if let Some(batch) = &self.batch {
            #[cfg(all(target_os = "linux", feature = "io-uring"))]
            if let Some(receiver) = &self.uring {
                return crate::performance::IoCounters {
                    io_uring_submissions: receiver.submissions,
                    ..batch.counters
                };
            }
            return batch.counters;
        }
        self.counters
    }
    fn retry_delay(&self) -> Option<std::time::Duration> {
        #[cfg(target_os = "linux")]
        if let Some(batch) = &self.batch {
            return batch.retry_delay();
        }
        self.retry_at
            .map(|when| when.saturating_duration_since(std::time::Instant::now()))
            .filter(|delay| !delay.is_zero())
    }
    #[cfg(windows)]
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.register(&mut self.socket, t, i)
    }
    #[cfg(unix)]
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.register(&mut SourceFd(&self.fd()), t, i)?;
        #[cfg(all(target_os = "linux", feature = "io-uring"))]
        if let Some(receiver) = &self.uring {
            r.register(
                &mut SourceFd(&receiver.event_fd()),
                COMPLETION,
                Interest::READABLE,
            )?;
        }
        Ok(())
    }
    #[cfg(windows)]
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.reregister(&mut self.socket, t, i)
    }

    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.socket.as_raw_fd()
    }
    fn receive(&mut self, packet: &mut Packet, pool: &mut PacketPool) -> Result<Receive> {
        #[cfg(all(target_os = "linux", feature = "io-uring"))]
        if let Some(receiver) = &mut self.uring
            && !receiver.disabled
        {
            match receiver.receive(
                &self.socket,
                packet,
                &mut self.batch.as_mut().expect("batch sender").counters,
            ) {
                Err(Error::Unsupported(_)) => {} // Terminal unsupported CQE returned no packet.
                result => return result,
            }
        }
        #[cfg(target_os = "linux")]
        if let Some(batch) = &mut self.batch {
            return batch.receive(&self.socket, packet, pool);
        }
        let _ = pool;
        self.counters.receive_calls += 1;
        match self.socket.recv(packet.writable()) {
            Ok(n) => {
                packet.set_len(n)?;
                self.counters.received_datagrams += 1;
                self.counters.max_receive_batch = self.counters.max_receive_batch.max(1);
                Ok(Receive::Packet)
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(Receive::WouldBlock),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => Ok(Receive::Yield),
            Err(e) => Err(e.into()),
        }
    }
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool> {
        #[cfg(target_os = "linux")]
        if let Some(batch) = &mut self.batch {
            return batch.flush(&self.socket, queue, pool);
        }
        if self.retry_delay().is_some() {
            return Ok(!queue.is_empty());
        }
        self.retry_at = None;
        for _ in 0..64 {
            let Some(packet) = queue.front() else {
                return Ok(false);
            };
            self.counters.send_calls += 1;
            match self.socket.send(packet.data()) {
                Ok(n) if n == packet.data().len() => {
                    pool.recycle(queue.pop().expect("front exists"))?;
                    self.counters.sent_datagrams += 1;
                    self.counters.max_send_batch = 1;
                }
                Ok(_) => return Err(Error::Invalid("partial UDP send")),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(true),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) if crate::platform::send_pressure(&e) => {
                    self.counters.enobufs += 1;
                    self.retry_at =
                        Some(std::time::Instant::now() + std::time::Duration::from_millis(2));
                    return Ok(true);
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(!queue.is_empty())
    }
}
pub(crate) trait WireSocket: Read + Write {
    fn receive_frame(stream: &mut FramedIo<Self>, packet: &mut Packet) -> Result<Receive>
    where
        Self: Sized,
    {
        stream.receive(packet)
    }
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()>;
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()>;

    #[cfg(unix)]
    fn fd(&self) -> i32;
    fn wants_write(&self) -> bool {
        false
    }
    fn handshaking(&self) -> bool {
        false
    }
}
impl WireSocket for TcpStream {
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        #[cfg(unix)]
        {
            r.register(&mut SourceFd(&self.as_raw_fd()), t, i)
        }
        #[cfg(windows)]
        {
            r.register(self, t, i)
        }
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        #[cfg(unix)]
        {
            r.reregister(&mut SourceFd(&self.as_raw_fd()), t, i)
        }
        #[cfg(windows)]
        {
            r.reregister(self, t, i)
        }
    }

    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.as_raw_fd()
    }
}
/// A nonblocking SYN is completed by the packet reactor rather than a setup
/// wait loop. The TUN and local service continue making progress meanwhile.
pub(crate) struct ConnectingTcp {
    socket: TcpStream,
    connected: bool,
}
impl ConnectingTcp {
    pub(crate) fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            connected: false,
        }
    }
    fn ready(&mut self) -> io::Result<()> {
        if self.connected {
            return Ok(());
        }
        if let Some(error) = self.socket.take_error()? {
            return Err(error);
        }
        match self.socket.peer_addr() {
            Ok(_) => {
                self.connected = true;
                Ok(())
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotConnected | io::ErrorKind::WouldBlock
                ) =>
            {
                Err(io::ErrorKind::WouldBlock.into())
            }
            Err(error) => Err(error),
        }
    }
}
impl Read for ConnectingTcp {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.ready()?;
        self.socket.read(bytes)
    }
}
impl Write for ConnectingTcp {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.ready()?;
        self.socket.write(bytes)
    }
    fn write_vectored(&mut self, bytes: &[IoSlice<'_>]) -> io::Result<usize> {
        self.ready()?;
        self.socket.write_vectored(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.ready()?;
        self.socket.flush()
    }
}
impl WireSocket for ConnectingTcp {
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::register(&mut self.socket, r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::reregister(&mut self.socket, r, t, i)
    }
    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.socket.as_raw_fd()
    }
    fn wants_write(&self) -> bool {
        !self.connected
    }
    fn handshaking(&self) -> bool {
        !self.connected
    }
}
impl<S: WireSocket> WireSocket for TlsIo<S> {
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::register(self.socket_mut(), r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::reregister(self.socket_mut(), r, t, i)
    }

    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.socket().fd()
    }
    fn wants_write(&self) -> bool {
        self.wants_write() || self.socket().wants_write()
    }
    fn handshaking(&self) -> bool {
        self.is_handshaking() || self.socket().handshaking()
    }
}
impl<S: WireSocket> WireSocket for crate::tls_unbuffered::UnbufferedTlsIo<S> {
    fn receive_frame(stream: &mut FramedIo<Self>, packet: &mut Packet) -> Result<Receive> {
        stream.receive_using(packet, |io, decoder, _| {
            Ok(io.read_plaintext(|data| {
                decoder.feed(data).map_err(io::Error::other)?;
                Ok(data.len())
            })?)
        })
    }
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        self.socket_mut().register(r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        self.socket_mut().reregister(r, t, i)
    }
    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.socket().fd()
    }
    fn wants_write(&self) -> bool {
        self.wants_write() || self.socket().wants_write()
    }
    fn handshaking(&self) -> bool {
        self.is_handshaking() || self.socket().handshaking()
    }
}
pub(crate) struct Stream<S>(pub FramedIo<S>, pub crate::performance::Options);
impl<S: WireSocket> Network for Stream<S> {
    fn options(&self) -> crate::performance::Options {
        self.1
    }
    fn max_ip_packet(&self) -> usize {
        0x3fff - 32 // Must also fit a normal, uncompressed TunSafe frame.
    }
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::register(self.0.io_mut(), r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::reregister(self.0.io_mut(), r, t, i)
    }

    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.0.io().fd()
    }
    fn receive(&mut self, packet: &mut Packet, _: &mut PacketPool) -> Result<Receive> {
        S::receive_frame(&mut self.0, packet)
    }
    fn pending(&self) -> usize {
        self.0.pending()
    }
    fn wants_write(&self) -> bool {
        self.0.io().wants_write()
    }
    fn handshaking(&self) -> bool {
        self.0.io().handshaking()
    }
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool> {
        while self.0.pending() < 64 {
            let Some(packet) = queue.pop() else {
                break;
            };
            if let Err(rejected) = self.0.queue(packet) {
                pool.recycle(rejected.packet)?;
                return Err(rejected.error);
            }
        }
        for _ in 0..64 {
            match self.0.flush(pool)? {
                Flush::Empty | Flush::WouldBlock => break,
                Flush::Progress => {}
            }
        }
        // TLS ciphertext can remain buffered after the final plaintext packet
        // has left FramedIo. Always pump it and retain writable interest.
        match self.0.io_mut().flush() {
            Ok(()) => {}
            Err(e)
                if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
        Ok(self.0.pending() > 0 || self.wants_write())
    }
}
