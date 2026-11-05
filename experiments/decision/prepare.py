"""Prepare an ephemeral benchmark copy of frozen 83f0252; never patch production."""
from pathlib import Path
import shutil, subprocess

root = Path(__file__).resolve().parents[2]
r = root / 'reference'
r.mkdir(exist_ok=True)
archive = subprocess.check_output(['git','archive','83f025231535a3ef0731123988e9dae8621cef4a'], cwd=root)
subprocess.run(['tar','-x','-C',str(r)],input=archive,check=True)
p=r/'src/lib.rs'; p.write_text(p.read_text()+'\n#[cfg(target_os="linux")] pub mod decision;\n')
shutil.copy(root/'experiments/decision/core.rs', r/'src/decision.rs')
p=r/'src/decision.rs';s=p.read_text();s=s.replace('if args.get(1).map(String::as_str)==Some("--cpu")', 'if args.get(1).map(String::as_str)==Some("--train"){for size in [512,1420] {for mode in ["copy","inplace"] {let(s,c)=crypto(mode,size,200000);emit(serde_json::json!({"test":"pgo-training-only","size":size,"mode":mode,"seconds":s,"cpu":c}));}}return;}if args.get(1).map(String::as_str)==Some("--cpu")');p.write_text(s)
(r/'src/bin/decision.rs').write_text('fn main() { protun_next::decision::run(); }\n')
p=r/'Cargo.toml';p.write_text(p.read_text()+'\n[[bin]]\nname="decision"\npath="src/bin/decision.rs"\n')
shutil.copy(root/'experiments/decision/worker.rs',r/'src/decision_worker.rs')
p=r/'tests/common/mod.rs';s=p.read_text().replace('pub fn server() -> Arc<ServerConfig> {','pub fn server() -> Arc<ServerConfig> {let versions=[if std::env::var("BENCH_TLS12").as_deref()==Ok("1"){&rustls::version::TLS12}else{&rustls::version::TLS13}];').replace('.with_safe_default_protocol_versions()', '.with_protocol_versions(&versions)');p.write_text(s)
p=r/'src/lib.rs';p.write_text(p.read_text()+'\n#[cfg(target_os="linux")] pub mod decision_worker;\n')
(r/'src/bin/decision-worker.rs').write_text('fn main(){protun_next::decision_worker::run();}\n')
p=r/'Cargo.toml';p.write_text(p.read_text()+'\n[[bin]]\nname="decision-worker"\npath="src/bin/decision-worker.rs"\n')
p=r/'src/network.rs';s=p.read_text().replace('(!options.udp_offload)\n                .then(|| crate::uring_rx::Receiver::new(&socket, options.udp_batch_size))','true\n                .then(|| crate::uring_rx::Receiver::new(&socket, options.udp_batch_size))');p.write_text(s)
shutil.copy(root/'experiments/decision/parallel.rs',r/'vendor/proton-boringtun/src/noise/decision_parallel.rs')
p=r/'vendor/proton-boringtun/src/noise/mod.rs';p.write_text(p.read_text()+'\npub mod decision_parallel;\n')
p=r/'src/decision.rs';s=p.read_text();s=s.replace('if args.get(1).map(String::as_str)==Some("--cpu")','''if args.get(1).map(String::as_str)==Some("--parallel"){
for workers in [1,2,4]{proton_boringtun::noise::decision_parallel::run(workers,1280,8192,true);}
for round in 0..6{for size in [64,1280]{for workers in if round%2==0{[1,2,4]}else{[4,2,1]}{let(s,n)=proton_boringtun::noise::decision_parallel::run(workers,size,262144,false);emit(serde_json::json!({"test":"shared-session-crypto","size":size,"workers":workers,"round":round,"seconds":s,"bytes":n*size,"Mbps":n as f64*size as f64*8.0/s/1e6,"scope":"one shared Strict Session pair, real nonce allocation and locked replay commit, 128-packet ordered epochs; not a production parallel packet worker"}));}}}return;}if args.get(1).map(String::as_str)==Some("--cpu")''');p.write_text(s)
p=r/'src/linux_batch.rs';s=p.read_text().replace('if options.udp_offload {','if options.udp_offload && std::env::var("BENCH_GRO").as_deref() != Ok("0") {').replace('gso: options.udp_offload,','gso: options.udp_offload && std::env::var("BENCH_GSO").as_deref() != Ok("0"),').replace('mmsg: true,','mmsg: std::env::var("BENCH_SINGLE").as_deref() != Ok("1"),').replace('fn gro_size(', 'pub(crate) fn gro_size(');p.write_text(s)
p=r/'src/tun_offload.rs';s=p.read_text().replace('pub(crate) header_len: usize,','pub(crate) header_len: usize,\n    pub(crate) read_records: u64,').replace('header_len: size as usize,','header_len: size as usize, read_records: 0,').replace('let n = unsafe { libc::readv(', 'self.read_records += 1;\n            let n = unsafe { libc::readv(');p.write_text(s)
p=r/'src/tun_offload.rs';s=p.read_text();s='pub(crate) static BENCH_TUN_RECORDS: std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);\n'+s.replace('//!','//');s=s.replace('self.read_records += 1;','self.read_records += 1; BENCH_TUN_RECORDS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);');p.write_text(s)
p=r/'src/tun_offload.rs';p.write_text(p.read_text().replace('header_len: 10,','header_len: 10, read_records: 0,'))
p=r/'src/uring_rx.rs';s=p.read_text()
s=s.replace('    ring: Option<IoUring>,','    message: Box<libc::msghdr>,\n    aggregate: Packet,\n    segment: usize,\n    offset: usize,\n    ring: Option<IoUring>,')
s=s.replace('let count = count.max(8).next_power_of_two() as usize;', 'let count = std::env::var("BENCH_BUFFERS").ok().and_then(|s|s.parse::<u16>().ok()).unwrap_or(count).max(8).next_power_of_two() as usize;')
s=s.replace('let mut receiver = Self {','let mut message: Box<libc::msghdr> = Box::new(unsafe { std::mem::zeroed() });\n        message.msg_controllen = 64;\n        let mut extra = PacketPool::new(1,65535).map_err(io::Error::other)?;\n        let mut receiver = Self { message, aggregate: extra.take().unwrap(), segment: 0, offset: 0,')
s=s.replace('opcode::RecvMulti::new(types::Fd(socket.as_raw_fd()), GROUP)','opcode::RecvMsgMulti::new(types::Fd(socket.as_raw_fd()), &*self.message, GROUP)')
needle='        self.arm(socket)?;\n        let mut completion'
s=s.replace(needle,'''        if self.segment > 0 {
            let end=(self.offset+self.segment).min(self.aggregate.data().len());
            let n=end-self.offset;
            output.writable()[..n].copy_from_slice(&self.aggregate.data()[self.offset..end]);
            output.set_len(n)?; self.offset=end;
            if end==self.aggregate.data().len() {self.segment=0;self.offset=0;}
            counters.received_datagrams+=1;
            return Ok(Receive::Packet);
        }
        self.arm(socket)?;
        let mut completion''')
