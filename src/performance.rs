/// Packet-worker-local counters: no atomics, callbacks, packet contents or timing
/// on each packet. Published with the existing coalesced statistics snapshot.
#[derive(Debug, Default, Clone, Copy, serde::Serialize)]
pub struct IoCounters {
    pub receive_calls: u64,
    pub send_calls: u64,
    pub received_datagrams: u64,
    pub sent_datagrams: u64,
    pub max_receive_batch: u64,
    pub max_send_batch: u64,
    pub truncated_datagrams: u64,
    pub gso_datagrams: u64,
    pub gro_datagrams: u64,
    pub kernel_fallbacks: u64,
    pub enobufs: u64,
    pub received_bytes: u64,
    pub sent_bytes: u64,
    pub receive_would_block: u64,
    pub send_would_block: u64,
    pub receive_interrupted: u64,
    pub send_interrupted: u64,
}
impl IoCounters {
    pub(crate) fn socket_error(&mut self, error: &std::io::Error, sending: bool) {
        let counter = match (error.kind(), sending) {
            (std::io::ErrorKind::WouldBlock, false) => &mut self.receive_would_block,
            (std::io::ErrorKind::WouldBlock, true) => &mut self.send_would_block,
            (std::io::ErrorKind::Interrupted, false) => &mut self.receive_interrupted,
            (std::io::ErrorKind::Interrupted, true) => &mut self.send_interrupted,
            _ => return,
        };
        *counter = counter.saturating_add(1);
    }
}
