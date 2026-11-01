//! Honors metadata on a host-created IFF_VNET_HDR descriptor. No TUNSETIFF,
//! TUNSETOFFLOAD, route, MTU or persistent host configuration writes occur here.
use crate::offload::{self, Header};
use std::{fs::File, io, os::fd::AsRawFd};
use zeroize::Zeroize;
const MAX_GSO_RECORD: usize = 65_536;
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
            // The extra byte detects a truncated oversized record before segmentation.
            packet: vec![0; MAX_GSO_RECORD + 1].into_boxed_slice(),
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
            let length = n as usize - self.header_len;
            if length > MAX_GSO_RECORD {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "TUN GSO record exceeds bounded capacity",
                ));
            }
            self.header = Header::decode(&metadata).map_err(io::Error::other)?;
            if self.header.gso_type == 0 && length > 65_535 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "oversized unsegmented TUN packet",
                ));
            }
            self.len = length;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::{fd::OwnedFd, unix::net::UnixDatagram};

    #[test]
    fn maximum_gso_record_preserves_tail_and_oversized_record_is_rejected() {
        let (sender, receiver) = UnixDatagram::pair().unwrap();
        let file = File::from(OwnedFd::from(receiver));
        let mut reader = Offload {
            header_len: 10,
            packet: vec![0; MAX_GSO_RECORD + 1].into_boxed_slice(),
            len: 0,
            header: Header::default(),
            offset: 0,
        };
        let mut record = vec![0; 10 + MAX_GSO_RECORD];
        record[1] = 1; // TCPv4 GSO.
        record[4..6].copy_from_slice(&1000u16.to_ne_bytes());
        let packet = &mut record[10..];
        packet[0] = 0x45;
        packet[8] = 64;
        packet[9] = 6;
        packet[32] = 0x50;
        packet[33] = 0x18;
        for (index, byte) in packet[40..].iter_mut().enumerate() {
            *byte = (index % 251) as u8;
        }
        sender.send(&record).unwrap();
        let mut output = [0; 2048];
        let mut restored = Vec::new();
        loop {
            let n = reader.read(&file, &mut output).unwrap();
            restored.extend_from_slice(&output[40..n]);
            if reader.len == 0 {
                break;
            }
        }
        assert_eq!(restored, record[50..]);
        record.push(42);
        sender.send(&record).unwrap();
        assert_eq!(
            reader.read(&file, &mut output).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}
