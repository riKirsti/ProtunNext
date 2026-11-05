//! Experiment only: one shared key/session pair using the library's Strict lock factory.
use super::session::Session;
use std::convert::TryInto;
use proton_os_interface::lock::{Lock,LockFactory};
struct Strict;
struct SyncLock<T>(std::sync::Mutex<T>);
impl LockFactory for Strict {type Lock<T> = SyncLock<T>;fn new<T>(v:T)->SyncLock<T>{SyncLock(std::sync::Mutex::new(v))}}
impl<'a,T> Lock<'a,T> for SyncLock<T>{type Guard=std::sync::MutexGuard<'a,T> where Self:'a;fn lock(&'a self)->Self::Guard{self.0.lock().unwrap()}}
use std::{sync::{Arc,Barrier},time::Instant};
pub fn run(workers:usize,size:usize,packets:usize,verify:bool)->(f64,usize){
 assert_eq!(packets%128,0);assert_eq!(128%workers,0);
 let a=Arc::new(Session::<Strict>::new(7,9,[11;32],[12;32]));
 let b=Arc::new(Session::<Strict>::new(9,7,[12;32],[11;32]));
 let barrier=Arc::new(Barrier::new(workers));let start=Instant::now();
 let hs:Vec<_>=(0..workers).map(|_|{let a=a.clone();let b=b.clone();let gate=barrier.clone();std::thread::spawn(move||{
  let mut data=vec![0x5a;size+32];let mut nonces=Vec::new();
  for _ in 0..packets/128 {for _ in 0..128/workers {
   let n=a.format_packet_data_in_place(size,&mut data).len();
   let counter=u64::from_le_bytes(data[8..16].try_into().unwrap());
   if verify{nonces.push(counter);}
   let clear=b.receive_packet_data_in_place(9,counter,&mut data[16..n]).unwrap();
   assert_eq!(clear.len(),size);assert!(clear.iter().all(|x|*x==0x5a));
  }gate.wait();}nonces
 })}).collect();
 let mut counters=Vec::new();for h in hs{counters.extend(h.join().unwrap());}
 let seconds=start.elapsed().as_secs_f64();
 if verify {counters.sort_unstable();assert_eq!(counters.len(),packets);for(i,n)in counters.iter().enumerate(){assert_eq!(*n,i as u64);}}
 // Authentication failures must not commit replay state; replay must fail after success.
 let mut wire=vec![0x5a;size+32];let n=a.format_packet_data_in_place(size,&mut wire).len();let c=u64::from_le_bytes(wire[8..16].try_into().unwrap());
 let mut bad=wire.clone();bad[n-1]^=1;assert!(b.receive_packet_data_in_place(9,c,&mut bad[16..n]).is_err());
 let mut replay=wire.clone();assert!(b.receive_packet_data_in_place(9,c,&mut wire[16..n]).is_ok());assert!(b.receive_packet_data_in_place(9,c,&mut replay[16..n]).is_err());
 // A new epoch/key cannot authenticate packets from this old epoch.
 let fresh=Session::<Strict>::new(9,7,[22;32],[21;32]);assert!(fresh.receive_packet_data_in_place(9,c,&mut replay[16..n]).is_err());
 (seconds,packets)
}
