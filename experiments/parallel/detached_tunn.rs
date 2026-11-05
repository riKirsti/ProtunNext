// Experimental owner-side prepare/commit API; no tunnel state crosses a thread.
pub use session::{DataCompletion, DataJob};
impl<R: CryptoRng, LF: LockFactory> Tunn<R, LF> {
    pub fn prepare_data_transmit(&self, length: usize) -> Result<Option<DataJob>, WireGuardError> {
        self.sessions[self.current % N_SESSIONS]
            .as_ref()
            .map(|s| s.prepare_transmit(length))
            .transpose()
    }
    pub fn prepare_data_receive(&self, packet: &[u8]) -> Result<DataJob, WireGuardError> {
        let Packet::PacketData(data) = tunn_parse_incoming_packet(packet)? else {
            return Err(WireGuardError::InvalidPacket);
        };
        self.sessions[data.receiver_idx as usize % N_SESSIONS]
            .as_ref()
            .ok_or(WireGuardError::NoCurrentSession)?
            .prepare_receive(data.receiver_idx, data.counter)
    }
    pub fn complete_data<'a>(
        &mut self,
        completion: DataCompletion,
        buffer: &'a mut [u8],
    ) -> TunnResult<'a> {
        let length = match completion.result {
            Ok(n) => n,
            Err(e) => return TunnResult::Err(e),
        };
        let job = completion.job;
        let Some(session) = self.sessions[job.local as usize % N_SESSIONS].as_ref() else {
            return TunnResult::Err(WireGuardError::NoCurrentSession);
        };
        if let Err(e) = session.commit_detached(&job) {
            return TunnResult::Err(e);
        }
        if job.transmit {
            self.timer_tick(TimerName::TimeLastPacketSent);
            if job.length != 0 {
                self.timer_tick(TimerName::TimeLastDataPacketSent);
            }
            self.tx_bytes += job.length;
            TunnResult::WriteToNetwork(&mut buffer[..length])
        } else {
            self.set_current_session(job.local as usize);
            self.timer_tick(TimerName::TimeLastPacketReceived);
            self.validate_decapsulated_packet(&mut buffer[16..16 + length])
        }
    }
}
