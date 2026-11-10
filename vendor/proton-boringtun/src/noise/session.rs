// Copyright (c) 2019 Cloudflare, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

use super::PacketData;
use crate::noise::errors::WireGuardError;
use core::sync::atomic::{AtomicU64, Ordering};
use proton_os_interface::lock::{Lock as _, LockFactory};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use std::sync::Arc;

pub struct Session<LF: LockFactory> {
    pub(crate) receiving_index: u32,
    sending_index: u32,
    receiver: Arc<LessSafeKey>,
    sender: LessSafeKey,
    sending_key_counter: AtomicU64,
    receiving_key_counter: LF::Lock<ReceivingKeyCounterValidator>,
}

/// An exclusively owned ciphertext buffer and immutable receiving-key lease.
/// No replay, timer or tunnel state travels to the authentication worker.
#[cfg(feature = "in-place")]
pub struct DataJob<P> {
    packet: P,
    receiver: u32,
    counter: u64,
    key: Arc<LessSafeKey>,
}

/// Owns authenticated bytes until their tunnel owner commits replay state.
/// The buffer cannot be exchanged or accessed before that commit.
#[cfg(feature = "in-place")]
pub struct AuthenticatedData<P> {
    pub(super) packet: P,
    receiver: u32,
    counter: u64,
    key: Arc<LessSafeKey>,
    opened: Option<usize>,
}

#[cfg(feature = "in-place")]
impl<P: AsMut<[u8]>> DataJob<P> {
    pub fn authenticate(mut self) -> AuthenticatedData<P> {
        let mut nonce = [0u8; 12];
        nonce[4..].copy_from_slice(&self.counter.to_le_bytes());
        let opened = self.key.open_in_place(
            Nonce::assume_unique_for_key(nonce), Aad::empty(),
            &mut self.packet.as_mut()[DATA_OFFSET..],
        ).ok().map(|p| p.len());
        AuthenticatedData { packet: self.packet, receiver: self.receiver,
            counter: self.counter, key: self.key, opened }
    }
}

#[cfg(feature = "in-place")]
impl<P> AuthenticatedData<P> {
    pub(super) fn receiver(&self) -> u32 { self.receiver }
}

impl<LF: LockFactory> std::fmt::Debug for Session<LF> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Session: {}<- ->{}",
            self.receiving_index, self.sending_index
        )
    }
}

/// Where encrypted data resides in a data packet
const DATA_OFFSET: usize = 16;
/// The overhead of the AEAD
const AEAD_SIZE: usize = 16;

// Receiving buffer constants
const WORD_SIZE: u64 = 64;
const N_WORDS: u64 = 16; // Suffice to reorder 64*16 = 1024 packets; can be increased at will
const N_BITS: u64 = WORD_SIZE * N_WORDS;

#[derive(Debug, Clone, Default)]
struct ReceivingKeyCounterValidator {
    /// In order to avoid replays while allowing for some reordering of the packets, we keep a
    /// bitmap of received packets, and the value of the highest counter
    next: u64,
    /// Used to estimate packet loss
    receive_cnt: u64,
    bitmap: [u64; N_WORDS as usize],
}

impl ReceivingKeyCounterValidator {
    #[inline(always)]
    fn set_bit(&mut self, idx: u64) {
        let bit_idx = idx % N_BITS;
        let word = (bit_idx / WORD_SIZE) as usize;
        let bit = (bit_idx % WORD_SIZE) as usize;
        self.bitmap[word] |= 1 << bit;
    }

    #[inline(always)]
    fn clear_bit(&mut self, idx: u64) {
        let bit_idx = idx % N_BITS;
        let word = (bit_idx / WORD_SIZE) as usize;
        let bit = (bit_idx % WORD_SIZE) as usize;
        self.bitmap[word] &= !(1u64 << bit);
    }

    /// Clear the word that contains idx
    #[inline(always)]
    fn clear_word(&mut self, idx: u64) {
        let bit_idx = idx % N_BITS;
        let word = (bit_idx / WORD_SIZE) as usize;
        self.bitmap[word] = 0;
    }

    /// Returns true if bit is set, false otherwise
    #[inline(always)]
    fn check_bit(&self, idx: u64) -> bool {
        let bit_idx = idx % N_BITS;
        let word = (bit_idx / WORD_SIZE) as usize;
        let bit = (bit_idx % WORD_SIZE) as usize;
        ((self.bitmap[word] >> bit) & 1) == 1
    }

