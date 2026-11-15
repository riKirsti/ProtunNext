//! Owned packet transitions: queue rejection recycles, crypto retains ownership.
use crate::{
    Error, Result,
    connection::Observer,
    packet::{PacketPool, PacketQueue},
    path_probe::PathProbe,
    session::{Output, Session},
};

pub(super) fn enqueue(
    pool: &mut PacketPool,
    queue: &mut PacketQueue,
    packet: crate::packet::Packet,
) -> Result<()> {
    if let Err(packet) = queue.push(packet) {
        pool.recycle(packet)?;
        return Err(Error::Full);
    }
    Ok(())
}
pub(super) fn enqueue_network(
    pool: &mut PacketPool,
    queue: &mut PacketQueue,
    packet: crate::packet::Packet,
    observer: Option<&Observer>,
    probe: &mut Option<PathProbe>,
) -> Result<()> {
    if let Some(probe) = probe {
        probe.sent();
    }
    if let Some(observer) = observer {
        observer.capture_wireguard(packet.data(), true);
    }
    enqueue(pool, queue, packet)
}
pub(super) fn encrypt_owned(
    session: &mut Session,
    mut input: crate::packet::Packet,
) -> Result<(Output, crate::packet::Packet)> {
    let length = input.data().len();
    let result = session.encrypt_in_place(input.wireguard_buffer()?, length)?;
    Ok((result, input))
}

pub(super) type OwnedDecryption = (Result<Output>, crate::packet::Packet);
pub(super) fn decrypt_owned(
    session: &mut Session,
    pool: &mut PacketPool,
    input: crate::packet::Packet,
    in_place: bool,
) -> Result<OwnedDecryption> {
    if in_place {
        let mut input = input;
        let result = session.decrypt_data_in_place(input.data_mut());
        if let Ok(Output::Tunnel(n)) = result {
            input.consume(16)?;
            input.set_data_len(n)?;
        }
        return Ok((result, input));
    }
    let capacity = input.data().len().max(148);
    let mut output = pool.take_sized(capacity).ok_or(Error::Full)?;
    // A failed AEAD open is allowed to modify its output. Track it for erasure.
    output.set_len(capacity)?;
    let result = session.decrypt(input.data(), output.writable());
    pool.recycle(input)?;
    Ok((result, output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::tests::{ip_packet, pair};

    #[test]
    fn full_queue_recycles_rejected_plaintext_without_displacing_accepted_packet() {
        let mut pool = PacketPool::new(2, 2048).unwrap();
        let mut queue = PacketQueue::new(1).unwrap();
        let mut first = pool.take().unwrap();
        first.writable()[..4].copy_from_slice(b"keep");
        first.set_len(4).unwrap();
        enqueue(&mut pool, &mut queue, first).unwrap();
        let mut rejected = pool.take().unwrap();
        rejected.writable()[..32].fill(0xa5);
        rejected.set_len(32).unwrap();
        assert!(matches!(
            enqueue(&mut pool, &mut queue, rejected),
            Err(Error::Full)
        ));
        assert_eq!(queue.front().unwrap().data(), b"keep");
        assert_eq!(pool.available(), 1);
        let mut recycled = pool.take().unwrap();
        assert!(recycled.writable()[..32].iter().all(|b| *b == 0));
        pool.recycle(recycled).unwrap();
        pool.recycle(queue.pop().unwrap()).unwrap();
        assert_eq!(pool.available(), 2);
    }

    #[test]
    fn owned_decryption_rejects_tamper_and_replay_and_clears_all_returned_buffers() {
        for in_place in [false, true] {
            let (mut sender, mut receiver) = pair();
            let mut pool = PacketPool::new(2, 2048).unwrap();
            let plain = ip_packet(1280);
            let mut input = pool.take().unwrap();
            input.writable()[..plain.len()].copy_from_slice(&plain);
            input.set_len(plain.len()).unwrap();
            let (Output::Network(n), mut wire) = encrypt_owned(&mut sender, input).unwrap() else {
                panic!("established data encryption");
            };
            wire.set_data_len(n).unwrap();
            let authentic = wire.data().to_vec();
            pool.recycle(wire).unwrap();
            let mut tampered = authentic.clone();
            *tampered.last_mut().unwrap() ^= 1;
            for (bytes, accepted) in [(&tampered, false), (&authentic, true), (&authentic, false)] {
                let mut packet = pool.take().unwrap();
                packet.writable()[..bytes.len()].copy_from_slice(bytes);
                packet.set_len(bytes.len()).unwrap();
                let (result, mut output) =
                    decrypt_owned(&mut receiver, &mut pool, packet, in_place).unwrap();
                if accepted {
                    assert_eq!(result.unwrap(), Output::Tunnel(plain.len()));
                    output.set_data_len(plain.len()).unwrap();
                    assert_eq!(output.data(), plain);
                } else {
                    assert!(result.is_err());
                }
                pool.recycle(output).unwrap();
                assert_eq!(pool.available(), 2);
                let mut all = Vec::new();
                while let Some(mut packet) = pool.take() {
                    assert!(packet.writable().iter().all(|b| *b == 0));
                    all.push(packet);
                }
                for packet in all {
                    pool.recycle(packet).unwrap();
                }
            }
        }
    }

    #[test]
    fn successful_in_place_roundtrip_retains_the_original_plaintext_allocation() {
        let (mut sender, mut receiver) = pair();
        let mut pool = PacketPool::new(2, 2048).unwrap();
        let plain = ip_packet(1280);
        let mut input = pool.take().unwrap();
        input.writable()[..plain.len()].copy_from_slice(&plain);
        input.set_len(plain.len()).unwrap();
        let original = input.data().as_ptr();
        let (Output::Network(n), mut wire) = encrypt_owned(&mut sender, input).unwrap() else {
            panic!("established data encryption");
        };
        wire.set_data_len(n).unwrap();
        let (result, output) = decrypt_owned(&mut receiver, &mut pool, wire, true).unwrap();
        assert_eq!(result.unwrap(), Output::Tunnel(plain.len()));
        assert_eq!(output.data(), plain);
        assert_eq!(output.data().as_ptr(), original);
        pool.recycle(output).unwrap();
        assert_eq!(pool.available(), 2);
    }
}
