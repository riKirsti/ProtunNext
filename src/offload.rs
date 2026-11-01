//! Linux virtio-net metadata and software segmentation at the TUN boundary.
//! WireGuard still authenticates each resulting IP packet independently.
//! ABI: include/uapi/linux/virtio_net.h; checksum: RFC 1071 / TCP/UDP pseudoheaders.
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, Default)]
pub struct Header {
    pub flags: u8,
    pub gso_type: u8,
    pub header_len: u16,
    pub segment_size: u16,
    pub checksum_start: u16,
    pub checksum_offset: u16,
}
impl Header {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 10 {
            return Err(Error::Invalid("short virtio header"));
        }
        let field = |i| u16::from_ne_bytes([bytes[i], bytes[i + 1]]);
        if bytes[0] & !3 != 0 {
            return Err(Error::Invalid("virtio flags"));
        }
        Ok(Self {
            flags: bytes[0],
            gso_type: bytes[1],
            header_len: field(2),
            segment_size: field(4),
            checksum_start: field(6),
            checksum_offset: field(8),
        })
    }
}
fn sum(mut value: u32, bytes: &[u8]) -> u32 {
    for pair in bytes.chunks(2) {
        value += u16::from_be_bytes([pair[0], *pair.get(1).unwrap_or(&0)]) as u32;
        value = (value & 0xffff) + (value >> 16);
    }
    value
}
fn finish(mut value: u32) -> u16 {
    while value > 0xffff {
        value = (value & 0xffff) + (value >> 16);
    }
    !(value as u16)
}
fn transport(packet: &[u8]) -> Result<(usize, u8)> {
    match packet.first().map(|b| b >> 4) {
        Some(4) if packet.len() >= 20 => {
            let n = (packet[0] & 15) as usize * 4;
            if n < 20 || n > packet.len() || packet[6] & 0x3f != 0 || packet[7] != 0 {
                return Err(Error::Invalid("fragmented or short IPv4 GSO"));
            }
            Ok((n, packet[9]))
        }
        Some(6) if packet.len() >= 40 => {
            let mut protocol = packet[6];
            let mut offset = 40;
            // Hop-by-hop and destination options do not change pseudoheader addresses.
            for _ in 0..8 {
                if !matches!(protocol, 0 | 60) {
                    break;
                }
                if packet.len() < offset + 2 {
                    return Err(Error::Invalid("IPv6 GSO extension"));
                }
                let n = (packet[offset + 1] as usize + 1) * 8;
                protocol = packet[offset];
                offset += n;
                if offset > packet.len() {
                    return Err(Error::Invalid("IPv6 GSO extension size"));
                }
            }
            Ok((offset, protocol))
        }
        _ => Err(Error::Invalid("GSO IP header")),
    }
}
fn pseudo(packet: &[u8], protocol: u8, size: usize) -> u32 {
    if packet[0] >> 4 == 4 {
        sum(size as u32 + protocol as u32, &packet[12..20])
    } else {
        sum(size as u32 + protocol as u32, &packet[8..40])
    }
}

