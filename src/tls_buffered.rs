//! Bounded nonblocking rustls adapter selected on Windows and Apple.
use std::io::{self, IoSlice, Read, Write};

/// Accepted plaintext belongs to rustls until its ciphertext is sent.
/// The reactor must retain writable interest while `wants_write()` is true.
pub struct BufferedTlsIo<S> {
    socket: S,
    connection: rustls::ClientConnection,
    write_closed: bool,
}
impl<S: Read + Write> BufferedTlsIo<S> {
    pub fn new(socket: S, mut connection: rustls::ClientConnection) -> Self {
        connection.set_buffer_limit(Some(65_536));
        Self {
            socket,
            connection,
            write_closed: false,
        }
    }
    pub fn socket(&self) -> &S {
        &self.socket
    }
    pub fn socket_mut(&mut self) -> &mut S {
        &mut self.socket
    }
    pub fn wants_write(&self) -> bool {
        self.connection.wants_write()
    }
    pub fn is_handshaking(&self) -> bool {
        self.connection.is_handshaking()
    }
    /// Queue a half-close; `flush` sends the alert, and reads remain available.
    pub fn queue_close_notify(&mut self) -> io::Result<()> {
        if self.write_closed {
            return Ok(());
        }
        self.connection.send_close_notify();
        self.write_closed = true;
        Ok(())
    }
    pub fn refresh_traffic_keys(&mut self) -> io::Result<()> {
        if self.write_closed {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        if self.connection.is_handshaking() {
            return Err(io::ErrorKind::WouldBlock.into());
        }
        if self.connection.protocol_version() != Some(rustls::ProtocolVersion::TLSv1_3) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "key update requires TLS 1.3",
            ));
        }
        self.connection
            .refresh_traffic_keys()
            .map_err(io::Error::other)
    }
    /// Bound each reactor turn without losing the remaining writable interest.
    pub fn flush_tls(&mut self) -> io::Result<()> {
        for _ in 0..64 {
            if !self.connection.wants_write() {
                return Ok(());
            }
            match self.connection.write_tls(&mut self.socket) {
                Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        Err(io::ErrorKind::WouldBlock.into())
    }
}
impl<S: Read + Write> Read for BufferedTlsIo<S> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        for _ in 0..64 {
            match self.connection.reader().read(output) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                result => return result,
            }
            // Blocked output must not stop independent readable TLS traffic.
            match self.flush_tls() {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
                Ok(()) => {}
            }
            if !self.connection.wants_read() {
                return Err(io::ErrorKind::WouldBlock.into());
            }
            match self.connection.read_tls(&mut self.socket) {
                Ok(_) => {
                    self.connection
                        .process_new_packets()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        // Framing distinguishes this work-budget yield from socket WouldBlock.
        Err(io::ErrorKind::Interrupted.into())
    }
}
impl<S: Read + Write> Write for BufferedTlsIo<S> {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        self.write_vectored(&[IoSlice::new(input)])
    }
    fn write_vectored(&mut self, input: &[IoSlice<'_>]) -> io::Result<usize> {
        if input.iter().all(|s| s.is_empty()) {
            return Ok(0);
        }
        if self.write_closed {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        match self.flush_tls() {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
            Ok(()) => {}
        }
        // Never flush after accepting bytes: a later socket error must not
        // make the framing queue retry plaintext already accepted by rustls.
        match self.connection.writer().write_vectored(input) {
            Ok(0) => Err(io::ErrorKind::WouldBlock.into()),
            result => result,
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flush_tls()
    }
}
