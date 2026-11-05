"""Construct two fingerprinted archives for the bounded worker screen."""
import argparse,hashlib,json,pathlib,shutil,subprocess,tempfile
from overlay import apply
p=argparse.ArgumentParser();p.add_argument('--candidate',default='HEAD');a=p.parse_args()
root=pathlib.Path(__file__).resolve().parents[2];out=root/'target/next';out.mkdir(parents=True,exist_ok=True)
sha=subprocess.check_output(['git','rev-parse',a.candidate],cwd=root,text=True).strip()
archive=subprocess.check_output(['git','archive',sha],cwd=root);state={}
for name in ['baseline','candidate']:
 dest=pathlib.Path(tempfile.mkdtemp(prefix='ktls-'+name+'-',dir=out))
 subprocess.run(['tar','-x','-C',str(dest)],input=archive,check=True)
 shutil.copyfile(root/'experiments/next/worker_probe.rs',dest/'src/worker_probe.rs')
 file=dest/'src/lib.rs';file.write_text(file.read_text()+'\n#[cfg(target_os="linux")] pub mod worker_probe;\n')
 (dest/'src/bin/next-probe.rs').write_text('fn main(){protun_next::worker_probe::run();}\n')
 file=dest/'Cargo.toml';file.write_text(file.read_text()+'\n[[bin]]\nname="next-probe"\npath="src/bin/next-probe.rs"\n')
 state[name]={'sha':sha,'path':str(dest),'archive_sha256':hashlib.sha256(archive).hexdigest()}
 if name=='candidate':state[name]['experimental_overlay_sha256']=apply(dest)
 digest=hashlib.sha256()
 for file in sorted([*dest.rglob('*.rs'),*dest.rglob('Cargo.toml'),*dest.rglob('Cargo.lock')]):
  digest.update(file.relative_to(dest).as_posix().encode()+b'\0'+file.read_bytes().replace(b'\r\n',b'\n')+b'\0')
 state[name]['effective_sources_sha256']=digest.hexdigest()
(out/'builds.json').write_text(json.dumps(state,indent=2));print(json.dumps(state))
