//! Honors metadata on a host-created IFF_VNET_HDR descriptor. No TUNSETIFF,
//! TUNSETOFFLOAD, route, MTU or persistent host configuration writes occur here.
use crate::offload::{self, Header};
use std::{fs::File, io, os::fd::AsRawFd};
use zeroize::Zeroize;
pub(crate) struct Offload {
    pub(crate) header_len: usize,
    packet: Box<[u8]>,
    len: usize,
    header: Header,
    offset: usize,
}
impl Offload {
    pub(crate) fn for_file(file: &File) -> io::Result<Option<Self>> {
        let mut flags = [0u8; 40];
        // SAFETY: bounded writable ifreq storage and a live descriptor.
        if unsafe { libc::ioctl(file.as_raw_fd(), libc::TUNGETIFF, flags.as_mut_ptr()) } < 0 {
            return Ok(None); // Non-TUN test streams retain their existing raw behavior.
        }
        let flags = i16::from_ne_bytes([flags[libc::IFNAMSIZ], flags[libc::IFNAMSIZ + 1]]) as i32;
        if flags & libc::IFF_VNET_HDR == 0 {
            return Ok(None);
        }
        let mut size: libc::c_int = 0;
        // SAFETY: live TUN and correctly sized mutable scalar output.
        if unsafe { libc::ioctl(file.as_raw_fd(), libc::TUNGETVNETHDRSZ, &mut size) } < 0 {
            return Err(io::Error::last_os_error());
        }
        if !matches!(size, 10 | 12) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unsupported TUN vnet header size",
            ));
        }
        Ok(Some(Self {
            header_len: size as usize,
            packet: vec![0; 65535].into_boxed_slice(),
            len: 0,
            header: Header::default(),
            offset: 0,
        }))
    }
    pub(crate) fn read(&mut self, file: &File, output: &mut [u8]) -> io::Result<usize> {
        if self.len == 0 {
            let mut metadata = [0u8; 12];
            let vectors = [
                libc::iovec {
                    iov_base: metadata.as_mut_ptr().cast(),
                    iov_len: self.header_len,
                },
                libc::iovec {
                    iov_base: self.packet.as_mut_ptr().cast(),
                    iov_len: self.packet.len(),
                },
            ];
            // SAFETY: both disjoint output buffers live through the synchronous TUN read.
            let n = unsafe { libc::readv(file.as_raw_fd(), vectors.as_ptr(), 2) };
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            if n == 0 {
                return Ok(0);
            }
            if n as usize <= self.header_len {
                return Err(io::ErrorKind::InvalidData.into());
            }
            self.header = Header::decode(&metadata).map_err(io::Error::other)?;
            self.len = n as usize - self.header_len;
            self.offset = 0;
        }
        let (n, next) =
            offload::segment(&self.packet[..self.len], self.header, self.offset, output)
                .map_err(io::Error::other)?;
        if let Some(offset) = next {
            self.offset = offset;
        } else {
            self.packet[..self.len].zeroize();
            self.len = 0;
        }
        Ok(n)
    }
    pub(crate) fn write(&self, file: &File, bytes: &[u8]) -> io::Result<usize> {
        let mut metadata = [0u8; 12];
        let vectors = [
            libc::iovec {
                iov_base: metadata.as_mut_ptr().cast(),
                iov_len: self.header_len,
            },
            libc::iovec {
                iov_base: bytes.as_ptr().cast_mut().cast(),
                iov_len: bytes.len(),
            },
        ];
        // SAFETY: writev reads live header/payload slices and injects one IP packet.
        let n = unsafe { libc::writev(file.as_raw_fd(), vectors.as_ptr(), 2) };
        if n < 0 {
            return Err(io::Error::last_os_error());
        }
        if n as usize != bytes.len() + self.header_len {
            return Err(io::ErrorKind::WriteZero.into());
        }
        Ok(bytes.len())
    }
}
impl Drop for Offload {
    fn drop(&mut self) {
        self.packet.zeroize();
    }
}