/// Emit one complete IP packet and the next payload offset (None when done).
/// Offset is relative to transport payload, never to untrusted metadata hdr_len.
pub fn segment(
    packet: &[u8],
    header: Header,
    offset: usize,
    output: &mut [u8],
) -> Result<(usize, Option<usize>)> {
    if header.gso_type == 0 {
        if offset != 0 || packet.len() > output.len() {
            return Err(Error::Invalid("unsegmented packet bounds"));
        }
        output[..packet.len()].copy_from_slice(packet);
        if header.flags & 1 != 0 {
            let start = usize::from(header.checksum_start);
            let field = start + usize::from(header.checksum_offset);
            if start >= packet.len() || field + 2 > packet.len() {
                return Err(Error::Invalid("partial checksum bounds"));
            }
            let value = finish(sum(0, &output[start..packet.len()]));
            output[field..field + 2].copy_from_slice(&value.to_be_bytes());
        }
        return Ok((packet.len(), None));
    }
    let kind = header.gso_type & !0x80;
    let (ip_len, protocol) = transport(packet)?;
    let tcp = matches!(kind, 1 | 4);
    if !((kind == 1 && packet[0] >> 4 == 4 && protocol == 6)
        || (kind == 4 && packet[0] >> 4 == 6 && protocol == 6)
        || (kind == 5 && protocol == 17))
    {
        return Err(Error::Invalid("unsupported virtio GSO type"));
    }
    let transport_len = if tcp {
        if packet.len() < ip_len + 20 {
            return Err(Error::Invalid("short TCP GSO"));
        }
        let size = (packet[ip_len + 12] >> 4) as usize * 4;
        if size < 20 || packet[ip_len + 13] & 0x26 != 0 {
            return Err(Error::Invalid("TCP GSO control flags"));
        }
        size
    } else {
        8
    };
    let headers = ip_len + transport_len;
    let segment_size = usize::from(header.segment_size);
    if headers >= packet.len()
        || segment_size == 0
        || offset >= packet.len() - headers
        || !offset.is_multiple_of(segment_size)
        || headers + segment_size > 65535
    {
        return Err(Error::Invalid("GSO payload bounds"));
    }
    let payload = segment_size.min(packet.len() - headers - offset);
    let n = headers + payload;
    if n > output.len() {
        return Err(Error::Full);
    }
    output[..headers].copy_from_slice(&packet[..headers]);
    output[headers..n].copy_from_slice(&packet[headers + offset..headers + offset + payload]);
    let next = offset + payload;
    let last = next == packet.len() - headers;
    if packet[0] >> 4 == 4 {
        output[2..4].copy_from_slice(&(n as u16).to_be_bytes());
        let id =
            u16::from_be_bytes([packet[4], packet[5]]).wrapping_add((offset / segment_size) as u16);
        output[4..6].copy_from_slice(&id.to_be_bytes());
        output[10..12].fill(0);
        let checksum = finish(sum(0, &output[..ip_len]));
        output[10..12].copy_from_slice(&checksum.to_be_bytes());
    } else {
        output[4..6].copy_from_slice(&((n - 40) as u16).to_be_bytes());
    }
    let checksum_offset = if tcp {
        let sequence = u32::from_be_bytes(
            packet[ip_len + 4..ip_len + 8]
                .try_into()
                .expect("checked TCP"),
        );
        output[ip_len + 4..ip_len + 8]
            .copy_from_slice(&sequence.wrapping_add(offset as u32).to_be_bytes());
        if !last {
            output[ip_len + 13] &= !0x09;
        } // PSH/FIN on final segment only.
        if offset != 0 {
            output[ip_len + 13] &= !0x80;
        } // CWR on first segment only.
        16
    } else {
        output[ip_len + 4..ip_len + 6].copy_from_slice(&((n - ip_len) as u16).to_be_bytes());
        6
    };
    output[ip_len + checksum_offset..ip_len + checksum_offset + 2].fill(0);
    let checksum = finish(sum(
        pseudo(output, protocol, n - ip_len),
        &output[ip_len..n],
    ));
    let checksum = if !tcp && checksum == 0 {
        0xffff
    } else {
        checksum
    };
    output[ip_len + checksum_offset..ip_len + checksum_offset + 2]
        .copy_from_slice(&checksum.to_be_bytes());
    Ok((n, (!last).then_some(next)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rfc1071_checksum_vector() {
        assert_eq!(
            finish(sum(0, &[0, 1, 0xf2, 3, 0xf4, 0xf5, 0xf6, 0xf7])),
            0x220d
        );
    }
    #[test]
    fn tcp_segmentation_preserves_payload_sequence_flags_and_valid_checksums() {
        for ipv6 in [false, true] {
            let ip = if ipv6 { 40 } else { 20 };
            let mut packet = vec![0u8; ip + 20 + 2501];
            if ipv6 {
                packet[0] = 0x60;
                packet[6] = 6;
                packet[7] = 64;
            } else {
                packet[0] = 0x45;
                packet[9] = 6;
                packet[8] = 64;
                packet[12..20].copy_from_slice(&[10, 0, 0, 1, 10, 0, 0, 2]);
            }
            packet[ip + 12] = 0x50;
            packet[ip + 13] = 0x99;
            packet[ip + 4..ip + 8].copy_from_slice(&(u32::MAX - 1000).to_be_bytes());
            for (i, b) in packet[ip + 20..].iter_mut().enumerate() {
                *b = (i % 251) as u8;
            }
            let header = Header {
                gso_type: if ipv6 { 4 } else { 1 },
                segment_size: 1000,
                ..Default::default()
            };
            let mut offset = 0;
            let mut restored = Vec::new();
            let mut index = 0;
            loop {
                let mut out = [0; 2048];
                let (n, next) = segment(&packet, header, offset, &mut out).unwrap();
                assert_eq!(finish(sum(pseudo(&out, 6, n - ip), &out[ip..n])), 0);
                if !ipv6 {
                    assert_eq!(finish(sum(0, &out[..ip])), 0);
                }
                assert_eq!(
                    u32::from_be_bytes(out[ip + 4..ip + 8].try_into().unwrap()),
                    (u32::MAX - 1000).wrapping_add(offset as u32)
                );
                assert_eq!(out[ip + 13] & 9, if next.is_none() { 9 } else { 0 });
                assert_eq!(out[ip + 13] & 0x80, if index == 0 { 0x80 } else { 0 });
                restored.extend_from_slice(&out[ip + 20..n]);
                index += 1;
                if let Some(next) = next {
                    offset = next;
                } else {
                    break;
                }
            }
            assert_eq!(index, 3);
            assert_eq!(restored, &packet[ip + 20..]);
        }
    }
    #[test]
    fn rejects_invalid_metadata_and_truncation() {
        assert!(Header::decode(&[0; 9]).is_err());
        let header = Header {
            flags: 1,
            checksum_start: 100,
            ..Default::default()
        };
        assert!(segment(&[0; 20], header, 0, &mut [0; 20]).is_err());
        let header = Header {
            gso_type: 1,
            segment_size: 0,
            ..Default::default()
        };
        assert!(segment(&[0; 20], header, 0, &mut [0; 20]).is_err());
    }
}
