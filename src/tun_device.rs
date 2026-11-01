//! Unix packet descriptors: Linux/Android raw IP and Apple's utun AF prefix.
//! Apple header handling uses readv/writev, preserving packet boundaries without
//! allocating or shifting the packet payload.
use std::{
    fs::File,
    io::{self, Read, Write},
    os::fd::{AsRawFd, RawFd},
};
pub(crate) struct TunDevice(pub File);
impl TunDevice {
    pub(crate) fn new(file: File) -> Self {
        Self(file)
    }
    pub(crate) fn register(
        &self,
        registry: &mio::Registry,
        token: mio::Token,
        interest: mio::Interest,
    ) -> io::Result<()> {
        registry.register(&mut mio::unix::SourceFd(&self.as_raw_fd()), token, interest)
    }
    pub(crate) fn reregister(
        &self,
        registry: &mio::Registry,
        token: mio::Token,
        interest: mio::Interest,
    ) -> io::Result<()> {
        registry.reregister(&mut mio::unix::SourceFd(&self.as_raw_fd()), token, interest)
    }
    pub(crate) fn deregister(&self, registry: &mio::Registry) -> io::Result<()> {
        registry.deregister(&mut mio::unix::SourceFd(&self.as_raw_fd()))
    }
}
impl AsRawFd for TunDevice {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
impl Read for TunDevice {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        #[cfg(not(target_vendor = "apple"))]
        {
            self.0.read(bytes)
        }
        #[cfg(target_vendor = "apple")]
        {
            let mut family = [0; 4];
            let vectors = [
                libc::iovec {
                    iov_base: family.as_mut_ptr().cast(),
                    iov_len: 4,
                },
                libc::iovec {
                    iov_base: bytes.as_mut_ptr().cast(),
                    iov_len: bytes.len(),
                },
            ];
            // SAFETY: both mutable output buffers remain valid and non-overlapping.
            let n = unsafe { libc::readv(self.as_raw_fd(), vectors.as_ptr(), 2) };
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            if n < 5 {
                return Err(io::ErrorKind::InvalidData.into());
            }
            let expected = family_header(bytes[0])?;
            if family != expected {
                return Err(io::ErrorKind::InvalidData.into());
            }
            Ok(n as usize - 4)
        }
    }
}
impl Write for TunDevice {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        #[cfg(not(target_vendor = "apple"))]
        {
            self.0.write(bytes)
        }
        #[cfg(target_vendor = "apple")]
        {
            let mut family = family_header(*bytes.first().ok_or(io::ErrorKind::InvalidInput)?)?;
            let vectors = [
                libc::iovec {
                    iov_base: family.as_mut_ptr().cast(),
                    iov_len: 4,
                },
                libc::iovec {
                    iov_base: bytes.as_ptr().cast_mut().cast(),
                    iov_len: bytes.len(),
                },
            ];
            // SAFETY: writev only reads these live buffers; utun writes one datagram.
            let n = unsafe { libc::writev(self.as_raw_fd(), vectors.as_ptr(), 2) };
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            if n as usize != bytes.len() + 4 {
                return Err(io::ErrorKind::WriteZero.into());
            }
            Ok(bytes.len())
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
#[cfg(any(test, target_vendor = "apple"))]
fn family_header(first: u8) -> io::Result<[u8; 4]> {
    // utun's 32-bit network-order address family uses Apple's constants,
    // regardless of the host on which this codec is tested.
    Ok(match first >> 4 {
        4 => 2u32.to_be_bytes(),
        6 => 30u32.to_be_bytes(),
        _ => return Err(io::ErrorKind::InvalidInput.into()),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apple_family_header_is_network_order_and_rejects_non_ip() {
        assert_eq!(family_header(0x45).unwrap(), [0, 0, 0, 2]);
        assert_eq!(family_header(0x60).unwrap(), [0, 0, 0, 30]);
        assert!(family_header(0x10).is_err());
    }
    #[cfg(target_vendor = "apple")]
    #[test]
    fn apple_vectored_device_roundtrip_preserves_datagrams_and_payloads() {
        use std::{
            os::{fd::OwnedFd, unix::net::UnixDatagram},
            time::Duration,
        };
        let (device, peer) = UnixDatagram::pair().unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
        let owned: OwnedFd = device.into();
        let mut device = TunDevice(File::from(owned));
        for payload in [vec![0x45u8; 20], vec![0x60u8; 40]] {
            let mut expected = family_header(payload[0]).unwrap().to_vec();
            expected.extend_from_slice(&payload);
            peer.send(&expected).unwrap();
            let mut bytes = [0; 128];
            let n = device.read(&mut bytes).unwrap();
            assert_eq!(&bytes[..n], payload);
            assert_eq!(device.write(&payload).unwrap(), payload.len());
            let n = peer.recv(&mut bytes).unwrap();
            assert_eq!(&bytes[..n], expected);
        }
    }
}
