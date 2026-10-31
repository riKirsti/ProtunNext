//! Standard certificate-verifying TLS by default; no silent insecure fallback.
//! Proton Stealth's authenticated-inner-WireGuard certificate policy needs a
//! separate, explicit integration review before a live Stealth release.
use crate::{Error, Result};
use std::io::{self, IoSlice, Read, Write};
use std::sync::Arc;

/// Nonblocking rustls adapter. The reactor must include `wants_write()` in its
/// writable interest even when its application packet queue has become empty.
/// Accepted plaintext remains owned by rustls until the socket acknowledges it.
pub struct TlsIo<S> {
    socket: S,
    connection: rustls::ClientConnection,
}
impl<S: Read + Write> TlsIo<S> {
    pub fn new(socket: S, mut connection: rustls::ClientConnection) -> Self {
        connection.set_buffer_limit(Some(65_536));
        Self { socket, connection }
    }
    pub fn socket(&self) -> &S {
        &self.socket
    }
    pub fn wants_write(&self) -> bool {
        self.connection.wants_write()
    }
    pub fn is_handshaking(&self) -> bool {
        self.connection.is_handshaking()
    }
    /// Bounded work: callers reschedule a nonempty TLS output queue on writability.
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
impl<S: Read + Write> Read for TlsIo<S> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        for _ in 0..64 {
            match self.connection.reader().read(output) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                result => return result,
            }
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
        // Distinguish a work-budget yield from socket WouldBlock: FramedIo will
        // retry and ultimately return Receive::Yield, preserving edge readiness.
        Err(io::ErrorKind::Interrupted.into())
    }
}
impl<S: Read + Write> Write for TlsIo<S> {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        self.write_vectored(&[IoSlice::new(input)])
    }
    fn write_vectored(&mut self, input: &[IoSlice<'_>]) -> io::Result<usize> {
        if input.iter().all(|slice| slice.is_empty()) {
            return Ok(0);
        }
        match self.flush_tls() {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
            Ok(()) => {}
        }
        // Do not flush after accepting bytes: a later socket error must not
        // report these accepted bytes as unconsumed to the framing queue.
        // Preserve FramedIo's batch at the rustls boundary instead of falling
        // back to Write's default, which accepts only its first nonempty slice.
        match self.connection.writer().write_vectored(input) {
            Ok(0) => Err(io::ErrorKind::WouldBlock.into()),
            result => result,
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flush_tls()
    }
}
pub fn client(server_name: &str) -> Result<rustls::ClientConnection> {
    let roots = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    client_with_roots(server_name, roots)
}
pub fn client_with_roots(
    server_name: &str,
    roots: rustls::RootCertStore,
) -> Result<rustls::ClientConnection> {
    let mut config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .map_err(|e| Error::Protocol(e.to_string()))?
    .with_root_certificates(roots)
    .with_no_client_auth();
    config.alpn_protocols.clear();
    rustls::ClientConnection::new(
        Arc::new(config),
        rustls::pki_types::ServerName::try_from(server_name.to_owned())
            .map_err(|_| Error::Invalid("TLS server name"))?,
    )
    .map_err(|e| Error::Protocol(e.to_string()))
}
