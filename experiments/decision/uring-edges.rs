#[cfg(test)]
mod decision_edges {
 use super::*;
 use crate::performance::Options;
 #[test]
 fn full_payload_capacity_gro_short_tail_ipv4_ipv6_and_shutdown(){
  for ipv6 in [false,true]{
   let bind=if ipv6{"[::1]:0"}else{"127.0.0.1:0"};let a=UdpSocket::bind(bind).unwrap();let b=UdpSocket::bind(bind).unwrap();a.connect(b.local_addr().unwrap()).unwrap();b.connect(a.local_addr().unwrap()).unwrap();b.set_nonblocking(true).unwrap();
   let mut o=Options::default();o.udp_offload=true;let _gro=crate::linux_batch::Batch::new(&b,o);
   let mut r=Receiver::new(&b,8).expect("required io_uring");let mut pool=PacketPool::new(2,65535).unwrap();let mut out=pool.take().unwrap();let mut c=IoCounters::default();
   for size in [0,1,1280,if ipv6{65480}else{65507}]{
    let body=vec![0x5a;size];a.send(&body).unwrap();let deadline=std::time::Instant::now()+std::time::Duration::from_secs(2);
    loop{assert!(std::time::Instant::now()<deadline);if matches!(r.receive(&b,&mut out,&mut c).unwrap(),Receive::Packet){break;}std::thread::yield_now();}assert_eq!(out.data(),body);
   }
   let segment:libc::c_int=1280;assert_eq!(unsafe{libc::setsockopt(a.as_raw_fd(),libc::SOL_UDP,libc::UDP_SEGMENT,(&segment as *const libc::c_int).cast(),4)},0);a.send(&vec![0x5a;1280*16+17]).unwrap();
   for i in 0..17 {let deadline=std::time::Instant::now()+std::time::Duration::from_secs(2);loop{assert!(std::time::Instant::now()<deadline);if matches!(r.receive(&b,&mut out,&mut c).unwrap(),Receive::Packet){break;}std::thread::yield_now();}assert_eq!(out.data().len(),if i==16{17}else{1280});assert!(out.data().iter().all(|x|*x==0x5a));}
   assert!(c.gro_datagrams>0);assert_eq!(c.kernel_fallbacks,0);assert_eq!(c.enobufs,0);let t=std::time::Instant::now();drop(r);assert!(t.elapsed()<std::time::Duration::from_secs(2));
  }
 }
}
