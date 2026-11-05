use protun_next::{tls, tls_unbuffered::UnbufferedTlsIo};
use rustls::{RootCertStore, ServerConfig, ServerConnection, StreamOwned, pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer}};
use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}, sync::Arc, time::{Duration, Instant}};

const ROOT: &[u8] = include_bytes!("../../../../reference/tests/fixtures/root-TEST-ONLY.der");
const CERT: &[u8] = include_bytes!("../../../../reference/tests/fixtures/server-TEST-ONLY.der");
const KEY: &[u8] = include_bytes!("../../../../reference/tests/fixtures/private-PUBLIC-TEST-ONLY.der");

struct Counted { socket: TcpStream, reads: usize, writes: usize }
impl Read for Counted {
    fn read(&mut self, data: &mut [u8]) -> io::Result<usize> { self.reads+=1; self.socket.read(data) }
}
impl Write for Counted {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> { self.writes+=1; self.socket.write(data) }
    fn flush(&mut self) -> io::Result<()> { self.socket.flush() }
}
enum Client { Buffered(tls::TlsIo<Counted>), Unbuffered(UnbufferedTlsIo<Counted>) }
impl Client {
    fn counts(&self)->(usize,usize) { let s=match self { Self::Buffered(s)=>s.socket(),Self::Unbuffered(s)=>s.socket() };(s.reads,s.writes) }
    fn blocking(&mut self) {let s=match self {Self::Buffered(s)=>s.socket_mut(),Self::Unbuffered(s)=>s.socket_mut()};s.socket.set_nonblocking(false).unwrap();s.socket.set_read_timeout(Some(Duration::from_secs(30))).unwrap();s.socket.set_write_timeout(Some(Duration::from_secs(30))).unwrap();}
}
impl Read for Client {
    fn read(&mut self,d:&mut[u8])->io::Result<usize> { match self {Self::Buffered(s)=>s.read(d),Self::Unbuffered(s)=>s.read(d)} }
}
impl Write for Client {
    fn write(&mut self,d:&[u8])->io::Result<usize> { match self {Self::Buffered(s)=>s.write(d),Self::Unbuffered(s)=>s.write(d)} }
    fn write_vectored(&mut self,d:&[io::IoSlice<'_>])->io::Result<usize> {match self {Self::Buffered(s)=>s.write_vectored(d),Self::Unbuffered(s)=>s.write_vectored(d)}}
    fn flush(&mut self)->io::Result<()> {match self {Self::Buffered(s)=>s.flush(),Self::Unbuffered(s)=>s.flush()}}
}
fn run(mode:&str,version:&'static rustls::SupportedProtocolVersion,chunk:usize,bytes:usize,direction:&str,round:usize) {
    let cfg=ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider())).with_protocol_versions(&[version]).unwrap().with_no_client_auth().with_single_cert(vec![CertificateDer::from(CERT.to_vec())],PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(KEY.to_vec()))).unwrap();
    let listener=TcpListener::bind("127.0.0.1:0").unwrap();let addr=listener.local_addr().unwrap();
    let upload=direction.starts_with("upload");
    let server=std::thread::spawn(move||{
        let (socket,_)=listener.accept().unwrap();socket.set_nodelay(true).unwrap();socket.set_read_timeout(Some(Duration::from_secs(30))).unwrap();socket.set_write_timeout(Some(Duration::from_secs(30))).unwrap();
        let mut s=StreamOwned::new(ServerConnection::new(Arc::new(cfg)).unwrap(),socket);
        let mut hello=[0;1];s.read_exact(&mut hello).unwrap();assert_eq!(hello,[7]);s.write_all(&[8]).unwrap();s.flush().unwrap();
        let body=vec![0x5a;chunk];let mut remaining=bytes;let mut read=vec![0;chunk];
        while remaining>0 {let n=remaining.min(chunk);if upload {s.read_exact(&mut read[..n]).unwrap();assert!(read[..n].iter().all(|b|*b==0x5a));} else {s.write_all(&body[..n]).unwrap();}remaining-=n;}
        if upload {s.write_all(&[9]).unwrap();s.flush().unwrap();} else {s.flush().unwrap();let mut ack=[0;1];s.read_exact(&mut ack).unwrap();assert_eq!(ack,[9]);}
    });
    let socket=TcpStream::connect(addr).unwrap();socket.set_nodelay(true).unwrap();socket.set_nonblocking(true).unwrap();
    let counted=Counted{socket,reads:0,writes:0};let mut roots=RootCertStore::empty();roots.add(CertificateDer::from(ROOT.to_vec())).unwrap();
    let mut client=if mode=="buffered" {Client::Buffered(tls::TlsIo::new(counted,tls::client_with_roots("localhost",roots).unwrap()))} else {Client::Unbuffered(UnbufferedTlsIo::new(counted,tls::unbuffered_client_with_roots("localhost",roots).unwrap()))};
    let deadline=Instant::now()+Duration::from_secs(30);
    let mut begun=false;let mut hello=[0;1];
    loop {
        if !begun {match client.write(&[7]) {Ok(1)=>begun=true,Err(e) if retry(&e)=>{},x=>panic!("hello {x:?}")}}
        let _=client.flush();match client.read(&mut hello) {Ok(1)=>{assert_eq!(hello,[8]);break},Err(e) if retry(&e)=>{},x=>panic!("hello read {x:?}")}
        assert!(Instant::now()<deadline);std::thread::yield_now();
    }
    client.blocking();let before=client.counts();let cpu_before=thread_cpu();let start=Instant::now();let request_chunk=if direction=="upload-burst" {chunk*16} else {chunk};let body=vec![0x5a;request_chunk];let mut received=vec![0;request_chunk];let mut remaining=bytes;
    while remaining>0 {
        assert!(Instant::now()<deadline,"transfer timed out");let n=remaining.min(request_chunk);
        let result=if direction=="upload-burst" {client.write_vectored(&[io::IoSlice::new(&body[..n/2]),io::IoSlice::new(&body[n/2..n])])} else if upload {client.write(&body[..n])} else {client.read(&mut received[..n])};
        match result {Ok(0)=>panic!("early EOF/write zero"),Ok(n)=>{if !upload {assert!(received[..n].iter().all(|b|*b==0x5a));}remaining-=n},Err(e) if retry(&e)=>{std::thread::yield_now()},Err(e)=>panic!("transfer {e}")}
        if upload {match client.flush(){Ok(())=>{},Err(e) if retry(&e)=>{},Err(e)=>panic!("flush {e}")}}
    }
    loop {
        assert!(Instant::now()<deadline);
        if upload {let _=client.flush();match client.read(&mut hello){Ok(1)=>{assert_eq!(hello,[9]);break},Err(e) if retry(&e)=>{},x=>panic!("ack {x:?}")}}
        else {match client.write(&[9]){Ok(1)=>break,Err(e) if retry(&e)=>{},x=>panic!("ack write {x:?}")}}
        std::thread::yield_now();
    }
    while let Err(e)=client.flush(){assert!(retry(&e));assert!(Instant::now()<deadline);std::thread::yield_now();}
    let seconds=start.elapsed().as_secs_f64();let cpu_seconds=thread_cpu()-cpu_before;let after=client.counts();server.join().unwrap();
    println!("{}",serde_json::json!({"mode":mode,"tls":format!("{:?}",version.version),"chunk":chunk,"bytes":bytes,"direction":direction,"round":round,"seconds":seconds,"client_thread_cpu_seconds":cpu_seconds,"client_cpu_ns_per_byte":cpu_seconds*1e9/bytes as f64,"Mbps":bytes as f64*8.0/seconds/1e6,"socket_reads":after.0-before.0,"socket_writes":after.1-before.1,"scope":"blocking Linux loopback TLS adapter, payload validated; no production reactor, WG, TUN or router throughput"}));
}
#[cfg(windows)]
fn thread_cpu()->f64 {use windows_sys::Win32::{Foundation::FILETIME,System::Threading::{GetCurrentThread,GetThreadTimes}};let mut times:[FILETIME;4]=unsafe{std::mem::zeroed()};assert_ne!(unsafe{GetThreadTimes(GetCurrentThread(),&mut times[0],&mut times[1],&mut times[2],&mut times[3])},0);let ticks=|t:&FILETIME|((t.dwHighDateTime as u64)<<32)|t.dwLowDateTime as u64;(ticks(&times[2])+ticks(&times[3])) as f64/1e7}
#[cfg(unix)]
fn thread_cpu()->f64 {let mut t:libc::timespec=unsafe{std::mem::zeroed()};assert_eq!(unsafe{libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID,&mut t)},0);t.tv_sec as f64+t.tv_nsec as f64/1e9}
fn retry(e:&io::Error)->bool {matches!(e.kind(),io::ErrorKind::WouldBlock|io::ErrorKind::Interrupted)}
fn framed(mode:&str,version:&'static rustls::SupportedProtocolVersion,chunk:usize,round:usize) {
    use protun_next::{framing::Decoder,packet::PacketPool};
    let cfg=ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider())).with_protocol_versions(&[version]).unwrap().with_no_client_auth().with_single_cert(vec![CertificateDer::from(CERT.to_vec())],PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(KEY.to_vec()))).unwrap();
    let listener=TcpListener::bind("127.0.0.1:0").unwrap();let addr=listener.local_addr().unwrap();let frames=std::env::var("BENCH_FRAMED_BYTES").ok().and_then(|s|s.parse::<usize>().ok()).unwrap_or(64*1024*1024)/chunk;let bytes=frames*chunk;
    let server=std::thread::spawn(move||{
        let (socket,_)=listener.accept().unwrap();socket.set_nodelay(true).unwrap();socket.set_read_timeout(Some(Duration::from_secs(30))).unwrap();socket.set_write_timeout(Some(Duration::from_secs(30))).unwrap();
        let mut s=StreamOwned::new(ServerConnection::new(Arc::new(cfg)).unwrap(),socket);let mut hello=[0;1];s.read_exact(&mut hello).unwrap();s.write_all(&[8]).unwrap();s.flush().unwrap();
        let mut frame=Vec::with_capacity(chunk+2);frame.extend_from_slice(&(chunk as u16).to_be_bytes());frame.resize(chunk+2,0x5a);let block=frame.repeat(16);let mut n=frames;
        while n>0 {let k=n.min(16);s.write_all(&block[..k*frame.len()]).unwrap();n-=k;}s.flush().unwrap();let mut ack=[0;1];s.read_exact(&mut ack).unwrap();assert_eq!(ack,[9]);
    });
    let socket=TcpStream::connect(addr).unwrap();socket.set_nodelay(true).unwrap();socket.set_read_timeout(Some(Duration::from_secs(30))).unwrap();socket.set_write_timeout(Some(Duration::from_secs(30))).unwrap();let counted=Counted{socket,reads:0,writes:0};let mut roots=RootCertStore::empty();roots.add(CertificateDer::from(ROOT.to_vec())).unwrap();
    let mut client=if mode=="buffered" {Client::Buffered(tls::TlsIo::new(counted,tls::client_with_roots("localhost",roots).unwrap()))} else {Client::Unbuffered(UnbufferedTlsIo::new(counted,tls::unbuffered_client_with_roots("localhost",roots).unwrap()))};
    client.write_all(&[7]).unwrap();client.flush().unwrap();let mut hello=[0;1];client.read_exact(&mut hello).unwrap();assert_eq!(hello,[8]);
    let mut decoder=Decoder::new(65536).unwrap();let mut pool=PacketPool::new(1,chunk).unwrap();let mut output=pool.take().unwrap();let mut scratch=[0;16384];let mut count=0;
    let cpu_start=thread_cpu();let start=Instant::now();let before=client.counts();
    while count<frames {
        while decoder.decode_into(&mut output).unwrap(){assert_eq!(output.data().len(),chunk);assert!(output.data().iter().all(|b|*b==0x5a));count+=1;}
        if count==frames {break;}
        let n=match &mut client {
            Client::Buffered(s)=>{let n=s.read(&mut scratch).unwrap();decoder.feed(&scratch[..n]).unwrap();n},
            Client::Unbuffered(s)=>s.read_plaintext(|data| {decoder.feed(data).map_err(io::Error::other)?;Ok(data.len())}).unwrap(),
        };assert!(n>0);
    }
    client.write_all(&[9]).unwrap();client.flush().unwrap();let seconds=start.elapsed().as_secs_f64();let cpu_seconds=thread_cpu()-cpu_start;let after=client.counts();server.join().unwrap();
    println!("{}",serde_json::json!({"mode":mode,"tls":format!("{:?}",version.version),"chunk":chunk,"bytes":bytes,"frames":frames,"round":round,"direction":"framed-download","seconds":seconds,"client_thread_cpu_seconds":cpu_seconds,"client_cpu_ns_per_byte":cpu_seconds*1e9/bytes as f64,"Mbps":bytes as f64*8.0/seconds/1e6,"socket_reads":after.0-before.0,"socket_writes":after.1-before.1,"scope":"Linux TLS-to-real-framing decoder, borrowed unbuffered records versus buffered scratch; payload validated; no WG/production reactor/TUN"}));
}
fn main(){
    if std::env::args().nth(1).as_deref()==Some("--framed") {for round in 0..4 {for version in [&rustls::version::TLS12,&rustls::version::TLS13] {for chunk in [1280,16352] {for mode in if round%2==0 {["buffered","unbuffered"]} else {["unbuffered","buffered"]} {framed(mode,version,chunk,round);}}}}return;}
    let bytes=std::env::args().nth(1).map(|s|s.parse().unwrap()).unwrap_or(256*1024*1024);
    for round in 0..4 {for version in [&rustls::version::TLS12,&rustls::version::TLS13] {for chunk in [1280,16384] {for direction in ["download","upload","upload-burst"] {for mode in if round%2==0 {["buffered","unbuffered"]} else {["unbuffered","buffered"]} {run(mode,version,chunk,bytes,direction,round);}}}}}
}