    /// Returns true if the counter was not yet received, and is not too far back
    #[inline(always)]
    fn will_accept(&self, counter: u64) -> Result<(), WireGuardError> {
        if counter >= self.next {
            // As long as the counter is growing no replay took place for sure
            return Ok(());
        }
        if counter + N_BITS < self.next {
            // Drop if too far back
            return Err(WireGuardError::InvalidCounter);
        }
        if !self.check_bit(counter) {
            Ok(())
        } else {
            Err(WireGuardError::DuplicateCounter)
        }
    }

    /// Marks the counter as received, and returns true if it is still good (in case during
    /// decryption something changed)
    #[inline(always)]
    fn mark_did_receive(&mut self, counter: u64) -> Result<(), WireGuardError> {
        if counter + N_BITS < self.next {
            // Drop if too far back
            return Err(WireGuardError::InvalidCounter);
        }
        if counter == self.next {
            // Usually the packets arrive in order, in that case we simply mark the bit and
            // increment the counter
            self.set_bit(counter);
            self.next += 1;
            return Ok(());
        }
        if counter < self.next {
            // A packet arrived out of order, check if it is valid, and mark
            if self.check_bit(counter) {
                return Err(WireGuardError::InvalidCounter);
            }
            self.set_bit(counter);
            return Ok(());
        }
        // Packets where dropped, or maybe reordered, skip them and mark unused
        if counter - self.next >= N_BITS {
            // Too far ahead, clear all the bits
            for c in self.bitmap.iter_mut() {
                *c = 0;
            }
        } else {
            let mut i = self.next;
            while i % WORD_SIZE != 0 && i < counter {
                // Clear until i aligned to word size
                self.clear_bit(i);
                i += 1;
            }
            while i + WORD_SIZE < counter {
                // Clear whole word at a time
                self.clear_word(i);
                i = (i + WORD_SIZE) & 0u64.wrapping_sub(WORD_SIZE);
            }
            while i < counter {
                // Clear any remaining bits
                self.clear_bit(i);
                i += 1;
            }
        }
        self.set_bit(counter);
        self.next = counter + 1;
        Ok(())
    }
}

impl<LF: LockFactory> Session<LF> {
    pub(super) fn new(
        local_index: u32,
        peer_index: u32,
        receiving_key: [u8; 32],
        sending_key: [u8; 32],
    ) -> Session<LF> {
        Session {
            receiving_index: local_index,
            sending_index: peer_index,
            receiver: Arc::new(LessSafeKey::new(
                UnboundKey::new(&CHACHA20_POLY1305, &receiving_key).unwrap(),
            )),
            sender: LessSafeKey::new(UnboundKey::new(&CHACHA20_POLY1305, &sending_key).unwrap()),
            sending_key_counter: AtomicU64::new(0),
            receiving_key_counter: LF::new(Default::default()),
        }
    }

    pub(super) fn local_index(&self) -> usize {
        self.receiving_index as usize
    }

    #[cfg(feature = "in-place")]
    pub(super) fn prepare_data<P>(&self, receiver: u32, counter: u64, packet: P)
        -> Result<DataJob<P>, (WireGuardError, P)>
    {
        let result = if receiver != self.receiving_index {
            Err(WireGuardError::WrongIndex)
        } else { self.receiving_counter_quick_check(counter) };
        if let Err(e) = result { return Err((e, packet)); }
        Ok(DataJob { packet, receiver, counter, key: Arc::clone(&self.receiver) })
    }

    #[cfg(feature = "in-place")]
    pub(super) fn commit_data<P>(&self, data: &AuthenticatedData<P>)
        -> Result<usize, WireGuardError>
    {
        let len = data.opened.ok_or(WireGuardError::InvalidAeadTag)?;
        if data.receiver != self.receiving_index || !Arc::ptr_eq(&data.key, &self.receiver) {
            return Err(WireGuardError::WrongIndex);
        }
        self.receiving_counter_mark(data.counter)?;
        Ok(len)
    }

    /// Returns true if receiving counter is good to use
    fn receiving_counter_quick_check(&self, counter: u64) -> Result<(), WireGuardError> {
        let counter_validator = self.receiving_key_counter.lock();
        counter_validator.will_accept(counter)
    }

    /// Returns true if receiving counter is good to use, and marks it as used {
    fn receiving_counter_mark(&self, counter: u64) -> Result<(), WireGuardError> {
        let mut counter_validator = self.receiving_key_counter.lock();
        let ret = counter_validator.mark_did_receive(counter);
        if ret.is_ok() {
            counter_validator.receive_cnt += 1;
        }
        ret
    }

