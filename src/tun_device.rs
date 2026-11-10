//! Unix packet descriptors: Linux/Android raw IP and Apple's utun AF prefix.
//! Apple header handling uses readv/writev, preserving packet boundaries without
//! allocating or shifting the packet payload.
use std::{
    fs::File,
    io::{self, Read, Write},
    os::fd::{AsRawFd, RawFd},
};
pub(crate) struct TunDevice {
    file: File,
    #[cfg(target_os = "linux")]
    vnet: bool,
    #[cfg(target_os = "linux")]
    coalescing: bool,
    #[cfg(target_os = "linux")]
    fallbacks: u64,
}
impl TunDevice {
    pub(crate) fn new(file: File) -> io::Result<Self> {
        #[cfg(target_os = "linux")]
        let vnet = vnet_mode(file.as_raw_fd())?;
        Ok(Self {
            file,
            #[cfg(target_os = "linux")]
            vnet,
            #[cfg(target_os = "linux")]
            coalescing: vnet,
            #[cfg(target_os = "linux")]
            fallbacks: 0,
        })
    }
    /// A successful datagram write acknowledges original packets, not bytes.
    pub(crate) fn write_queued(&mut self, queue: &crate::packet::PacketQueue) -> io::Result<usize> {
        #[cfg(target_os = "linux")]
        if self.coalescing
            && queue.len() > 1
            && let Some(mut plan) = crate::tun_coalesce::plan(queue.iter().map(|p| p.data()))
        {
            let mut vectors: [libc::iovec; crate::tun_coalesce::MAX_SEGMENTS + 2] =
                std::array::from_fn(|_| libc::iovec {
                    iov_base: std::ptr::null_mut(),
                    iov_len: 0,
                });
            vectors[0] = libc::iovec {
                iov_base: plan.vnet.as_ptr().cast_mut().cast(),
                iov_len: plan.vnet.len(),
            };
            vectors[1] = libc::iovec {
                iov_base: plan.header.as_ptr().cast_mut().cast(),
                iov_len: plan.header_len,
            };
            for (i, packet) in queue.iter().take(plan.count).enumerate() {
                let payload = &packet.data()[plan.header_len..];
                vectors[i + 2] = libc::iovec {
                    iov_base: payload.as_ptr().cast_mut().cast(),
                    iov_len: payload.len(),
                };
            }
            // SAFETY: live TUN fd; all read-only header/payload allocations and
            // initialized vectors remain borrowed until this synchronous call returns.
            let n =
                unsafe { libc::writev(self.as_raw_fd(), vectors.as_ptr(), (plan.count + 2) as _) };
            // The scatter write borrows original payloads, but copies IP/TCP
            // headers. Preserve plaintext-scratch erasure on every return path.
            crate::packet::erase(&mut plan.header);
            if n >= 0 {
                if n as usize != plan.len + plan.vnet.len() {
                    return Err(io::ErrorKind::WriteZero.into());
                }
                return Ok(plan.count);
            }
            let error = io::Error::last_os_error();
            if matches!(error.raw_os_error(), Some(libc::EINVAL | libc::EOPNOTSUPP)) {
                // These errors reject the whole datagram. Keep its original
                // packets queued, disable aggregation and retry ordinary delivery.
                self.coalescing = false;
                self.fallbacks += 1;
            } else {
                return Err(error);
            }
        }
        let packet = queue.front().ok_or(io::ErrorKind::InvalidInput)?;
        let n = self.write(packet.data())?;
        if n != packet.data().len() {
            return Err(io::ErrorKind::WriteZero.into());
        }
        Ok(1)
    }
    pub(crate) fn offload_fallbacks(&self) -> u64 {
        #[cfg(target_os = "linux")]
        {
            self.fallbacks
        }
        #[cfg(not(target_os = "linux"))]
        {
            0
        }
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
        self.file.as_raw_fd()
    }
}
impl Read for TunDevice {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        #[cfg(target_os = "linux")]
        if self.vnet {
            let mut header = [0u8; 10];
            let vectors = [
                libc::iovec {
                    iov_base: header.as_mut_ptr().cast(),
                    iov_len: header.len(),
                },
                libc::iovec {
                    iov_base: bytes.as_mut_ptr().cast(),
                    iov_len: bytes.len(),
                },
            ];
            // SAFETY: live descriptor and disjoint, valid writable buffers.
            let n = unsafe { libc::readv(self.as_raw_fd(), vectors.as_ptr(), 2) };
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            if n == 0 {
                return Ok(0);
            }
            if n < 10 || header[0] & !2 != 0 || header[1] != 0 || header[4..6] != [0, 0] {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "receive-only TUN requires ordinary fully checksummed kernel output",
                ));
            }
            return Ok(n as usize - 10);
        }
        #[cfg(not(target_vendor = "apple"))]
        {
            self.file.read(bytes)
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
        #[cfg(target_os = "linux")]
        if self.vnet {
            let header = [0u8; 10];
            let vectors = [
                libc::iovec {
                    iov_base: header.as_ptr().cast_mut().cast(),
                    iov_len: header.len(),
                },
                libc::iovec {
                    iov_base: bytes.as_ptr().cast_mut().cast(),
                    iov_len: bytes.len(),
                },
            ];
            // SAFETY: live descriptor and read-only header/payload slices remain
            // valid until writev returns. This is one ordinary TUN datagram.
            let n = unsafe { libc::writev(self.as_raw_fd(), vectors.as_ptr(), 2) };
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            if n as usize != bytes.len() + header.len() {
                return Err(io::ErrorKind::WriteZero.into());
            }
            return Ok(bytes.len());
        }
        #[cfg(not(target_vendor = "apple"))]
        {
            self.file.write(bytes)
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
#[cfg(target_os = "linux")]
pub(crate) fn vnet_mode(fd: RawFd) -> io::Result<bool> {
    let mut info = [0u8; 40];
    // SAFETY: live caller descriptor and a complete mutable Linux ifreq.
    if unsafe { libc::ioctl(fd, libc::TUNGETIFF, info.as_mut_ptr()) } < 0 {
        return Err(io::Error::last_os_error());
    }
    let flags = i16::from_ne_bytes([info[libc::IFNAMSIZ], info[libc::IFNAMSIZ + 1]]) as i32;
    if flags & (libc::IFF_TUN | libc::IFF_NO_PI) != (libc::IFF_TUN | libc::IFF_NO_PI) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "descriptor requires TUN without packet-info headers",
        ));
    }
    if flags & libc::IFF_VNET_HDR == 0 {
        return Ok(false);
    }
    let mut size = 0i32;
    let mut little = 0i32;
    // SAFETY: ioctl outputs point to correctly sized mutable scalar storage.
    if unsafe { libc::ioctl(fd, libc::TUNGETVNETHDRSZ, &mut size) } < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: the second ioctl also receives a complete mutable scalar output.
    if unsafe { libc::ioctl(fd, libc::TUNGETVNETLE, &mut little) } < 0 {
        return Err(io::Error::last_os_error());
    }
    if size != 10 || little != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "prepare the receive-only 10-byte little-endian TUN metadata contract before attaching",
        ));
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_os = "linux")]
    fn fake(vnet: bool) -> (TunDevice, std::os::unix::net::UnixDatagram) {
        use std::os::{fd::OwnedFd, unix::net::UnixDatagram};
        let (socket, peer) = UnixDatagram::pair().unwrap();
        socket.set_nonblocking(true).unwrap();
        peer.set_nonblocking(true).unwrap();
        let owned: OwnedFd = socket.into();
        (
            TunDevice {
                file: File::from(owned),
                vnet,
                coalescing: vnet,
                fallbacks: 0,
            },
            peer,
        )
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn vnet_vectored_batch_is_atomic_and_retains_ownership_on_backpressure() {
        use crate::packet::{PacketPool, PacketQueue};
        for v6 in [false, true] {
            let (mut device, peer) = fake(true);
            socket2::SockRef::from(&device.file)
                .set_send_buffer_size(4096)
                .unwrap();
            let mut pool = PacketPool::new(4, 2048).unwrap();
            let mut queue = PacketQueue::new(4).unwrap();
            let originals = [
                crate::tun_coalesce::test_packet(v6, 0, 1000, false),
                crate::tun_coalesce::test_packet(v6, 1000, 23, true),
            ];
            for bytes in &originals {
                let mut packet = pool.take().unwrap();
                packet.writable()[..bytes.len()].copy_from_slice(bytes);
                packet.set_len(bytes.len()).unwrap();
                assert!(queue.push(packet).is_ok());
            }
            while device.file.write(&[0u8; 1024]).is_ok() {}
            assert!(
                matches!(device.write_queued(&queue), Err(e) if e.kind() == io::ErrorKind::WouldBlock)
            );
            assert_eq!(queue.len(), 2);
            for (packet, original) in queue.iter().zip(&originals) {
                assert_eq!(packet.data(), original);
            }
            let mut bytes = [0u8; 65536];
            while peer.recv(&mut bytes).is_ok() {}
            assert_eq!(device.write_queued(&queue).unwrap(), 2);
            let n = peer.recv(&mut bytes).unwrap();
            let plan = crate::tun_coalesce::plan(originals.iter().map(Vec::as_slice)).unwrap();
            assert_eq!(&bytes[..10], plan.vnet);
            assert_eq!(
                &bytes[10..10 + plan.header_len],
                &plan.header[..plan.header_len]
            );
            let payload: Vec<_> = originals
                .iter()
                .flat_map(|p| p[plan.header_len..].iter().copied())
                .collect();
            assert_eq!(&bytes[10 + plan.header_len..n], payload);
            assert_eq!(
                queue.len(),
                2,
                "only the caller acknowledges accepted packets"
            );
            while let Some(packet) = queue.pop() {
                pool.recycle(packet).unwrap();
            }
            assert_eq!(pool.available(), 4);
        }
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn aggregate_rejection_disables_once_and_keeps_originals_for_ordinary_retry() {
        use crate::packet::{PacketPool, PacketQueue};
        use std::os::fd::FromRawFd;
        // eventfd rejects these non-eight-byte datagrams with EINVAL, providing
        // a real failed syscall without a production fault-injection hook.
        // SAFETY: scalar flags create a new independently owned event counter.
        let fd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) };
        assert!(fd >= 0);
        let mut device = TunDevice {
            // SAFETY: the successful eventfd call returned a new owned fd.
            file: unsafe { File::from_raw_fd(fd) },
            vnet: true,
            coalescing: true,
            fallbacks: 0,
        };
        let originals = [
            crate::tun_coalesce::test_packet(false, 0, 1000, false),
            crate::tun_coalesce::test_packet(false, 1000, 1000, false),
        ];
        let mut pool = PacketPool::new(2, 2048).unwrap();
        let mut queue = PacketQueue::new(2).unwrap();
        for bytes in &originals {
            let mut packet = pool.take().unwrap();
            packet.writable()[..bytes.len()].copy_from_slice(bytes);
            packet.set_len(bytes.len()).unwrap();
            assert!(queue.push(packet).is_ok());
        }
        for _ in 0..2 {
            assert_eq!(
                device.write_queued(&queue).unwrap_err().raw_os_error(),
                Some(libc::EINVAL)
            );
            assert_eq!(device.offload_fallbacks(), 1);
            assert!(!device.coalescing);
            assert_eq!(queue.len(), 2);
            for (packet, original) in queue.iter().zip(&originals) {
                assert_eq!(packet.data(), original);
            }
        }
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn vnet_reads_strip_metadata_and_reject_unnegotiated_tx_offloads() {
        let (mut device, peer) = fake(true);
        let mut frame = vec![0u8; 10];
        frame.extend_from_slice(&[0x45u8; 20]);
        let mut bytes = [0u8; 128];
        peer.send(&frame).unwrap();
        assert_eq!(device.read(&mut bytes).unwrap(), 20);
        assert_eq!(&bytes[..20], &[0x45u8; 20]);
        frame[0] = 1;
        peer.send(&frame).unwrap();
        assert_eq!(
            device.read(&mut bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        peer.send(&frame[..3]).unwrap();
        assert_eq!(
            device.read(&mut bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        let (mut raw, peer) = fake(false);
        assert_eq!(raw.write(&frame[10..]).unwrap(), 20);
        assert_eq!(peer.recv(&mut bytes).unwrap(), 20);
        assert_eq!(&bytes[..20], &frame[10..]);
    }
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
        let mut device = TunDevice::new(File::from(owned)).unwrap();
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
