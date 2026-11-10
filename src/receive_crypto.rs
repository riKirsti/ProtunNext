//! Bounded parallel authentication with ordered, owner-only replay commits.
//! One outstanding helper batch; no additional packet buffers or payload copies.
use crate::{
    Error, Result,
    network::Network,
    packet::{Packet, PacketPool},
    session::{Output, Session},
    stream::Receive,
    transport::Counters,
};
use proton_boringtun::noise::{AuthenticatedData, DataJob};
use std::{collections::VecDeque, sync::mpsc, thread};

const MAX_BATCH: usize = 32;
const MIN_PARALLEL: usize = 8;

enum Item {
    Raw(Packet),
    Job(DataJob<Packet>),
    Opened(AuthenticatedData<Packet>),
    Rejected(Error, Packet),
    Empty,
}
impl Item {
    fn authenticate(&mut self) {
        *self = match std::mem::replace(self, Self::Empty) {
            Self::Job(job) => Self::Opened(job.authenticate()),
            other => other,
        };
    }
}
struct Helper {
    input: Option<mpsc::SyncSender<Vec<Item>>>,
    output: Option<mpsc::Receiver<Vec<Item>>>,
    thread: Option<thread::JoinHandle<()>>,
}
impl Helper {
    fn new() -> std::io::Result<Self> {
        let (send, receive) = mpsc::sync_channel::<Vec<Item>>(1);
        let (done, collect) = mpsc::sync_channel(1);
        let thread = thread::Builder::new()
            .name("protun-next-open".into())
            .spawn(move || {
                while let Ok(mut batch) = receive.recv() {
                    for item in &mut batch {
                        item.authenticate();
                    }
                    if done.send(batch).is_err() {
                        break;
                    }
                }
            })?;
        Ok(Self {
            input: Some(send),
            output: Some(collect),
            thread: Some(thread),
        })
    }
}
impl Drop for Helper {
    fn drop(&mut self) {
        // Disconnect both directions before joining: a parked receiver or a
        // completed send can never hold shutdown hostage. Packet Drop erases
        // any owned ciphertext/plaintext that cannot be returned to the owner.
        self.output.take();
        self.input.take();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

pub(crate) enum Decrypted {
    Packet {
        result: Result<Output>,
        packet: Packet,
        in_place: bool,
    },
    Event(Receive),
}
pub(crate) struct Receiver {
    helper: Option<Helper>,
    ready: VecDeque<Item>,
    owner_batch: Vec<Item>,
    helper_batch: Vec<Item>,
    deferred: Option<Result<Receive>>,
}
impl Receiver {
    pub(crate) fn new() -> Self {
        Self::with_parallel(
            cfg!(target_os = "linux")
                && thread::available_parallelism().is_ok_and(|n| n.get() >= 2),
        )
    }
    fn with_parallel(enabled: bool) -> Self {
        let helper = enabled.then(Helper::new).and_then(std::result::Result::ok);
        let capacity = if helper.is_some() { MAX_BATCH } else { 0 };
        Self {
            helper,
            ready: VecDeque::with_capacity(capacity),
            owner_batch: Vec::with_capacity(capacity),
            helper_batch: Vec::with_capacity(capacity),
            deferred: None,
        }
    }
    pub(crate) fn is_drained(&self) -> bool {
        self.ready.is_empty()
    }
    fn parallelize(&mut self, session: &Session, counters: &mut Counters) -> Result<()> {
        let data_count = self
            .ready
            .iter()
            .take_while(|item| matches!(item, Item::Raw(p) if p.data().starts_with(&[4, 0, 0, 0])))
            .count();
        if data_count < MIN_PARALLEL {
            return Ok(());
        }
        let tail = if self.ready.len() > data_count {
            self.ready.pop_back()
        } else {
            None
        };
        for i in 0..data_count {
            let Some(Item::Raw(packet)) = self.ready.pop_front() else {
                unreachable!("collected data")
            };
            let item = match session.prepare_data(packet) {
                Ok(job) => Item::Job(job),
                Err((error, packet)) => Item::Rejected(error, packet),
            };
            if i < data_count / 2 {
                self.helper_batch.push(item);
            } else {
                self.owner_batch.push(item);
            }
        }
        let helper = self.helper.as_ref().expect("parallel receiver");
        counters.parallel_batches += 1;
        counters.parallel_helper_packets += self.helper_batch.len() as u64;
        counters.parallel_owner_packets += self.owner_batch.len() as u64;
        counters.parallel_max_batch = counters.parallel_max_batch.max(data_count as u64);
        helper
            .input
            .as_ref()
            .expect("live sender")
            .send(std::mem::take(&mut self.helper_batch))
            .map_err(|_| Error::Protocol("receive authentication worker stopped".into()))?;
        for item in &mut self.owner_batch {
            item.authenticate();
        }
        self.helper_batch = helper
            .output
            .as_ref()
            .expect("live receiver")
            .recv()
            .map_err(|_| Error::Protocol("receive authentication worker stopped".into()))?;
        // Commit and delivery happen later, in original receive order, even
        // when owner-side authentication finishes before the helper prefix.
        self.ready.extend(self.helper_batch.drain(..));
        self.ready.extend(self.owner_batch.drain(..));
        self.ready.extend(tail);
        Ok(())
    }
    pub(crate) fn receive<N: Network>(
        &mut self,
        network: &mut N,
        pool: &mut PacketPool,
        session: &mut Session,
        limit: usize,
        counters: &mut Counters,
        mut capture: impl FnMut(&[u8]),
    ) -> Result<Decrypted> {
        if self.ready.is_empty() {
            if let Some(event) = self.deferred.take() {
                return event.map(Decrypted::Event);
            }
            let limit = if self.helper.is_some() {
                limit.clamp(1, MAX_BATCH)
            } else {
                1
            };
            for _ in 0..limit {
                if !self.ready.is_empty() && pool.available() < network.receive_reserve() {
                    break;
                }
                let mut input = pool.take_sized(65535).ok_or(Error::Full)?;
                match network.receive(&mut input, pool) {
                    Ok(Receive::Packet) => {
                        counters.network_reads += 1;
                        capture(input.data());
                        let data = input.data().starts_with(&[4, 0, 0, 0]);
                        self.ready.push_back(Item::Raw(input));
                        if !data {
                            counters.parallel_control_barriers += u64::from(self.helper.is_some());
                            break;
                        }
                    }
                    event => {
                        pool.recycle(input)?;
                        if self.ready.is_empty() {
                            return event.map(Decrypted::Event);
                        }
                        self.deferred = Some(event);
                        break;
                    }
                }
            }
            if self.helper.is_some()
                && let Err(error) = self.parallelize(session, counters)
            {
                self.ready.clear();
                self.owner_batch.clear();
                self.helper_batch.clear();
                return Err(error);
            }
        }
        let item = self.ready.pop_front().expect("received prefix");
        let (result, packet, in_place) = match item {
            Item::Raw(packet) => {
                let in_place = packet.data().starts_with(&[4, 0, 0, 0]);
                let (result, packet) =
                    crate::engine::decrypt_owned(session, pool, packet, in_place)?;
                (result, packet, in_place)
            }
            Item::Opened(data) => {
                let (result, mut packet) = session.finish_data(data);
                if let Ok(Output::Tunnel(n)) = result {
                    packet.consume(16)?;
                    packet.set_data_len(n)?;
                }
                (result, packet, true)
            }
            Item::Rejected(error, packet) => (Err(error), packet, true),
            _ => unreachable!("authentication completes before owner commit"),
        };
        Ok(Decrypted::Packet {
            result,
            packet,
            in_place,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::PacketQueue;

    struct Source(VecDeque<Vec<u8>>);
    impl Network for Source {
        #[cfg(unix)]
        fn fd(&self) -> i32 {
            -1
        }
        #[cfg(windows)]
        fn register(
            &mut self,
            _: &mio::Registry,
            _: mio::Token,
            _: mio::Interest,
        ) -> std::io::Result<()> {
            Ok(())
        }
        #[cfg(windows)]
        fn reregister(
            &mut self,
            _: &mio::Registry,
            _: mio::Token,
            _: mio::Interest,
        ) -> std::io::Result<()> {
            Ok(())
        }
        fn receive(&mut self, packet: &mut Packet, _: &mut PacketPool) -> Result<Receive> {
            let Some(bytes) = self.0.pop_front() else {
                return Ok(Receive::Eof);
            };
            packet.writable()[..bytes.len()].copy_from_slice(&bytes);
            packet.set_len(bytes.len())?;
            Ok(Receive::Packet)
        }
        fn flush(&mut self, _: &mut PacketQueue, _: &mut PacketPool) -> Result<bool> {
            Ok(false)
        }
    }
    fn wire(sender: &mut Session, i: u8) -> Vec<u8> {
        let mut ip = crate::session::tests::ip_packet(1280);
        ip[20..].fill(i);
        let mut bytes = vec![0; 2048];
        let Output::Network(n) = sender.encrypt(&ip, &mut bytes).unwrap() else {
            panic!("data")
        };
        bytes.truncate(n);
        bytes
    }
    #[test]
    fn parallel_batches_commit_in_order_reject_tamper_duplicates_and_preserve_eof() {
        let (mut sender, mut session) = crate::session::tests::pair();
        let before = session.statistics().received_bytes;
        let mut bytes: Vec<_> = (0..32).map(|i| wire(&mut sender, i)).collect();
        let last = bytes[3].len() - 1;
        bytes[3][last] ^= 1;
        bytes[17] = bytes[10].clone();
        let mut source = Source(bytes.into());
        let mut receiver = Receiver::with_parallel(true);
        assert!(receiver.helper.is_some());
        let mut pool = PacketPool::compact(40, 2048, 65535).unwrap();
        let mut counters = Counters::default();
        let mut captured = 0;
        for i in 0..32 {
            let Decrypted::Packet {
                result,
                packet,
                in_place,
            } = receiver
                .receive(
                    &mut source,
                    &mut pool,
                    &mut session,
                    32,
                    &mut counters,
                    |_| captured += 1,
                )
                .unwrap()
            else {
                panic!("packet")
            };
            assert!(in_place);
            if i == 3 || i == 17 {
                assert!(result.is_err());
            } else {
                assert_eq!(result.unwrap(), Output::Tunnel(1280));
                assert_eq!(&packet.data()[20..1280], &[i as u8; 1260]);
            }
            if i == 0 {
                assert_eq!(session.statistics().received_bytes, before + 1280);
            }
            pool.recycle(packet).unwrap();
        }
        assert!(receiver.is_drained());
        assert_eq!(captured, 32);
        assert_eq!(session.statistics().received_bytes, before + 30 * 1280);
        assert_eq!(pool.available(), 40);
        assert_eq!(counters.parallel_batches, 1);
        assert_eq!(counters.parallel_helper_packets, 16);
        assert_eq!(counters.parallel_max_batch, MAX_BATCH as u64);
        assert!(matches!(
            receiver
                .receive(
                    &mut source,
                    &mut pool,
                    &mut session,
                    32,
                    &mut counters,
                    |_| {}
                )
                .unwrap(),
            Decrypted::Event(Receive::Eof)
        ));
    }
    #[test]
    fn real_handshake_is_an_owner_barrier_and_small_batches_stay_serial() {
        let (mut sender, mut session) = crate::session::tests::pair();
        let mut bytes: VecDeque<_> = (0..8).map(|i| wire(&mut sender, i)).collect();
        let mut handshake = vec![0; 2048];
        sender
            .advance(std::time::Duration::from_secs(1), &mut handshake)
            .unwrap();
        let Output::Network(n) = sender.initiate_forced(&mut handshake).unwrap() else {
            panic!("initiation")
        };
        handshake.truncate(n);
        bytes.push_back(handshake);
        bytes.push_back(wire(&mut sender, 9));
        let mut source = Source(bytes);
        let mut receiver = Receiver::with_parallel(true);
        let mut pool = PacketPool::compact(16, 2048, 65535).unwrap();
        let mut counters = Counters::default();
        for i in 0..10 {
            let Decrypted::Packet {
                result,
                packet,
                in_place,
            } = receiver
                .receive(
                    &mut source,
                    &mut pool,
                    &mut session,
                    16,
                    &mut counters,
                    |_| {},
                )
                .unwrap()
            else {
                panic!("packet")
            };
            if i == 8 {
                assert!(!in_place);
                assert!(matches!(result, Ok(Output::Network(92))), "{result:?}");
            } else {
                assert!(in_place);
                assert!(matches!(result, Ok(Output::Tunnel(1280))));
            }
            pool.recycle(packet).unwrap();
        }
        assert!(receiver.is_drained());
        assert_eq!(
            counters.parallel_batches, 1,
            "single trailing data remains serial"
        );
        assert_eq!(counters.parallel_control_barriers, 1);
        assert_eq!(pool.available(), 16);
    }
    #[test]
    fn helper_shutdown_disconnects_a_full_completion_channel_before_joining() {
        let (done, wait) = mpsc::channel();
        thread::spawn(move || {
            let (mut sender, session) = crate::session::tests::pair();
            let mut pool = PacketPool::new(2, 2048).unwrap();
            let helper = Helper::new().unwrap();
            for i in 0..2 {
                let bytes = wire(&mut sender, i);
                let mut packet = pool.take().unwrap();
                packet.writable()[..bytes.len()].copy_from_slice(&bytes);
                packet.set_len(bytes.len()).unwrap();
                let job = match session.prepare_data(packet) {
                    Ok(job) => job,
                    Err(_) => panic!("prepare"),
                };
                assert!(
                    helper
                        .input
                        .as_ref()
                        .unwrap()
                        .send(vec![Item::Job(job)])
                        .is_ok()
                );
            }
            drop(helper);
            done.send(()).unwrap();
        });
        wait.recv_timeout(std::time::Duration::from_secs(5))
            .expect("helper shutdown deadlocked");
    }
    #[test]
    fn backend_prefetch_cannot_take_the_owner_control_reserve() {
        struct Reserving {
            source: Source,
            held: Vec<Packet>,
        }
        impl Network for Reserving {
            #[cfg(unix)]
            fn fd(&self) -> i32 {
                -1
            }
            #[cfg(windows)]
            fn register(
                &mut self,
                _: &mio::Registry,
                _: mio::Token,
                _: mio::Interest,
            ) -> std::io::Result<()> {
                Ok(())
            }
            #[cfg(windows)]
            fn reregister(
                &mut self,
                _: &mio::Registry,
                _: mio::Token,
                _: mio::Interest,
            ) -> std::io::Result<()> {
                Ok(())
            }
            fn receive(&mut self, packet: &mut Packet, pool: &mut PacketPool) -> Result<Receive> {
                let result = self.source.receive(packet, pool)?;
                while pool.available() > 2 {
                    self.held.push(pool.take().unwrap());
                }
                Ok(result)
            }
            fn flush(&mut self, _: &mut PacketQueue, _: &mut PacketPool) -> Result<bool> {
                Ok(false)
            }
        }
        let (mut sender, mut session) = crate::session::tests::pair();
        let mut source = Reserving {
            source: Source((0..32).map(|i| wire(&mut sender, i)).collect()),
            held: Vec::new(),
        };
        let mut receiver = Receiver::with_parallel(true);
        let mut pool = PacketPool::compact(40, 2048, 65535).unwrap();
        let mut counters = Counters::default();
        let Decrypted::Packet { result, packet, .. } = receiver
            .receive(
                &mut source,
                &mut pool,
                &mut session,
                32,
                &mut counters,
                |_| {},
            )
            .unwrap()
        else {
            panic!("packet")
        };
        assert_eq!(result.unwrap(), Output::Tunnel(1280));
        assert!(
            receiver.is_drained(),
            "no plaintext may cross an owner barrier under pressure"
        );
        assert_eq!(pool.available(), 2);
        assert_eq!(counters.network_reads, 1);
        pool.recycle(packet).unwrap();
        for packet in source.held.drain(..) {
            pool.recycle(packet).unwrap();
        }
        assert_eq!(pool.available(), 40);
    }
}