    /// src - an IP packet from the interface
    /// dst - pre-allocated space to hold the encapsulating UDP packet to send over the network
    /// returns the size of the formatted packet
    pub(super) fn format_packet_data<'a>(&self, src: &[u8], dst: &'a mut [u8]) -> &'a mut [u8] {
        assert!(dst.len() >= src.len() + super::DATA_OVERHEAD_SZ);
        dst[DATA_OFFSET..DATA_OFFSET + src.len()].copy_from_slice(src);
        self.seal_packet_data(src.len(), dst)
    }

    #[cfg(feature = "in-place")]
    pub(super) fn format_packet_data_in_place<'a>(&self, src_len: usize, dst: &'a mut [u8]) -> &'a mut [u8] {
        self.seal_packet_data(src_len, dst)
    }

    fn seal_packet_data<'a>(&self, src_len: usize, dst: &'a mut [u8]) -> &'a mut [u8] {
        if dst.len() < src_len + super::DATA_OVERHEAD_SZ {
            panic!("The destination buffer is too small");
        }

        let sending_key_counter = self.sending_key_counter.fetch_add(1, Ordering::Relaxed);

        let (message_type, rest) = dst.split_at_mut(4);
        let (receiver_index, rest) = rest.split_at_mut(4);
        let (counter, data) = rest.split_at_mut(8);

        message_type.copy_from_slice(&super::DATA.to_le_bytes());
        receiver_index.copy_from_slice(&self.sending_index.to_le_bytes());
        counter.copy_from_slice(&sending_key_counter.to_le_bytes());

        // TODO: spec requires padding to 16 bytes, but actually works fine without it
        let n = {
            let mut nonce = [0u8; 12];
            nonce[4..12].copy_from_slice(&sending_key_counter.to_le_bytes());
            self.sender
                .seal_in_place_separate_tag(
                    Nonce::assume_unique_for_key(nonce),
                    Aad::from(&[]),
                    &mut data[..src_len],
                )
                .map(|tag| {
                    data[src_len..src_len + AEAD_SIZE].copy_from_slice(tag.as_ref());
                    src_len + AEAD_SIZE
                })
                .unwrap()
        };

        &mut dst[..DATA_OFFSET + n]
    }

    /// packet - a data packet we received from the network
    /// dst - pre-allocated space to hold the encapsulated IP packet, to send to the interface
    ///       dst will always take less space than src
    /// return the size of the encapsulated packet on success
    pub(super) fn receive_packet_data<'a>(
        &self,
        packet: PacketData,
        dst: &'a mut [u8],
    ) -> Result<&'a mut [u8], WireGuardError> {
        let ct_len = packet.encrypted_encapsulated_packet.len();
        if dst.len() < ct_len {
            // This is a very incorrect use of the library, therefore panic and not error
            panic!("The destination buffer is too small");
        }
        if packet.receiver_idx != self.receiving_index {
            return Err(WireGuardError::WrongIndex);
        }
        // Don't reuse counters, in case this is a replay attack we want to quickly check the counter without running expensive decryption
        self.receiving_counter_quick_check(packet.counter)?;

        dst[..ct_len].copy_from_slice(packet.encrypted_encapsulated_packet);
        self.open_packet_data(packet.counter, &mut dst[..ct_len])
    }

    #[cfg(feature = "in-place")]
    pub(super) fn receive_packet_data_in_place<'a>(&self, receiver: u32, counter: u64, dst: &'a mut [u8]) -> Result<&'a mut [u8], WireGuardError> {
        if receiver != self.receiving_index { return Err(WireGuardError::WrongIndex); }
        self.receiving_counter_quick_check(counter)?;
        self.open_packet_data(counter, dst)
    }

    fn open_packet_data<'a>(&self, counter: u64, dst: &'a mut [u8]) -> Result<&'a mut [u8], WireGuardError> {
        let ret = {
            let mut nonce = [0u8; 12];
            nonce[4..12].copy_from_slice(&counter.to_le_bytes());
            self.receiver
                .open_in_place(
                    Nonce::assume_unique_for_key(nonce),
                    Aad::from(&[]),
                    dst,
                )
                .map_err(|_| WireGuardError::InvalidAeadTag)?
        };

        // After decryption is done, check counter again, and mark as received
        self.receiving_counter_mark(counter)?;
        Ok(ret)
    }

    /// Returns the estimated downstream packet loss for this session
    pub(super) fn current_packet_cnt(&self) -> (u64, u64) {
        let counter_validator = self.receiving_key_counter.lock();
        (counter_validator.next, counter_validator.receive_cnt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "in-place")]
    #[test]
    fn copy_and_in_place_emit_identical_wire_bytes_and_commit_replay_only_after_authentication() {
        use core::convert::TryInto;
        type S = Session<proton_os_interface::lock::Loose>;
        let copy_sender = S::new(7, 9, [1;32], [2;32]);
        let inplace_sender = S::new(7, 9, [1;32], [2;32]);
        let copy_receiver = S::new(9, 7, [2;32], [1;32]);
        let inplace_receiver = S::new(9, 7, [2;32], [1;32]);
        for size in [0,20,64,1280,1420,16_000,65_503] {
            let body: Vec<u8> = (0..size).map(|n| (n%251) as u8).collect();
            let mut copy = vec![0;size+32];
            let mut inplace = vec![0;size+32];
            inplace[16..16+size].copy_from_slice(&body);
            copy_sender.format_packet_data(&body,&mut copy);
            inplace_sender.format_packet_data_in_place(size,&mut inplace);
            assert_eq!(copy,inplace,"wire bytes differ for {size}");
            let counter = u64::from_le_bytes(copy[8..16].try_into().unwrap());
            let packet = PacketData { receiver_idx:9, counter, encrypted_encapsulated_packet:&copy[16..] };
            let mut plain = vec![0;size+16];
            assert_eq!(copy_receiver.receive_packet_data(packet,&mut plain).unwrap(),body);
            assert_eq!(inplace_receiver.receive_packet_data_in_place(9,counter,&mut inplace[16..]).unwrap(),body);
        }
        let mut valid=vec![0;96];valid[16..80].fill(7);
        inplace_sender.format_packet_data_in_place(64,&mut valid);
        let counter=u64::from_le_bytes(valid[8..16].try_into().unwrap());
        let mut bad=valid.clone();bad[95]^=1;
        let before=inplace_receiver.current_packet_cnt();
        assert!(inplace_receiver.receive_packet_data_in_place(9,counter,&mut bad[16..]).is_err());
        assert_eq!(before,inplace_receiver.current_packet_cnt());
        let mut replay=valid.clone();
        assert!(inplace_receiver.receive_packet_data_in_place(9,counter,&mut valid[16..]).is_ok());
        assert!(inplace_receiver.receive_packet_data_in_place(9,counter,&mut replay[16..]).is_err());
    }
    #[test]
    fn test_replay_counter() {
        let mut c: ReceivingKeyCounterValidator = Default::default();

        assert!(c.mark_did_receive(0).is_ok());
        assert!(c.mark_did_receive(0).is_err());
        assert!(c.mark_did_receive(1).is_ok());
        assert!(c.mark_did_receive(1).is_err());
        assert!(c.mark_did_receive(63).is_ok());
        assert!(c.mark_did_receive(63).is_err());
        assert!(c.mark_did_receive(15).is_ok());
        assert!(c.mark_did_receive(15).is_err());

        for i in 64..N_BITS + 128 {
            assert!(c.mark_did_receive(i).is_ok());
            assert!(c.mark_did_receive(i).is_err());
        }

        assert!(c.mark_did_receive(N_BITS * 3).is_ok());
        for i in 0..=N_BITS * 2 {
            assert!(matches!(
                c.will_accept(i),
                Err(WireGuardError::InvalidCounter)
            ));
            assert!(c.mark_did_receive(i).is_err());
        }
        for i in N_BITS * 2 + 1..N_BITS * 3 {
            assert!(c.will_accept(i).is_ok());
        }
        assert!(matches!(
            c.will_accept(N_BITS * 3),
            Err(WireGuardError::DuplicateCounter)
        ));

        for i in (N_BITS * 2 + 1..N_BITS * 3).rev() {
            assert!(c.mark_did_receive(i).is_ok());
            assert!(c.mark_did_receive(i).is_err());
        }

        assert!(c.mark_did_receive(N_BITS * 3 + 70).is_ok());
        assert!(c.mark_did_receive(N_BITS * 3 + 71).is_ok());
        assert!(c.mark_did_receive(N_BITS * 3 + 72).is_ok());
        assert!(c.mark_did_receive(N_BITS * 3 + 72 + 125).is_ok());
        assert!(c.mark_did_receive(N_BITS * 3 + 63).is_ok());

        assert!(c.mark_did_receive(N_BITS * 3 + 70).is_err());
        assert!(c.mark_did_receive(N_BITS * 3 + 71).is_err());
        assert!(c.mark_did_receive(N_BITS * 3 + 72).is_err());
    }
}
