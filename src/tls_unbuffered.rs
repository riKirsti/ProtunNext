//! Caller-owned TLS record buffers, using the pinned rustls unbuffered state
//! machine. This is not kTLS and does not claim allocation-free TLS reception.
use crate::packet::erase;
use rustls::{client::UnbufferedClientConnection, unbuffered::ConnectionState};
use std::io::{self, IoSlice, Read, Write};

const TLS_BUFFER: usize = 65536;
const RECORD: usize = 16384;
// Room for four maximum plaintext records and their negotiated AEAD framing.
// InsufficientSize remains authoritative if rustls needs more than this reserve.
const RECORD_RESERVE: usize = 256;
const TLS_OUTPUT: usize = TLS_BUFFER + 4 * RECORD_RESERVE;
const MAX_PROTOCOL_OUTPUT: usize = 128 * 1024;

fn encode_control<Data>(
    state: &mut rustls::unbuffered::EncodeTlsData<'_, Data>,
    output: &mut Box<[u8]>,
    used: usize,
) -> io::Result<usize> {
    match state.encode(&mut output[used..]) {
        Ok(n) => Ok(n),
        Err(rustls::unbuffered::EncodeError::InsufficientSize(size)) => {
            let required = used
                .checked_add(size.required_size)
                .ok_or(io::ErrorKind::InvalidData)?;
            if required > MAX_PROTOCOL_OUTPUT {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "bounded TLS protocol output exhausted",
                ));
            }
            // Rare protocol backpressure, not per-application-record allocation.
            let mut bytes = std::mem::replace(output, Box::new([])).into_vec();
            bytes.resize(required.next_power_of_two().max(TLS_BUFFER), 0);
            *output = bytes.into_boxed_slice();
            state.encode(&mut output[used..]).map_err(io::Error::other)
        }
        Err(e) => Err(io::Error::other(e)),
    }
}
enum Step {
    Progress,
    Read(usize),
    Written(usize),
    NeedRead,
    ReadBlocked,
    NeedFlush,
    Closed,
    PeerClosed,
}
enum ControlMessage {
    Close,
    KeyUpdate,
}

