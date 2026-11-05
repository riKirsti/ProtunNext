//! Blocking, bounded-fixture interoperability probe. Not a production adapter:
//! asynchronous handoff/failure recovery and cipher-limit enforcement are later
//! gates. Only the fixed small test workload may use this experimental module.
use rustls::{
    ConnectionTrafficSecrets, ProtocolVersion,
    client::{ClientConnectionData, UnbufferedClientConnection},
    kernel::KernelConnection,
    unbuffered::ConnectionState,
};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    os::fd::AsRawFd,
    sync::Arc,
};
use zeroize::Zeroize;
const SOL_TLS: i32 = 282;
const TLS_TX: i32 = 1;
const TLS_RX: i32 = 2;

#[derive(Debug)]
pub struct RekeyUnavailable(pub i32);
impl std::fmt::Display for RekeyUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kernel rejected TLS key replacement (errno {})", self.0)
    }
}
impl std::error::Error for RekeyUnavailable {}
fn reinstall(
    socket: &TcpStream,
    direction: i32,
    version: ProtocolVersion,
    next: (u64, ConnectionTrafficSecrets),
) -> io::Result<()> {
    install(socket, direction, version, next).map_err(|e| {
        if e.raw_os_error() == Some(libc::EBUSY) {
            io::Error::new(io::ErrorKind::Unsupported, RekeyUnavailable(libc::EBUSY))
        } else {
            e
        }
    })
}

fn install(
    socket: &TcpStream,
    direction: i32,
    version: ProtocolVersion,
    (seq, secret): (u64, ConnectionTrafficSecrets),
) -> io::Result<()> {
    // Initial prototype supports TLS 1.3 only. TLS 1.2 AES-GCM has an explicit
    // nonce handoff to audit separately; preserve the existing userspace path.
    if version != ProtocolVersion::TLSv1_3 {
        return Err(io::ErrorKind::Unsupported.into());
    }
    // Layouts and cipher IDs: Linux v6.18 include/uapi/linux/tls.h. Arrays have
    // byte alignment; both u16 fields are encoded in native endianness.
    let mut bytes = [0u8; 56];
    bytes[..2].copy_from_slice(&u16::from(version).to_ne_bytes());
    let size = match secret {
        ConnectionTrafficSecrets::Aes128Gcm { key, iv } => {
            bytes[2..4].copy_from_slice(&51u16.to_ne_bytes());
            bytes[4..12].copy_from_slice(&iv.as_ref()[4..]);
            bytes[12..28].copy_from_slice(key.as_ref());
            bytes[28..32].copy_from_slice(&iv.as_ref()[..4]);
            bytes[32..40].copy_from_slice(&seq.to_be_bytes());
            40
        }
        ConnectionTrafficSecrets::Aes256Gcm { key, iv } => {
            bytes[2..4].copy_from_slice(&52u16.to_ne_bytes());
            bytes[4..12].copy_from_slice(&iv.as_ref()[4..]);
            bytes[12..44].copy_from_slice(key.as_ref());
            bytes[44..48].copy_from_slice(&iv.as_ref()[..4]);
            bytes[48..56].copy_from_slice(&seq.to_be_bytes());
            56
        }
        ConnectionTrafficSecrets::Chacha20Poly1305 { key, iv } => {
            bytes[2..4].copy_from_slice(&54u16.to_ne_bytes());
            bytes[4..16].copy_from_slice(iv.as_ref());
            bytes[16..48].copy_from_slice(key.as_ref());
            bytes[48..56].copy_from_slice(&seq.to_be_bytes());
            56
        }
        _ => return Err(io::ErrorKind::Unsupported.into()),
    };
    // SAFETY: connected live descriptor and exactly the selected UAPI layout.
    let result = unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            SOL_TLS,
            direction,
            bytes.as_ptr().cast(),
            size,
        )
    };
    let error = (result < 0).then(io::Error::last_os_error);
    bytes.zeroize();
    if let Some(e) = error { Err(e) } else { Ok(()) }
}

