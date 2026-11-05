//! Code-generation fixture only; never packaged with the runtime.
//! Live bytes escape BEFORE erasure. No readback forces the erasure stores.
use protun_next::packet::{MAX_IP_PACKET, PacketPool};

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn audit_drop_packet(value: u8) {
    let mut pool = PacketPool::new(1, MAX_IP_PACKET).unwrap();
    let mut packet = pool.take().unwrap();
    packet.writable().fill(value);
    packet.set_len(MAX_IP_PACKET).unwrap();
    std::hint::black_box(packet.data());
    drop(packet);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn audit_recycle_packet(value: u8) {
    let mut pool = PacketPool::new(1, MAX_IP_PACKET).unwrap();
    let mut packet = pool.take().unwrap();
    packet.writable().fill(value);
    packet.set_len(MAX_IP_PACKET).unwrap();
    std::hint::black_box(packet.data());
    pool.recycle(packet).unwrap();
    // Retain this fixture's pool until process exit: destruction must not stand
    // in for the recycle-path erasure being inspected. No production leak.
    std::mem::forget(pool);
}

fn main() {
    let value = std::env::args()
        .nth(1)
        .expect("one live-byte argument")
        .as_bytes()[0];
    audit_drop_packet(value);
    audit_recycle_packet(value);
    println!("ERASURE_CODEGEN_OK");
}
