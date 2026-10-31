//! A narrow adapter around the existing WireGuard cryptographic protocol only.
//! No pvpnclient, PFFF, action channel or userspace NAT pipeline is used.
use crate::{Error, Result};
use proton_boringtun::proton_os_interface::{
    lock::Loose,
    time::{FromDuration, Instant, SystemTime},
};
use proton_boringtun::rand::SeedableRng;
use proton_boringtun::{
    noise::{Tunn, TunnResult},
    x25519,
};
use rand_chacha::ChaCha20Rng;
use std::time::Duration;
use zeroize::Zeroize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Output {
    None,
    Network(usize),
    Tunnel(usize),
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Statistics {
    pub handshake_age: Option<Duration>,
    pub sent_bytes: u64,
    pub received_bytes: u64,
    pub estimated_loss: f32,
    pub handshake_rtt: Option<Duration>,
}
pub struct Session {
    tunnel: Tunn<ChaCha20Rng, Loose>,
    now: Duration,
}
impl Session {
    pub fn new(
        mut private: [u8; 32],
        peer: [u8; 32],
        keepalive: Option<u16>,
        now: Duration,
        wall: Duration,
    ) -> Result<Self> {
        if private == [0; 32] || peer == [0; 32] || wall.as_secs() == 0 {
            private.zeroize();
            return Err(Error::Invalid("key or synchronized wall clock"));
        }
        let mut seed = [0u8; 32];
        if let Err(e) = getrandom::fill(&mut seed) {
            private.zeroize();
            return Err(Error::Protocol(e.to_string()));
        }
        let mut index_bytes = [0u8; 4];
        if let Err(e) = getrandom::fill(&mut index_bytes) {
            private.zeroize();
            seed.zeroize();
            return Err(Error::Protocol(e.to_string()));
        }
        let index = u32::from_le_bytes(index_bytes) & 0x00ff_ffff;
        let rng = ChaCha20Rng::from_seed(seed);
        seed.zeroize();
        let tunnel = Tunn::new(
            x25519::StaticSecret::from(private),
            x25519::PublicKey::from(peer),
            None,
            keepalive,
            index,
            None,
            Instant::from_duration(now),
            SystemTime::from_duration(wall),
            rng,
        );
        private.zeroize();
        Ok(Self { tunnel, now })
    }
    pub fn public_key(mut private: [u8; 32]) -> [u8; 32] {
        let key = x25519::PublicKey::from(&x25519::StaticSecret::from(private)).to_bytes();
        private.zeroize();
        key
    }
    fn output(result: TunnResult<'_>) -> Result<Output> {
        match result {
            TunnResult::Done => Ok(Output::None),
            TunnResult::WriteToNetwork(p) => Ok(Output::Network(p.len())),
            TunnResult::WriteToTunnelV4(p, _) | TunnResult::WriteToTunnelV6(p, _) => {
                Ok(Output::Tunnel(p.len()))
            }
            TunnResult::Err(e) => Err(Error::Protocol(format!("WireGuard: {e:?}"))),
        }
    }
    /// Must be called at each wake-up before packet processing, including after suspend.
    pub fn advance(&mut self, now: Duration, output: &mut [u8]) -> Result<Output> {
        if output.len() < 148 {
            return Err(Error::Invalid("timer output capacity"));
        }
        if now < self.now {
            return Err(Error::Invalid("monotonic clock moved backward"));
        }
        self.now = now;
        Self::output(
            self.tunnel
                .update_timers(output, Instant::from_duration(now)),
        )
    }
    pub fn encrypt(&mut self, packet: &[u8], output: &mut [u8]) -> Result<Output> {
        if packet.len() > 65_503 || output.len() < packet.len().saturating_add(32).max(148) {
            return Err(Error::Invalid("WireGuard output capacity"));
        }
        Self::output(self.tunnel.encapsulate(packet, output))
    }
    pub fn initiate(&mut self, output: &mut [u8]) -> Result<Output> {
        if output.len() < 148 {
            return Err(Error::Invalid("handshake output capacity"));
        }
        Self::output(self.tunnel.format_handshake_initiation(output, false))
    }
    #[cfg(unix)]
    pub(crate) fn initiate_forced(&mut self, output: &mut [u8]) -> Result<Output> {
        if output.len() < 148 {
            return Err(Error::Invalid("handshake output capacity"));
        }
        Self::output(self.tunnel.format_handshake_initiation(output, true))
    }
    pub fn decrypt(&mut self, packet: &[u8], output: &mut [u8]) -> Result<Output> {
        if output.len() < packet.len().max(148) {
            return Err(Error::Invalid("WireGuard output capacity"));
        }
        Self::output(self.tunnel.decapsulate(None, packet, output))
    }
    /// Drain handshake-queued output without allocating a second packet representation.
    pub fn drain(&mut self, output: &mut [u8]) -> Result<Output> {
        self.decrypt(&[], output)
    }
    pub fn next_wakeup(&self) -> Option<Duration> {
        self.tunnel.poll_delay(Instant::from_duration(self.now))
    }
    pub fn stats(&self) -> (Option<Duration>, usize, usize, f32, Option<u32>) {
        self.tunnel.stats()
    }
    pub fn statistics(&self) -> Statistics {
        let (handshake_age, sent, received, loss, rtt_ms) = self.tunnel.stats();
        Statistics {
            handshake_age,
            sent_bytes: sent as u64,
            received_bytes: received as u64,
            estimated_loss: loss,
            handshake_rtt: rtt_ms.map(|ms| Duration::from_millis(ms as u64)),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) fn pair() -> (Session, Session) {
        let wall = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let mut a = Session::new(
            [7; 32],
            Session::public_key([9; 32]),
            None,
            Duration::ZERO,
            wall,
        )
        .unwrap();
        let mut b = Session::new(
            [9; 32],
            Session::public_key([7; 32]),
            None,
            Duration::ZERO,
            wall,
        )
        .unwrap();
        let ip = ip_packet(20);
        let mut first = [0; 2048];
        let mut second = [0; 2048];
        let Output::Network(n) = a.encrypt(&ip, &mut first).unwrap() else {
            panic!("initiation")
        };
        let Output::Network(m) = b.decrypt(&first[..n], &mut second).unwrap() else {
            panic!("response")
        };
        let Output::Network(n) = a.decrypt(&second[..m], &mut first).unwrap() else {
            panic!("confirmation")
        };
        // The encrypted empty confirmation establishes the responder's session.
        let _ = b.decrypt(&first[..n], &mut second).unwrap();
        while let Output::Network(n) = a.drain(&mut first).unwrap() {
            let _ = b.decrypt(&first[..n], &mut second).unwrap();
        }
        (a, b)
    }
    pub(crate) fn ip_packet(size: usize) -> Vec<u8> {
        assert!((20..=65503).contains(&size));
        let mut p = vec![0; size];
        p[0] = 0x45;
        p[2..4].copy_from_slice(&(size as u16).to_be_bytes());
        p[8] = 64;
        p[9] = 17;
        p[12..16].copy_from_slice(&[10, 2, 0, 2]);
        p[16..20].copy_from_slice(&[10, 2, 0, 3]);
        p
    }
    #[test]
    fn actual_handshake_packet_roundtrip_replay_and_tamper() {
        let (mut a, mut b) = pair();
        let ip = ip_packet(1280);
        let mut wire = [0; 2048];
        let mut plain = [0; 2048];
        let Output::Network(n) = a.encrypt(&ip, &mut wire).unwrap() else {
            panic!("network")
        };
        let mut bad = wire;
        bad[n - 1] ^= 1;
        assert!(b.decrypt(&bad[..n], &mut plain).is_err());
        let Output::Tunnel(m) = b.decrypt(&wire[..n], &mut plain).unwrap() else {
            panic!("tunnel")
        };
        assert_eq!(&plain[..m], ip);
        assert!(b.decrypt(&wire[..n], &mut plain).is_err());
    }
    #[test]
    fn refuses_clock_regression_and_bad_capacity() {
        let (mut a, _) = pair();
        let mut buf = [0; 2048];
        a.advance(Duration::from_secs(1), &mut buf).unwrap();
        assert!(a.advance(Duration::ZERO, &mut buf).is_err());
        assert!(a.encrypt(&[0; 20], &mut []).is_err());
    }
}