#[repr(C, align(8))]
struct Control([u8; 64]);
pub struct Ktls {
    socket: TcpStream,
    keys: KernelConnection<ClientConnectionData>,
    incoming: Box<[u8; 16384]>,
    first: usize,
    end: usize,
    handshake: Vec<u8>,
    closed: bool,
    write_closed: bool,
    pub rx_updates: u64,
    pub tx_updates: u64,
    pub tickets: u64,
}
impl Ktls {
    pub fn connect(mut socket: TcpStream, config: Arc<rustls::ClientConfig>) -> io::Result<Self> {
        let mut conn = UnbufferedClientConnection::new(config, "localhost".try_into().unwrap())
            .map_err(io::Error::other)?;
        let mut input = vec![0u8; 65536];
        let mut used = 0;
        let mut output = vec![0u8; 65536];
        let mut encoded = 0;
        loop {
            let mut need_read = false;
            let mut established = false;
            let status = conn.process_tls_records(&mut input[..used]);
            let discard = status.discard;
            match status.state.map_err(io::Error::other)? {
                ConnectionState::EncodeTlsData(mut data) => {
                    encoded += data
                        .encode(&mut output[encoded..])
                        .map_err(io::Error::other)?;
                }
                ConnectionState::TransmitTlsData(data) => {
                    socket.write_all(&output[..encoded])?;
                    output[..encoded].zeroize();
                    encoded = 0;
                    data.done();
                }
                ConnectionState::BlockedHandshake => need_read = true,
                ConnectionState::WriteTraffic(_) => established = true,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "unexpected handshake state",
                    ));
                }
            }
            assert!(discard <= used);
            input.copy_within(discard..used, 0);
            input[used - discard..used].zeroize();
            used -= discard;
            if established {
                assert_eq!(used, 0, "handoff must not lose already-read ciphertext");
                assert_eq!(encoded, 0);
                break;
            }
            if need_read {
                // Read exactly one record: no application/ticket bytes are
                // pulled out of the socket beyond the handoff record boundary.
                let mut header = [0u8; 5];
                socket.read_exact(&mut header)?;
                let n = u16::from_be_bytes([header[3], header[4]]) as usize;
                if n > 18432 || used + n + 5 > input.len() {
                    return Err(io::ErrorKind::InvalidData.into());
                }
                input[used..used + 5].copy_from_slice(&header);
                socket.read_exact(&mut input[used + 5..used + 5 + n])?;
                used += 5 + n;
            }
        }
        let (secrets, keys) = conn
            .dangerous_into_kernel_connection()
            .map_err(io::Error::other)?;
        let version = keys.protocol_version();
        // SAFETY: NUL-terminated ULP name and live established TCP socket.
        if unsafe {
            libc::setsockopt(
                socket.as_raw_fd(),
                libc::IPPROTO_TCP,
                libc::TCP_ULP,
                b"tls\0".as_ptr().cast(),
                4,
            )
        } < 0
        {
            return Err(io::Error::last_os_error());
        }
        install(&socket, TLS_TX, version, secrets.tx)?;
        install(&socket, TLS_RX, version, secrets.rx)?;
        Ok(Self {
            socket,
            keys,
            incoming: Box::new([0; 16384]),
            first: 0,
            end: 0,
            handshake: Vec::new(),
            closed: false,
            write_closed: false,
            rx_updates: 0,
            tx_updates: 0,
            tickets: 0,
        })
    }
    pub fn suite(&self) -> String {
        format!("{:?}", self.keys.negotiated_cipher_suite())
    }
    /// Component probe borrows decrypted records, as the production decoder does.
    pub fn read_plaintext(
        &mut self,
        mut consume: impl FnMut(&[u8]) -> io::Result<usize>,
    ) -> io::Result<usize> {
        loop {
            if self.first < self.end {
                let n = consume(&self.incoming[self.first..self.end])?;
                if n > self.end - self.first {
                    return Err(io::ErrorKind::InvalidData.into());
                }
                self.incoming[self.first..self.first + n].zeroize();
                self.first += n;
                return Ok(n);
            }
            if self.closed {
                return Ok(0);
            }
            self.receive()?;
        }
    }
    fn control(&mut self, typ: u8, bytes: &[u8]) -> io::Result<()> {
        let mut storage = Control([0; 64]);
        let mut sent = 0;
        while sent < bytes.len() {
            let mut iov = libc::iovec {
                iov_base: bytes[sent..].as_ptr().cast_mut().cast(),
                iov_len: bytes.len() - sent,
            };
            // SAFETY: zero valid msghdr populated with live aligned buffers.
            let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
            msg.msg_iov = &mut iov;
            msg.msg_iovlen = 1;
            msg.msg_control = storage.0.as_mut_ptr().cast();
            msg.msg_controllen = unsafe { libc::CMSG_SPACE(1) } as _;
            unsafe {
                let h = libc::CMSG_FIRSTHDR(&msg);
                (*h).cmsg_level = SOL_TLS;
                (*h).cmsg_type = 1;
                (*h).cmsg_len = libc::CMSG_LEN(1) as _;
                *libc::CMSG_DATA(h) = typ;
            }
            let n = unsafe { libc::sendmsg(self.socket.as_raw_fd(), &msg, libc::MSG_NOSIGNAL) };
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            if n == 0 {
                return Err(io::ErrorKind::WriteZero.into());
            }
            sent += n as usize;
        }
        Ok(())
    }
    pub fn key_update(&mut self) -> io::Result<()> {
        // KeyUpdate itself uses the old TX key; subsequent records use the new
        // one. This probe is blocking and serializes every operation.
        self.control(22, &[24, 0, 0, 1, 0])?;
        let next = self.keys.update_tx_secret().map_err(io::Error::other)?;
        reinstall(&self.socket, TLS_TX, self.keys.protocol_version(), next)?;
        self.tx_updates += 1;
        Ok(())
    }
    pub fn close_notify(&mut self) -> io::Result<()> {
        if !self.write_closed {
            self.control(21, &[1, 0])?;
            self.write_closed = true;
        }
        Ok(())
    }
    fn handle_handshake(&mut self) -> io::Result<()> {
        loop {
            if self.handshake.len() < 4 {
                return Ok(());
            }
            let n = u32::from_be_bytes([0, self.handshake[1], self.handshake[2], self.handshake[3]])
                as usize;
            if n > 65536 {
                return Err(io::ErrorKind::InvalidData.into());
            }
            if self.handshake.len() < 4 + n {
                return Ok(());
            }
            match self.handshake[0] {
                4 => {
                    self.keys
                        .handle_new_session_ticket(&self.handshake[4..4 + n])
                        .map_err(io::Error::other)?;
                    self.tickets += 1;
                }
                24 if n == 1 && self.handshake[4] <= 1 => {
                    let requested = self.handshake[4] == 1;
                    let next = self.keys.update_rx_secret().map_err(io::Error::other)?;
                    reinstall(&self.socket, TLS_RX, self.keys.protocol_version(), next)?;
                    self.rx_updates += 1;
                    if requested {
                        self.key_update()?;
                    }
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "unexpected post-handshake message",
                    ));
                }
            }
            let old = self.handshake.len();
            self.handshake.copy_within(4 + n.., 0);
            self.handshake[old - 4 - n..].zeroize();
            self.handshake.truncate(old - 4 - n);
        }
    }
    fn receive(&mut self) -> io::Result<()> {
        let mut storage = Control([0; 64]);
        let mut iov = libc::iovec {
            iov_base: self.incoming.as_mut_ptr().cast(),
            iov_len: self.incoming.len(),
        };
        let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1;
        msg.msg_control = storage.0.as_mut_ptr().cast();
        msg.msg_controllen = 64;
        let n = unsafe { libc::recvmsg(self.socket.as_raw_fd(), &mut msg, 0) };
        if n < 0 {
            return Err(io::Error::last_os_error());
        }
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "missing close_notify",
            ));
        }
        if msg.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0 {
            return Err(io::ErrorKind::InvalidData.into());
        }
        let mut typ = 23u8;
        let mut h = unsafe { libc::CMSG_FIRSTHDR(&msg) };
        while !h.is_null() {
            let value = unsafe { &*h };
            let offset = h as usize - msg.msg_control as usize;
            let base = unsafe { libc::CMSG_LEN(1) } as usize;
            if (value.cmsg_len as usize) < base
                || offset + value.cmsg_len as usize > msg.msg_controllen as usize
            {
                return Err(io::ErrorKind::InvalidData.into());
            }
            if value.cmsg_level == SOL_TLS && value.cmsg_type == 2 {
                typ = unsafe { *libc::CMSG_DATA(h) };
            }
            h = unsafe { libc::CMSG_NXTHDR(&msg, h) };
        }
        self.first = 0;
        self.end = n as usize;
        match typ {
            23 => {}
            22 => {
                if self.handshake.len() + self.end > 65540 {
                    return Err(io::ErrorKind::InvalidData.into());
                }
                self.handshake.extend_from_slice(&self.incoming[..self.end]);
                self.incoming[..self.end].zeroize();
                self.end = 0;
                self.handle_handshake()?;
            }
            21 => {
                if self.incoming[..self.end] != [1, 0] {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "TLS alert"));
                }
                self.incoming[..self.end].zeroize();
                self.end = 0;
                self.closed = true;
            }
            _ => return Err(io::ErrorKind::InvalidData.into()),
        }
        Ok(())
    }
}
impl Read for Ktls {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() {
            return Ok(0);
        }
        loop {
            if self.first < self.end {
                let n = out.len().min(self.end - self.first);
                out[..n].copy_from_slice(&self.incoming[self.first..self.first + n]);
                self.incoming[self.first..self.first + n].zeroize();
                self.first += n;
                return Ok(n);
            }
            if self.closed {
                return Ok(0);
            }
            self.receive()?;
        }
    }
}
impl Write for Ktls {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.write_closed {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        self.socket.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.socket.flush()
    }
}
impl Drop for Ktls {
    fn drop(&mut self) {
        self.incoming.zeroize();
        self.handshake.zeroize();
    }
}
