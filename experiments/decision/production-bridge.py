"""Exercise PR #2's frozen production hot path without adding runtime options."""
from pathlib import Path
import subprocess
root=Path(__file__).resolve().parents[2];dest=root/'production-reference';dest.mkdir(exist_ok=True)
subprocess.run(['tar','-x','-C',str(dest)],input=subprocess.check_output(['git','archive','f53b96328d3b8c50070bc62731467b9a0c9f61a3'],cwd=root),check=True)
s=(root/'reference/src/decision_worker.rs').read_text()
s=s.replace('performance::{Options,UdpBackend},','')
start=s.index('fn options(');end=s.index('\nfn one(',start);s=s[:start]+s[end:]
s=s.replace('let mut o=Options::default();o.udp_backend=UdpBackend::Batch;o.udp_offload=true;','')
s=s.replace('Batch::new(&u,o)','Batch::new(&u,true)')
s=s.replace('let o=options(name);','').replace('.with_performance(o)','')
start=s.index(' if proto=="udp" && name.starts_with("uring")');end=s.index('if proto=="udp"&&o.udp_offload',start);s=s[:start]+s[end:]
s=s.replace('o.udp_offload','true').replace('crate::tun_offload::BENCH_TUN_RECORDS.load(Ordering::Relaxed)','0u64')
(dest/'src/decision_worker.rs').write_text(s)
p=dest/'src/lib.rs';p.write_text(p.read_text()+'\n#[cfg(target_os="linux")] pub mod decision_worker;\n')
(dest/'src/bin/production-worker.rs').write_text('fn main(){protun_next::decision_worker::run();}\n')
p=dest/'Cargo.toml';p.write_text(p.read_text()+'\n[[bin]]\nname="production-worker"\npath="src/bin/production-worker.rs"\n')