start=s.index('            if result >= 0 && result as usize <= 65535 {')
end=s.index('            self.buffers[id].set_len(0)?;',start+len('            if result >= 0 && result as usize <= 65535 {'))
# Find the end of the original successful-message branch, rather than its inner reset.
end=s.index('            }\n            self.buffers[id].set_len(0)?;',start)+len('            }')
s=s[:start]+'''            if result >= 0 && result as usize <= 65535 {
                let parsed=types::RecvMsgOut::parse(&self.buffers[id].writable()[..result as usize], &self.message).map_err(|_|Error::Invalid("recvmsg layout"))?;
                if parsed.is_payload_truncated() || parsed.is_control_data_truncated() {return Err(Error::Invalid("recvmsg truncated"));}
                let length=parsed.payload_data().len();
                let offset=16+self.message.msg_namelen as usize+self.message.msg_controllen as usize;
                let mut control: libc::msghdr=unsafe{std::mem::zeroed()};
                control.msg_control=parsed.control_data().as_ptr().cast_mut().cast();
                control.msg_controllen=parsed.control_data().len() as _;
                let segment=crate::linux_batch::gro_size(&control)?;
                self.buffers[id].set_len(result as usize)?;
                std::mem::swap(output, &mut self.buffers[id]);
                self.buffers[id].set_len(0)?;self.provide(id);
                output.consume(offset)?; output.set_data_len(length)?;
                counters.received_bytes+=length as u64;
                if segment>0 {
                    counters.gro_datagrams+=length.div_ceil(segment) as u64;
                    std::mem::swap(output,&mut self.aggregate);self.segment=segment;self.offset=0;
                    return self.receive(socket,output,counters);
                }
                counters.received_datagrams+=1; return Ok(Receive::Packet);
            }'''+s[end:]
s=s.replace('.flags(libc::MSG_TRUNC)', '.flags(libc::MSG_TRUNC as _)')
s=s.replace('opcode::RecvMulti::CODE','opcode::RecvMsgMulti::CODE')
# Keep recvmsg's 16-byte prefix and 24-byte ancillary area in existing headroom.
# Payload capacity remains 65535, and swapped packets still obey the normal pool contract.
s=s.replace('message.msg_controllen = 64;', 'message.msg_controllen = 24;')
s=s.replace('let bytes = self.buffers[id].writable();','let bytes = self.buffers[id].recvmsg_storage();')
s=s.replace('result as usize <= 65535','result as usize <= 65575')
s=s.replace('self.buffers[id].writable()[..result as usize]','self.buffers[id].recvmsg_storage()[..result as usize]')
s=s.replace('self.buffers[id].set_len(result as usize)?;','self.buffers[id].set_len(length)?;')
s=s.replace('output.consume(offset)?; output.set_data_len(length)?;','let _=offset; output.set_data_len(length)?;')
p.write_text(s+(root/'experiments/decision/uring-edges.rs').read_text())
p=r/'src/packet.rs';s=p.read_text().replace('    pub fn data(&self)', '    pub(crate) fn recvmsg_storage(&mut self)->&mut[u8]{&mut self.bytes[HEADROOM-40..]}\n    pub fn data(&self)');p.write_text(s)
# Reuse the actual validated adapter/framing benchmark, with immutable reference paths.
t=root/'experiments/decision/tls';(t/'src').mkdir(parents=True,exist_ok=True)
shutil.copy(root/'experiments/decision/tls-main.rs',t/'src/main.rs')
(t/'Cargo.toml').write_text('''[package]
name="decision-tls"
version="0.0.0"
edition="2024"
[dependencies]
protun-next={path="../../../reference",default-features=false,features=["in-place-crypto","io-uring"]}
rustls={version="=0.23.46",default-features=false,features=["std","ring","tls12"]}
serde_json="1"
libc="0.2"
[profile.release]
lto="thin"
codegen-units=1
panic="abort"
''')
