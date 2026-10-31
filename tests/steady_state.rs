//! Executes the real engine, not a replacement cipher or model of the packet path.
use protun_next::{
    framing::{Decoder, Encoder},
    packet::PacketPool,
    session::{Output, Session},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
thread_local! {static WATCH:Cell<bool>=const{Cell::new(false)};static ALLOCATIONS:Cell<usize>=const{Cell::new(0)};}
struct Counting;
// SAFETY: all operations delegate unchanged to System, with thread-local observation.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        WATCH.with(|w| {
            if w.get() {
                ALLOCATIONS.with(|a| a.set(a.get() + 1))
            }
        });
        // SAFETY: exactly forwards the valid allocator arguments.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        // SAFETY: forwards the original allocation pointer and layout.
        unsafe { System.dealloc(p, layout) }
    }
    unsafe fn realloc(&self, p: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        WATCH.with(|w| {
            if w.get() {
                ALLOCATIONS.with(|a| a.set(a.get() + 1))
            }
        });
        // SAFETY: exactly forwards the valid allocator arguments.
        unsafe { System.realloc(p, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: Counting = Counting;
fn pair() -> (Session, Session) {
    let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
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
    let mut x = [0; 2048];
    let mut y = [0; 2048];
    let mut ip = [0; 20];
    ip[0] = 0x45;
    ip[2..4].copy_from_slice(&20u16.to_be_bytes());
    let Output::Network(n) = a.encrypt(&ip, &mut x).unwrap() else {
        panic!("handshake init")
    };
    let Output::Network(n) = b.decrypt(&x[..n], &mut y).unwrap() else {
        panic!("handshake reply")
    };
    let Output::Network(n) = a.decrypt(&y[..n], &mut x).unwrap() else {
        panic!("confirmation")
    };
    b.decrypt(&x[..n], &mut y).unwrap();
    while let Output::Network(n) = a.drain(&mut x).unwrap() {
        b.decrypt(&x[..n], &mut y).unwrap();
    }
    (a, b)
}
#[test]
fn healthy_established_crypto_and_framing_do_not_allocate_per_packet() {
    let (mut a, mut b) = pair();
    let mut pool = PacketPool::new(3, 2048).unwrap();
    let allowed = protun_next::routing::AllowedIps::new(&["0.0.0.0/0".into()]).unwrap();
    let mut tx = Encoder::default();
    let mut rx = Decoder::new(65536).unwrap();
    let mut ip = [0; 1280];
    ip[0] = 0x45;
    ip[2..4].copy_from_slice(&1280u16.to_be_bytes());
    ip[8] = 64;
    ip[9] = 17;
    let mut clear = [0; 2048];
    let mut wire = [0; 2048];
    // Warm provider dispatch before observing the current thread's allocation calls.
    for _ in 0..8 {
        let Output::Network(n) = a.encrypt(&ip, &mut wire).unwrap() else {
            panic!()
        };
        b.decrypt(&wire[..n], &mut clear).unwrap();
    }
    ALLOCATIONS.with(|n| n.set(0));
    WATCH.with(|w| w.set(true));
    for _ in 0..1024 {
        assert!(allowed.outbound(&ip));
        let mut encrypted = pool.take().unwrap();
        let Output::Network(n) = a.encrypt(&ip, encrypted.writable()).unwrap() else {
            panic!()
        };
        encrypted.set_len(n).unwrap();
        tx.frame(&mut encrypted).unwrap();
        rx.feed(encrypted.data()).unwrap();
        let mut restored = pool.take().unwrap();
        assert!(rx.decode_into(&mut restored).unwrap());
        let Output::Tunnel(n) = b.decrypt(restored.data(), &mut clear).unwrap() else {
            panic!()
        };
        assert!(allowed.inbound(&clear[..n]));
        assert_eq!(&clear[..n], ip);
        pool.recycle(restored).unwrap();
        pool.recycle(encrypted).unwrap();
    }
    WATCH.with(|w| w.set(false));
    let allocations = ALLOCATIONS.with(Cell::get);
    assert_eq!(allocations, 0, "steady-state allocations");
}