pub struct UnbufferedTlsIo<S> {
    socket: S,
    connection: UnbufferedClientConnection,
    incoming: Box<[u8]>,
    first: usize,
    end: usize,
    outgoing: Box<[u8]>,
    sent: usize,
    encoded: usize,
    pending: Box<[u8]>,
    pending_first: usize,
    pending_end: usize,
    gather: Box<[u8]>,
    drive_write: bool,
    eof: bool,
    write_closed: bool,
    control: Option<ControlMessage>,
}
impl<S: Read + Write> UnbufferedTlsIo<S> {
    pub fn new(socket: S, connection: UnbufferedClientConnection) -> Self {
        Self {
            socket,
            connection,
            incoming: vec![0; TLS_BUFFER].into_boxed_slice(),
            first: 0,
            end: 0,
            outgoing: vec![0; TLS_OUTPUT].into_boxed_slice(),
            sent: 0,
            encoded: 0,
            pending: vec![0; RECORD].into_boxed_slice(),
            pending_first: 0,
            pending_end: 0,
            gather: vec![0; RECORD].into_boxed_slice(),
            drive_write: true,
            eof: false,
            write_closed: false,
            control: None,
        }
    }
    pub fn socket(&self) -> &S {
        &self.socket
    }
    pub fn socket_mut(&mut self) -> &mut S {
        &mut self.socket
    }
    pub fn wants_write(&self) -> bool {
        self.sent < self.encoded || self.drive_write
    }
    pub fn is_handshaking(&self) -> bool {
        self.connection.is_handshaking()
    }
    fn flush_output(&mut self) -> io::Result<()> {
        for _ in 0..64 {
            if self.sent == self.encoded {
                self.sent = 0;
                self.encoded = 0;
                return Ok(());
            }
            match self.socket.write(&self.outgoing[self.sent..self.encoded]) {
                Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
                Ok(n) if n <= self.encoded - self.sent => self.sent += n,
                Ok(_) => return Err(io::ErrorKind::InvalidData.into()),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        Err(io::ErrorKind::WouldBlock.into())
    }
    fn receive_tls(&mut self) -> io::Result<()> {
        if self.end == self.incoming.len() && self.first != 0 {
            self.incoming.copy_within(self.first..self.end, 0);
            self.end -= self.first;
            self.first = 0;
        }
        if self.end == self.incoming.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "bounded TLS input exhausted",
            ));
        }
        match self.socket.read(&mut self.incoming[self.end..]) {
            Ok(0) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "outer TLS missing close_notify",
            )),
            Ok(n) => {
                self.end += n;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    fn step(
        &mut self,
        input: Option<&[u8]>,
        consume: &mut impl FnMut(&[u8]) -> io::Result<usize>,
    ) -> io::Result<Step> {
        let append_application = input.is_some_and(|data| {
            !self.connection.is_handshaking()
                && !self.drive_write
                && self.control.is_none()
                && self.outgoing.len() - self.encoded >= data.len() + RECORD_RESERVE
        });
        match if append_application {
            Ok(())
        } else {
            self.flush_output()
        } {
            Ok(()) => {}
            // Readability is independent of writability. In particular both
            // directions can have full TCP send buffers at the same time.
            Err(e) if input.is_none() && e.kind() == io::ErrorKind::WouldBlock => {
                if self.sent > 0 {
                    self.outgoing.copy_within(self.sent..self.encoded, 0);
                    self.encoded -= self.sent;
                    self.sent = 0;
                }
            }
            Err(e) => return Err(e),
        }
        // Once established, writes pump TLS output using an empty input span.
        // Feeding unread ciphertext here makes rustls return ReadTraffic before
        // WriteTraffic. Parking that plaintext can otherwise block a full ACK
        // queue whose owner has paused reads until its output drains.
        // rustls 0.23.46 process_tls_records_common explicitly yields
        // WriteTraffic on empty input when application writes are permitted.
        let process_end = if input.is_some() && !self.connection.is_handshaking() {
            self.first
        } else {
            self.end
        };
        let supplied = process_end - self.first;
        let status = self
            .connection
            .process_tls_records(&mut self.incoming[self.first..process_end]);
        let mut discard = status.discard;
        let result = match status.state {
            Ok(ConnectionState::ReadTraffic(mut traffic)) => {
                if input.is_some() && self.pending_first < self.pending_end {
                    // Leave any pre-existing rustls plaintext owned by rustls
                    // until there is room; never overwrite parked application data.
                    Ok(Step::ReadBlocked)
                } else if let Some(record) = traffic.next_record() {
                    let record = record.map_err(io::Error::other)?;
                    discard += record.discard;
                    consume(record.payload).and_then(|n| {
                        if n != record.payload.len() {
                            return Err(io::ErrorKind::InvalidInput.into());
                        }
                        Ok(Step::Read(n))
                    })
                } else {
                    Ok(Step::Progress)
                }
            }
            Ok(ConnectionState::EncodeTlsData(mut data)) => {
                match encode_control(&mut data, &mut self.outgoing, self.encoded) {
                    Ok(n) => {
                        self.encoded += n;
                        self.drive_write = true;
                        Ok(Step::Progress)
                    }
                    Err(e) => Err(io::Error::other(e)),
                }
            }
            Ok(ConnectionState::TransmitTlsData(data)) => {
                if self.sent < self.encoded {
                    // Keep rustls's transmit flag until all encoded bytes have
                    // reached socket.write, but continue processing inbound TLS.
                    self.drive_write = true;
                    Ok(Step::NeedRead)
                } else {
                    data.done();
                    self.drive_write = false;
                    Ok(Step::Progress)
                }
            }
            Ok(ConnectionState::WriteTraffic(mut traffic)) => {
                self.drive_write = false;
                if let Some(control) = self.control.take() {
                    match control {
                        ControlMessage::Close => {
                            match traffic.queue_close_notify(&mut self.outgoing[self.encoded..]) {
                                Ok(n) => {
                                    self.encoded += n;
                                    self.write_closed = true;
                                    Ok(Step::Written(0))
                                }
                                Err(e) => Err(io::Error::other(e)),
                            }
                        }
                        ControlMessage::KeyUpdate => traffic
                            .refresh_traffic_keys()
                            .map_err(io::Error::other)
                            .map(|()| {
                                self.drive_write = true;
                                Step::Written(0)
                            }),
                    }
                } else if let Some(input) = input {
                    match traffic.encrypt(input, &mut self.outgoing[self.encoded..]) {
                        Ok(n) => {
                            self.encoded += n;
                            Ok(Step::Written(input.len()))
                        }
                        Err(rustls::unbuffered::EncryptError::InsufficientSize(_))
                            if self.encoded > 0 =>
                        {
                            Ok(Step::NeedFlush)
                        }
                        Err(e) => Err(io::Error::other(e)),
                    }
                } else {
                    Ok(Step::NeedRead)
                }
            }
            Ok(ConnectionState::BlockedHandshake) => {
                self.drive_write = false;
                Ok(Step::NeedRead)
            }
            Ok(ConnectionState::PeerClosed) => {
                self.eof = true;
                self.drive_write = false;
                Ok(Step::PeerClosed)
            }
            Ok(ConnectionState::Closed) => {
                self.eof = true;
                self.write_closed = true;
                self.drive_write = false;
                Ok(Step::Closed)
            }
            Ok(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected client TLS state",
            )),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("outer TLS: {e}"),
            )),
        };
        // Apply *both* status and application-record discard before the next call.
        if discard > supplied {
            return Err(io::ErrorKind::InvalidData.into());
        }
        erase(&mut self.incoming[self.first..self.first + discard]);
        self.first += discard;
        if self.first == self.end {
            self.first = 0;
            self.end = 0;
        }
        result
    }
    /// Consume a complete record span without an intermediate Read scratch copy.
    /// The callback must accept the entire span; framing supplies enough capacity.
    pub fn read_plaintext(
        &mut self,
        mut consume: impl FnMut(&[u8]) -> io::Result<usize>,
    ) -> io::Result<usize> {
        if self.pending_first < self.pending_end {
            let n = consume(&self.pending[self.pending_first..self.pending_end])?;
            if n > self.pending_end - self.pending_first {
                return Err(io::ErrorKind::InvalidData.into());
            }
            erase(&mut self.pending[self.pending_first..self.pending_first + n]);
            self.pending_first += n;
            return Ok(n);
        }
        if self.eof {
            return Ok(0);
        }
        for _ in 0..64 {
            match self.step(None, &mut consume)? {
                Step::Read(0) => continue,
                Step::Read(n) => return Ok(n),
                Step::Closed | Step::PeerClosed => return Ok(0),
                Step::NeedRead => match self.receive_tls() {
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    result => result?,
                },
                _ => {}
            }
        }
        Err(io::ErrorKind::Interrupted.into())
    }
    fn write_record(&mut self, input: &[u8]) -> io::Result<usize> {
        if self.write_closed {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        let previous_first = self.pending_first;
        let previous_end = self.pending_end;
        // Writes may encounter a server's application data while processing
        // handshake/control records. Keep that record until the caller reads it.
        let mut pending = std::mem::replace(&mut self.pending, Box::new([]));
        let mut received = 0;
        let result = (|| {
            for _ in 0..64 {
                match self.step(Some(input), &mut |data| {
                    if data.len() > pending.len() {
                        return Err(io::ErrorKind::InvalidData.into());
                    }
                    pending[..data.len()].copy_from_slice(data);
                    received = data.len();
                    Ok(data.len())
                })? {
                    Step::Written(0) if !input.is_empty() => {
                        if self.write_closed {
                            return Err(io::ErrorKind::BrokenPipe.into());
                        }
                    }
                    Step::Written(n) => return Ok(n), // Never flush after accepting input.
                    Step::Read(_) => return Err(io::ErrorKind::WouldBlock.into()),
                    Step::ReadBlocked => return Err(io::ErrorKind::WouldBlock.into()),
                    Step::NeedRead => self.receive_tls()?,
                    Step::NeedFlush => self.flush_output()?,
                    Step::Closed => return Err(io::ErrorKind::BrokenPipe.into()),
                    Step::PeerClosed => {}
                    Step::Progress => {}
                }
            }
            Err(io::ErrorKind::WouldBlock.into())
        })();
        self.pending = pending;
        if received != 0 {
            self.pending_first = 0;
            self.pending_end = received;
        } else {
            self.pending_first = previous_first;
            self.pending_end = previous_end;
        }
        result
    }
    /// Queue a TLS close_notify. Call flush to send it; inbound data remains readable.
    pub fn queue_close_notify(&mut self) -> io::Result<()> {
        if self.write_closed {
            return Ok(());
        }
        self.control = Some(ControlMessage::Close);
        self.write_record(&[]).map(|_| ())
    }
    /// Request a TLS 1.3 key update through rustls, preserving its key/record limits.
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
        self.control = Some(ControlMessage::KeyUpdate);
        self.drive_write = true;
        Ok(())
    }
}
impl<S: Read + Write> Read for UnbufferedTlsIo<S> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        // Keep the unconsumed suffix when a generic Read caller supplies a small buffer.
        let mut pending = std::mem::replace(&mut self.pending, Box::new([]));
        let first = self.pending_first;
        let end = self.pending_end;
        if first < end {
            let n = output.len().min(end - first);
            output[..n].copy_from_slice(&pending[first..first + n]);
            erase(&mut pending[first..first + n]);
            self.pending = pending;
            self.pending_first += n;
            return Ok(n);
        }
        let mut rest = 0;
        let mut copied = 0;
        let result = self.read_plaintext(|data| {
            let n = output.len().min(data.len());
            output[..n].copy_from_slice(&data[..n]);
            rest = data.len() - n;
            if rest > pending.len() {
                return Err(io::ErrorKind::InvalidData.into());
            }
            pending[..rest].copy_from_slice(&data[n..]);
            copied = n;
            Ok(data.len())
        });
        self.pending = pending;
        self.pending_first = 0;
        self.pending_end = rest;
        result.map(|_| copied)
    }
}
impl<S: Read + Write> Write for UnbufferedTlsIo<S> {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if input.is_empty() {
            return Ok(0);
        }
        self.write_record(&input[..input.len().min(RECORD)])
    }
    fn write_vectored(&mut self, input: &[IoSlice<'_>]) -> io::Result<usize> {
        // Sparse queues commonly contain one frame. Borrow it directly rather
        // than copying and erasing it in the gather buffer before rustls copies it.
        let mut nonempty = input.iter().filter(|slice| !slice.is_empty());
        if let Some(first) = nonempty.next()
            && (first.len() >= RECORD || nonempty.next().is_none())
        {
            return self.write(first);
        }
        let mut gather = std::mem::replace(&mut self.gather, Box::new([]));
        let mut n = 0;
        for slice in input {
            let count = slice.len().min(RECORD - n);
            gather[n..n + count].copy_from_slice(&slice[..count]);
            n += count;
            if n == RECORD {
                break;
            }
        }
        let result = self.write(&gather[..n]);
        erase(&mut gather[..n]);
        self.gather = gather;
        result
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flush_output()?;
        if !self.drive_write || self.pending_first < self.pending_end {
            return Ok(());
        }
        let mut pending = std::mem::replace(&mut self.pending, Box::new([]));
        let mut received = 0;
        let result = (|| {
            for _ in 0..64 {
                match self.step(None, &mut |data| {
                    if data.len() > pending.len() {
                        return Err(io::ErrorKind::InvalidData.into());
                    }
                    pending[..data.len()].copy_from_slice(data);
                    received = data.len();
                    Ok(data.len())
                })? {
                    Step::Progress | Step::Written(_) => {}
                    _ => return Ok(()),
                }
            }
            Err(io::ErrorKind::WouldBlock.into())
        })();
        self.pending = pending;
        self.pending_first = 0;
        self.pending_end = received;
        result
    }
}
impl<S> Drop for UnbufferedTlsIo<S> {
    fn drop(&mut self) {
        erase(&mut self.incoming);
        erase(&mut self.outgoing);
        erase(&mut self.pending);
        erase(&mut self.gather);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct Blocked;
    impl Read for Blocked {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::ErrorKind::WouldBlock.into())
        }
    }
    impl Write for Blocked {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::ErrorKind::WouldBlock.into())
        }
        fn flush(&mut self) -> io::Result<()> {
            Err(io::ErrorKind::WouldBlock.into())
        }
    }
    fn adapter() -> UnbufferedTlsIo<Blocked> {
        let config = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_no_client_auth();
        UnbufferedTlsIo::new(
            Blocked,
            UnbufferedClientConnection::new(Arc::new(config), "example.test".try_into().unwrap())
                .unwrap(),
        )
    }
    #[test]
    fn partial_pending_reads_erase_only_consumed_bytes() {
        let mut io = adapter();
        io.pending.fill(0xa5);
        io.pending[4..11].copy_from_slice(b"abcdefg");
        io.pending_first = 4;
        io.pending_end = 11;
        let mut out = [0; 3];
        assert_eq!(io.read(&mut out).unwrap(), 3);
        assert_eq!(&out, b"abc");
        assert_eq!(&io.pending[4..7], &[0; 3]);
        assert_eq!(&io.pending[7..11], b"defg");
        assert_eq!(
            io.read_plaintext(|data| {
                assert_eq!(data, b"defg");
                Ok(2)
            })
            .unwrap(),
            2
        );
        assert_eq!(&io.pending[4..9], &[0; 5]);
        assert_eq!(&io.pending[9..11], b"fg");
        assert!(
            io.read_plaintext(|_| Err(io::ErrorKind::WouldBlock.into()))
                .is_err()
        );
        assert_eq!(io.pending_first, 9);
        assert_eq!(&io.pending[9..11], b"fg");
        assert!(io.pending[..4].iter().all(|b| *b == 0xa5));
        assert!(io.pending[11..].iter().all(|b| *b == 0xa5));
    }
    #[test]
    fn blocked_vectored_write_erases_gathered_bytes_before_returning() {
        let mut io = adapter();
        io.gather.fill(0xa5);
        assert_eq!(
            io.write_vectored(&[IoSlice::new(b"abc"), IoSlice::new(b"defg")])
                .unwrap_err()
                .kind(),
            io::ErrorKind::WouldBlock
        );
        assert_eq!(&io.gather[..7], &[0; 7]);
        assert!(io.gather[7..].iter().all(|b| *b == 0xa5));
    }
}
