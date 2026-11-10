//! Conservative TCP receive coalescing for a negotiated 10-byte virtio TUN.
//! Only adjacent, checksum-valid segments are combined. Original packets stay
//! immutable and owned by the queue until the atomic TUN write succeeds.

pub(crate) const MAX_SEGMENTS: usize = 32;
pub(crate) const VNET_LEN: usize = 10;
#[cfg(all(test, target_os = "linux"))]
pub(crate) use tests::packet as test_packet;

pub(crate) struct Plan {
    pub header: [u8; 100],
    pub vnet: [u8; VNET_LEN],
    pub header_len: usize,
    pub count: usize,
    pub len: usize,
}

struct Segment<'a> {
    bytes: &'a [u8],
    ip: usize,
    header: usize,
    payload: usize,
    sequence: u32,
    push: bool,
}

fn word(bytes: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([bytes[at], bytes[at + 1]])
}

fn sum(bytes: &[u8]) -> u64 {
    let (pairs, tail) = bytes.as_chunks::<2>();
    let mut total = pairs
        .iter()
        .map(|p| u16::from_be_bytes([p[0], p[1]]) as u64)
        .sum();
    if let Some(&last) = tail.first() {
        total += (last as u64) << 8;
    }
    total
}

fn fold(mut value: u64) -> u16 {
    while value >> 16 != 0 {
        value = (value & 0xffff) + (value >> 16);
    }
    value as u16
}

fn pseudo(bytes: &[u8], ip: usize, tcp_len: usize) -> u64 {
    sum(if ip == 20 {
        &bytes[12..20]
    } else {
        &bytes[8..40]
    }) + 6
        + tcp_len as u64
}

fn segment(bytes: &[u8]) -> Option<Segment<'_>> {
    let ip = match bytes.first()? >> 4 {
        4 if bytes.len() >= 40
            && bytes[0] == 0x45
            && bytes[1] & 3 == 0
            && word(bytes, 2) as usize == bytes.len()
            && word(bytes, 6) == 0x4000
            && bytes[9] == 6 =>
        {
            20
        }
        6 if bytes.len() >= 60
            && bytes[1] & 0x30 == 0
            && word(bytes, 4) as usize + 40 == bytes.len()
            && bytes[6] == 6 =>
        {
            40
        }
        _ => return None,
    };
    let tcp = &bytes[ip..];
    let tcp_len = (tcp[12] >> 4) as usize * 4;
    let header = ip + tcp_len;
    if tcp_len < 20
        || header >= bytes.len()
        || tcp[12] & 15 != 0
        || tcp[13] & !0x08 != 0x10
        || word(tcp, 18) != 0
    {
        return None;
    }
    Some(Segment {
        bytes,
        ip,
        header,
        payload: bytes.len() - header,
        sequence: u32::from_be_bytes(tcp[4..8].try_into().ok()?),
        push: tcp[13] & 8 != 0,
    })
}

fn valid_checksum(packet: &Segment<'_>) -> bool {
    (packet.ip != 20 || fold(sum(&packet.bytes[..20])) == 0xffff)
        && fold(
            pseudo(packet.bytes, packet.ip, packet.bytes.len() - packet.ip)
                + sum(&packet.bytes[packet.ip..]),
        ) == 0xffff
}

fn compatible(first: &Segment<'_>, next: &Segment<'_>) -> bool {
    if first.ip != next.ip || first.header != next.header {
        return false;
    }
    let (a, b) = (first.bytes, next.bytes);
    let same_ip = if first.ip == 20 {
        a[..2] == b[..2] && a[6..10] == b[6..10] && a[12..20] == b[12..20]
    } else {
        a[..4] == b[..4] && a[6..40] == b[6..40]
    };
    let (a, b) = (&a[first.ip..first.header], &b[next.ip..next.header]);
    same_ip
        && a[..4] == b[..4]
        && a[8..13] == b[8..13]
        && a[14..16] == b[14..16]
        && a[20..] == b[20..]
}

