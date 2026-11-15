//! UDP delivery and bounded send retry; Linux/Android retain their batch owner.
use super::Network;
#[cfg(not(any(target_os = "linux", target_os = "android")))]
use crate::Error;
use crate::{
    Result,
    packet::{Packet, PacketPool, PacketQueue},
    stream::Receive,
};
#[cfg(windows)]
use mio::net::UdpSocket;
#[cfg(unix)]
use mio::unix::SourceFd;
use mio::{Interest, Token};
use std::io;
#[cfg(unix)]
use std::{net::UdpSocket, os::fd::AsRawFd};

pub(crate) struct Datagram {
    socket: UdpSocket,
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    counters: crate::performance::IoCounters,
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    retry_at: Option<std::time::Instant>,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    batch: crate::linux_batch::Batch,
}
impl Datagram {
    pub(crate) fn new(socket: UdpSocket) -> Self {
        Self {
            #[cfg(any(target_os = "linux", target_os = "android"))]
            batch: crate::linux_batch::Batch::new(&socket, true),
            socket,
            #[cfg(not(any(target_os = "linux", target_os = "android")))]
            counters: Default::default(),
            #[cfg(not(any(target_os = "linux", target_os = "android")))]
            retry_at: None,
        }
    }
}
impl Network for Datagram {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.counters
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            self.counters
        }
    }
    fn retry_delay(&self) -> Option<std::time::Duration> {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.retry_delay()
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            self.retry_at
                .map(|when| when.saturating_duration_since(std::time::Instant::now()))
                .filter(|delay| !delay.is_zero())
        }
    }
    #[cfg(windows)]
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.register(&mut self.socket, t, i)
    }
    #[cfg(unix)]
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.register(&mut SourceFd(&self.fd()), t, i)?;
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
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.receive(&self.socket, packet, pool)
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            let _ = pool;
            self.counters.receive_calls += 1;
            let result = self.socket.recv(packet.writable());
            if let Err(error) = &result {
                self.counters.socket_error(error, false);
            }
            match result {
                Ok(n) => {
                    packet.set_len(n)?;
                    self.counters.received_datagrams += 1;
                    self.counters.received_bytes += n as u64;
                    self.counters.max_receive_batch = self.counters.max_receive_batch.max(1);
                    Ok(Receive::Packet)
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(Receive::WouldBlock),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => Ok(Receive::Yield),
                Err(e) => Err(e.into()),
            }
        }
    }
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool> {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.flush(&self.socket, queue, pool)
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            if self.retry_delay().is_some() {
                return Ok(!queue.is_empty());
            }
            self.retry_at = None;
            for _ in 0..64 {
                let Some(packet) = queue.front() else {
                    return Ok(false);
                };
                self.counters.send_calls += 1;
                let result = self.socket.send(packet.data());
                if let Err(error) = &result {
                    self.counters.socket_error(error, true);
                }
                match result {
                    Ok(n) if n == packet.data().len() => {
                        pool.recycle(queue.pop().expect("front exists"))?;
                        self.counters.sent_datagrams += 1;
                        self.counters.sent_bytes += n as u64;
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
}
