//! Real packet/framing implementation benchmark; no network speed claim.
use protun_next::{
    framing::{Decoder, Encoder},
    packet::PacketPool,
};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
fn main() {
    let mut pool = PacketPool::new(2, 2048).unwrap();
    let mut tx = Encoder::default();
    let mut rx = Decoder::new(65536).unwrap();
    let start = Instant::now();
    let mut count = 0u64;
    while start.elapsed() < Duration::from_secs(1) {
        let mut p = pool.take().unwrap();
        p.writable()[..1280].fill(0x5a);
        p.writable()[..4].copy_from_slice(&[4, 0, 0, 0]);
        p.writable()[8..16].copy_from_slice(&count.to_le_bytes());
        p.set_len(1280).unwrap();
        tx.frame(&mut p).unwrap();
        rx.feed(black_box(p.data())).unwrap();
        let mut out = pool.take().unwrap();
        assert!(rx.decode_into(&mut out).unwrap());
        black_box(out.data());
        pool.recycle(p).unwrap();
        pool.recycle(out).unwrap();
        count += 1;
    }
    println!(
        "{{\"benchmark\":\"actual-frame-roundtrip\",\"bytes\":1280,\"packets\":{count},\"seconds\":{},\"scope\":\"host framing, no encryption, TUN or network; not VPN throughput\"}}",
        start.elapsed().as_secs_f64()
    );
}