pub(crate) fn plan<'a>(mut packets: impl Iterator<Item = &'a [u8]>) -> Option<Plan> {
    let first = segment(packets.next()?)?;
    if first.push {
        return None;
    }
    let mut count = 1;
    let mut payload = first.payload;
    let mut push = false;
    for bytes in packets.take(MAX_SEGMENTS - 1) {
        let Some(next) = segment(bytes) else { break };
        if !compatible(&first, &next)
            || next.sequence != first.sequence.wrapping_add(payload as u32)
            || next.payload > first.payload
            || first.header + payload + next.payload > u16::MAX as usize
        {
            break;
        }
        if (count == 1 && !valid_checksum(&first)) || !valid_checksum(&next) {
            break;
        }
        payload += next.payload;
        count += 1;
        push = next.push;
        if push || next.payload < first.payload {
            break;
        }
    }
    if count == 1 {
        return None;
    }
    let len = first.header + payload;
    let mut result = Plan {
        header: [0; 100],
        vnet: [0; VNET_LEN],
        header_len: first.header,
        count,
        len,
    };
    result.header[..first.header].copy_from_slice(&first.bytes[..first.header]);
    let offset = if first.ip == 20 { 2 } else { 4 };
    let ip_len = if first.ip == 20 { len } else { len - 40 };
    result.header[offset..offset + 2].copy_from_slice(&(ip_len as u16).to_be_bytes());
    if first.ip == 20 {
        result.header[10..12].fill(0);
        let check = !fold(sum(&result.header[..20]));
        result.header[10..12].copy_from_slice(&check.to_be_bytes());
    }
    result.header[first.ip + 13] = 0x10 | if push { 8 } else { 0 };
    let check = fold(pseudo(&result.header, first.ip, len - first.ip));
    result.header[first.ip + 16..first.ip + 18].copy_from_slice(&check.to_be_bytes());
    result.vnet[0] = 1; // VIRTIO_NET_HDR_F_NEEDS_CSUM
    result.vnet[1] = if first.ip == 20 { 1 } else { 4 }; // TCPv4 / TCPv6
    result.vnet[2..4].copy_from_slice(&(first.header as u16).to_le_bytes());
    result.vnet[4..6].copy_from_slice(&(first.payload as u16).to_le_bytes());
    result.vnet[6..8].copy_from_slice(&(first.ip as u16).to_le_bytes());
    result.vnet[8..10].copy_from_slice(&16u16.to_le_bytes());
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn packet(v6: bool, sequence: u32, payload: usize, push: bool) -> Vec<u8> {
        let ip = if v6 { 40 } else { 20 };
        let mut bytes = vec![0; ip + 32 + payload];
        let len = bytes.len();
        if v6 {
            bytes[0] = 0x60;
            bytes[4..6].copy_from_slice(&((len - 40) as u16).to_be_bytes());
            bytes[6] = 6;
            bytes[7] = 64;
            bytes[23] = 1;
            bytes[39] = 2;
        } else {
            bytes[0] = 0x45;
            bytes[2..4].copy_from_slice(&(len as u16).to_be_bytes());
            bytes[6] = 0x40;
            bytes[8] = 64;
            bytes[9] = 6;
            bytes[12..16].copy_from_slice(&[192, 0, 2, 1]);
            bytes[16..20].copy_from_slice(&[192, 0, 2, 2]);
            let check = !fold(sum(&bytes[..20]));
            bytes[10..12].copy_from_slice(&check.to_be_bytes());
        }
        bytes[ip..ip + 4].copy_from_slice(&[1, 2, 3, 4]);
        bytes[ip + 4..ip + 8].copy_from_slice(&sequence.to_be_bytes());
        bytes[ip + 8..ip + 12].copy_from_slice(&7u32.to_be_bytes());
        bytes[ip + 12] = 0x80;
        bytes[ip + 13] = 0x10 | if push { 8 } else { 0 };
        bytes[ip + 14..ip + 16].copy_from_slice(&32000u16.to_be_bytes());
        bytes[ip + 20..ip + 32].copy_from_slice(&[1, 1, 8, 10, 0, 0, 0, 1, 0, 0, 0, 2]);
        bytes[ip + 32..].fill(sequence as u8);
        let check = !fold(pseudo(&bytes, ip, len - ip) + sum(&bytes[ip..]));
        bytes[ip + 16..ip + 18].copy_from_slice(&check.to_be_bytes());
        bytes
    }

    #[test]
    fn ipv4_ipv6_metadata_checksums_short_tail_push_and_sequence_wrap() {
        for v6 in [false, true] {
            let start = u32::MAX - 699;
            let packets = [
                packet(v6, start, 1000, false),
                packet(v6, start.wrapping_add(1000), 1000, false),
                packet(v6, start.wrapping_add(2000), 123, true),
            ];
            let originals = packets.clone();
            let p = plan(packets.iter().map(Vec::as_slice)).unwrap();
            let ip = if v6 { 40 } else { 20 };
            assert_eq!(p.count, 3);
            assert_eq!(p.len, ip + 32 + 2123);
            assert_eq!(p.header_len, ip + 32);
            assert_eq!(p.header[ip + 13], 0x18);
            assert_eq!(p.vnet[0], 1);
            assert_eq!(p.vnet[1], if v6 { 4 } else { 1 });
            assert_eq!(
                word(&p.header, ip + 16),
                fold(pseudo(&p.header, ip, p.len - ip))
            );
            assert_eq!(u16::from_le_bytes(p.vnet[4..6].try_into().unwrap()), 1000);
            assert_eq!(
                packets, originals,
                "planning must not mutate queued packets"
            );
            if !v6 {
                assert_eq!(fold(sum(&p.header[..20])), 0xffff);
            }
        }
    }

    #[test]
    fn checksum_errors_and_header_changes_are_not_laundered() {
        for v6 in [false, true] {
            let ip = if v6 { 40 } else { 20 };
            let first = packet(v6, 0, 1000, false);
            let next = packet(v6, 1000, 1000, false);
            for offset in [
                ip,
                ip + 4,
                ip + 8,
                ip + 12,
                ip + 13,
                ip + 14,
                ip + 20,
                ip + 32,
            ] {
                let mut changed = next.clone();
                changed[offset] ^= 1;
                assert!(plan([first.as_slice(), changed.as_slice()].into_iter()).is_none());
            }
            let mut broken_first = first.clone();
            broken_first[ip + 32] ^= 1;
            assert!(plan([broken_first.as_slice(), next.as_slice()].into_iter()).is_none());
            if !v6 {
                let mut broken_ip = first.clone();
                broken_ip[10] ^= 1;
                assert!(plan([broken_ip.as_slice(), next.as_slice()].into_iter()).is_none());
            }
        }
    }

    fn rechecksum(bytes: &mut [u8], ip: usize) {
        bytes[ip + 16..ip + 18].fill(0);
        let check = !fold(pseudo(bytes, ip, bytes.len() - ip) + sum(&bytes[ip..]));
        bytes[ip + 16..ip + 18].copy_from_slice(&check.to_be_bytes());
        if ip == 20 {
            bytes[10..12].fill(0);
            let check = !fold(sum(&bytes[..20]));
            bytes[10..12].copy_from_slice(&check.to_be_bytes());
        }
    }

    #[test]
    fn valid_checksum_changes_still_end_a_group_at_the_first_boundary() {
        for v6 in [false, true] {
            let ip = if v6 { 40 } else { 20 };
            let first = packet(v6, 0, 1000, false);
            let next = packet(v6, 1000, 1000, false);
            let mut offsets = vec![ip, ip + 4, ip + 8, ip + 12, ip + 14, ip + 20];
            offsets.extend(if v6 {
                vec![1, 7, 23, 39]
            } else {
                vec![1, 6, 8, 12, 16]
            });
            for offset in offsets {
                let mut changed = next.clone();
                changed[offset] ^= 1;
                rechecksum(&mut changed, ip);
                assert!(plan([first.as_slice(), changed.as_slice()].into_iter()).is_none());
            }
            for flags in [0, 0x01, 0x02, 0x04, 0x11, 0x30, 0x50, 0x90] {
                let mut changed = next.clone();
                changed[ip + 13] = flags;
                rechecksum(&mut changed, ip);
                assert!(plan([first.as_slice(), changed.as_slice()].into_iter()).is_none());
            }
            let mut damaged = packet(v6, 2000, 1000, false);
            damaged[ip + 32] ^= 1;
            let packets = [first, next, damaged];
            assert_eq!(plan(packets.iter().map(Vec::as_slice)).unwrap().count, 2);
        }
    }

    #[test]
    fn boundaries_control_packets_and_segment_limits_preserve_order() {
        for v6 in [false, true] {
            let packets: Vec<_> = (0..64).map(|i| packet(v6, i * 1000, 1000, false)).collect();
            assert_eq!(
                plan(packets.iter().map(Vec::as_slice)).unwrap().count,
                MAX_SEGMENTS
            );
            assert!(
                plan([packet(v6, 0, 1000, true).as_slice(), packets[1].as_slice()].into_iter())
                    .is_none()
            );
            let short = packet(v6, 1000, 20, false);
            assert_eq!(
                plan(
                    [
                        packets[0].as_slice(),
                        short.as_slice(),
                        packets[2].as_slice()
                    ]
                    .into_iter()
                )
                .unwrap()
                .count,
                2
            );
            let large = [
                packet(v6, 0, 16000, false),
                packet(v6, 16000, 16000, false),
                packet(v6, 32000, 16000, false),
                packet(v6, 48000, 16000, false),
                packet(v6, 64000, 2000, true),
            ];
            assert_eq!(plan(large.iter().map(Vec::as_slice)).unwrap().count, 4);
        }
    }
}
