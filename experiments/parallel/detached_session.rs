// Experimental extension included only in an archived vendor tree by overlay.py.
// Keys are immutable and shared; replay state remains on the tunnel owner.
pub struct DataJob {
    pub(super) key: std::sync::Arc<LessSafeKey>,
    pub(super) local: u32,
    pub(super) peer: u32,
    pub(super) counter: u64,
    pub(super) length: usize,
    pub(super) transmit: bool,
}
pub struct DataCompletion {
    pub(super) job: DataJob,
    pub(super) result: Result<usize, WireGuardError>,
}
impl DataJob {
    /// Consumes the ticket. A nonce cannot be reused by rerunning the job.
    pub fn run(self, buffer: &mut [u8]) -> DataCompletion {
        let mut nonce = [0; 12];
        nonce[4..].copy_from_slice(&self.counter.to_le_bytes());
        let result = if self.transmit {
            if buffer.len() < self.length + 32 {
                Err(WireGuardError::DestinationBufferTooSmall)
            } else {
                buffer[..4].copy_from_slice(&4u32.to_le_bytes());
                buffer[4..8].copy_from_slice(&self.peer.to_le_bytes());
                buffer[8..16].copy_from_slice(&self.counter.to_le_bytes());
                self.key
                    .seal_in_place_separate_tag(
                        Nonce::assume_unique_for_key(nonce),
                        Aad::empty(),
                        &mut buffer[16..16 + self.length],
                    )
                    .map(|tag| {
                        buffer[16 + self.length..32 + self.length].copy_from_slice(tag.as_ref());
                        self.length + 32
                    })
                    .map_err(|_| WireGuardError::InvalidAeadTag)
            }
        } else if buffer.len() < 32 {
            Err(WireGuardError::InvalidPacket)
        } else {
            self.key
                .open_in_place(
                    Nonce::assume_unique_for_key(nonce),
                    Aad::empty(),
                    &mut buffer[16..],
                )
                .map(|plain| plain.len())
                .map_err(|_| WireGuardError::InvalidAeadTag)
        };
        DataCompletion { job: self, result }
    }
}
impl<LF: LockFactory> Session<LF> {
    pub(super) fn prepare_transmit(&self, length: usize) -> Result<DataJob, WireGuardError> {
        // Reserve on the serial owner before dispatch, never on a crypto worker.
        let counter = self
            .sending_key_counter
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                if n < u64::MAX - 8192 {
                    Some(n + 1)
                } else {
                    None
                }
            })
            .map_err(|_| WireGuardError::InvalidCounter)?;
        Ok(DataJob {
            key: self.sender.clone(),
            local: self.receiving_index,
            peer: self.sending_index,
            counter,
            length,
            transmit: true,
        })
    }
    pub(super) fn prepare_receive(
        &self,
        receiver: u32,
        counter: u64,
    ) -> Result<DataJob, WireGuardError> {
        if receiver != self.receiving_index {
            return Err(WireGuardError::WrongIndex);
        }
        self.receiving_counter_quick_check(counter)?;
        Ok(DataJob {
            key: self.receiver.clone(),
            local: receiver,
            peer: self.sending_index,
            counter,
            length: 0,
            transmit: false,
        })
    }
    pub(super) fn commit_detached(&self, job: &DataJob) -> Result<(), WireGuardError> {
        let key = if job.transmit {
            &self.sender
        } else {
            &self.receiver
        };
        // Index equality alone is insufficient if the session ring slot is reused.
        if self.receiving_index != job.local || !std::sync::Arc::ptr_eq(key, &job.key) {
            return Err(WireGuardError::NoCurrentSession);
        }
        if !job.transmit {
            self.receiving_counter_mark(job.counter)?;
        }
        Ok(())
    }
}
