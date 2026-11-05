//! Archived-source experiment: two persistent workers, at most 63 owned packets.
//! The owner waits for a bounded batch, then commits and delivers in input order.
//! This deliberately measures a simple batch barrier before attempting an async pipeline.
use crate::{
    Error, Result,
    packet::Packet,
    session::{Output, Session},
};
use proton_boringtun::noise::{DataCompletion, DataJob};
use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread::{self, JoinHandle},
};

struct Work {
    order: usize,
    packet: Packet,
    job: DataJob,
    transmit: bool,
}
struct Done {
    order: usize,
    packet: Packet,
    completion: DataCompletion,
    transmit: bool,
}
pub(crate) struct Pool {
    senders: Vec<SyncSender<Work>>,
    receive: Receiver<Done>,
    workers: Vec<JoinHandle<()>>,
    slots: [Option<Done>; 64],
    results: Vec<(Result<Output>, Packet)>,
}
impl Pool {
    pub(crate) fn new() -> Result<Self> {
        let (done_tx, receive) = mpsc::sync_channel(64);
        let mut result = Self {
            senders: Vec::with_capacity(2),
            receive,
            workers: Vec::with_capacity(2),
            slots: std::array::from_fn(|_| None),
            results: Vec::with_capacity(64),
        };
        for index in 0..2 {
            let (sender, jobs) = mpsc::sync_channel::<Work>(32);
            let done = done_tx.clone();
            let worker = thread::Builder::new()
                .name(format!("next-crypto-{index}"))
                .spawn(move || {
                    while let Ok(mut work) = jobs.recv() {
                        let completion = work.job.run(work.packet.data_mut());
                        if done
                            .send(Done {
                                order: work.order,
                                packet: work.packet,
                                completion,
                                transmit: work.transmit,
                            })
                            .is_err()
                        {
                            break;
                        }
                    }
                })
                .map_err(Error::from)?;
            result.senders.push(sender);
            result.workers.push(worker);
        }
        Ok(result)
    }
    /// Input/output slots and channels are preallocated. Caller drains all results.
    pub(crate) fn batch(
        &mut self,
        session: &mut Session,
        input: &mut Vec<Packet>,
        transmit: bool,
    ) -> Result<&mut Vec<(Result<Output>, Packet)>> {
        assert!(input.len() <= 63 && self.results.is_empty());
        let count = input.len();
        let mut pending = 0;
        // At an established session every item is a detached job. On the TX cold
        // path retain the original handshake queue API, without changing ordering.
        let mut inline = Vec::new();
        for (order, mut packet) in input.drain(..).enumerate() {
            let length = packet.data().len();
            let job = if transmit {
                let buffer = packet.wireguard_buffer()?;
                match session.tunnel.prepare_data_transmit(length) {
                    Ok(Some(job)) => Ok(job),
                    Ok(None) => {
                        let output =
                            Session::output(session.tunnel.encapsulate_in_place(buffer, length));
                        inline.push((order, output, packet));
                        continue;
                    }
                    Err(e) => Err(e),
                }
            } else {
                session.tunnel.prepare_data_receive(packet.data())
            };
            match job {
                Ok(job) => {
                    self.senders[order % 2]
                        .send(Work {
                            order,
                            packet,
                            job,
                            transmit,
                        })
                        .map_err(|_| Error::Invalid("crypto worker stopped"))?;
                    pending += 1;
                }
                Err(e) => inline.push((
                    order,
                    Err(Error::Protocol(format!("WireGuard: {e:?}"))),
                    packet,
                )),
            }
        }
        for _ in 0..pending {
            let done = self
                .receive
                .recv()
                .map_err(|_| Error::Invalid("crypto completion stopped"))?;
            let order = done.order;
            assert!(self.slots[order].is_none());
            self.slots[order] = Some(done);
        }
        let mut inline = inline.into_iter().peekable();
        for order in 0..count {
            if let Some(mut done) = self.slots[order].take() {
                let result = Session::output(
                    session
                        .tunnel
                        .complete_data(done.completion, done.packet.data_mut()),
                );
                if !done.transmit
                    && let Ok(Output::Tunnel(n)) = result
                {
                    done.packet.consume(16)?;
                    done.packet.set_data_len(n)?;
                }
                self.results.push((result, done.packet));
            } else {
                let (slot, result, packet) =
                    inline.next().expect("every batch slot returns ownership");
                assert_eq!(slot, order);
                self.results.push((result, packet));
            }
        }
        Ok(&mut self.results)
    }
}
impl Drop for Pool {
    fn drop(&mut self) {
        self.senders.clear();
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        packet::PacketPool,
        session::tests::{ip_packet, pair},
    };
    #[test]
    fn full_batch_preserves_nonce_and_delivery_order_and_standard_peer_interop() {
        let (mut a, mut b) = pair();
        let mut buffers = PacketPool::new(63, 65535).unwrap();
        let mut pool = Pool::new().unwrap();
        let mut input = Vec::with_capacity(63);
        let mut previous = None;
        for size in [64, 1280, 65503] {
            for order in 0..63 {
                let mut packet = buffers.take().unwrap();
                let mut ip = ip_packet(size);
                ip[size - 1] = order;
                packet.writable()[..size].copy_from_slice(&ip);
                packet.set_len(size).unwrap();
                input.push(packet);
            }
            for (order, (result, packet)) in pool
                .batch(&mut a, &mut input, true)
                .unwrap()
                .drain(..)
                .enumerate()
            {
                assert_eq!(result.unwrap(), Output::Network(size + 32));
                let counter = u64::from_le_bytes(packet.data()[8..16].try_into().unwrap());
                if let Some(prev) = previous {
                    assert_eq!(counter, prev + 1);
                }
                previous = Some(counter);
                let mut plain = vec![0; (size + 32).max(148)];
                assert_eq!(
                    b.decrypt(packet.data(), &mut plain).unwrap(),
                    Output::Tunnel(size)
                );
                assert_eq!(plain[size - 1], order as u8);
                assert!(b.decrypt(packet.data(), &mut plain).is_err());
                buffers.recycle(packet).unwrap();
            }
            assert!(input.is_empty());
            assert_eq!(buffers.available(), 63);
        }
    }
    #[test]
    fn duplicate_jobs_commit_only_once_and_tamper_does_not_poison_replay() {
        let (mut a, mut b) = pair();
        let ip = ip_packet(1280);
        let mut wire = vec![0; 1312];
        assert_eq!(a.encrypt(&ip, &mut wire).unwrap(), Output::Network(1312));
        let first = b.tunnel.prepare_data_receive(&wire).unwrap();
        let duplicate = b.tunnel.prepare_data_receive(&wire).unwrap();
        let bad_job = b.tunnel.prepare_data_receive(&wire).unwrap();
        let mut bad = wire.clone();
        bad[1311] ^= 1;
        let completed = bad_job.run(&mut bad);
        assert!(Session::output(b.tunnel.complete_data(completed, &mut bad)).is_err());
        let mut second = wire.clone();
        let second_done = duplicate.run(&mut second); // finish out of order
        let first_done = first.run(&mut wire);
        assert_eq!(
            Session::output(b.tunnel.complete_data(first_done, &mut wire)).unwrap(),
            Output::Tunnel(1280)
        );
        assert!(Session::output(b.tunnel.complete_data(second_done, &mut second)).is_err());
    }
    #[test]
    fn expired_session_cannot_commit_detached_plaintext() {
        let (mut a, mut b) = pair();
        let mut wire = vec![0; 1312];
        a.encrypt(&ip_packet(1280), &mut wire).unwrap();
        let job = b.tunnel.prepare_data_receive(&wire).unwrap();
        let completed = job.run(&mut wire);
        let _ = b.advance(std::time::Duration::from_secs(181), &mut [0; 2048]);
        assert!(Session::output(b.tunnel.complete_data(completed, &mut wire)).is_err());
    }
    #[test]
    fn shutdown_joins_workers_with_owned_work_in_flight() {
        let (a, _) = pair();
        let mut buffers = PacketPool::new(63, 2048).unwrap();
        let pool = Pool::new().unwrap();
        for order in 0..63 {
            let mut packet = buffers.take().unwrap();
            packet.writable()[..1280].copy_from_slice(&ip_packet(1280));
            packet.set_len(1280).unwrap();
            packet.wireguard_buffer().unwrap();
            let job = a.tunnel.prepare_data_transmit(1280).unwrap().unwrap();
            pool.senders[order % 2]
                .send(Work {
                    order,
                    packet,
                    job,
                    transmit: true,
                })
                .ok()
                .unwrap();
        }
        drop(pool); // completion capacity covers the entire bounded batch
    }
}
