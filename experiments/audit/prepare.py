"""Freeze each runtime independently; copy the SAME corrected harness into all."""
import hashlib, importlib.util, json, pathlib, shutil, subprocess, tempfile
ROOT=pathlib.Path(__file__).resolve().parents[2]
OUT=ROOT/'target/audit-v2';OUT.mkdir(parents=True,exist_ok=True)
refs={'erase':'849be97a207ec0762e1b3660fd61db4a77d2bca8','main':'c4b2554398ffcbf35cdaa9c8797504d8df146bd9'}
refs.update({k:refs['erase']for k in ['gro','parallel','ktls']})
state={}
for name,ref in refs.items():
 dest=pathlib.Path(tempfile.mkdtemp(prefix=name+'-',dir=OUT))
 data=subprocess.check_output(['git','archive',ref],cwd=ROOT)
 subprocess.run(['tar','-x','-C',str(dest)],input=data,check=True)
 shutil.copyfile(ROOT/'experiments/next/worker_probe.rs',dest/'src/worker_probe.rs')
 f=dest/'src/lib.rs';f.write_text(f.read_text()+'\n#[cfg(target_os="linux")] pub mod worker_probe;\n')
 (dest/'src/bin/next-probe.rs').write_text('fn main(){protun_next::worker_probe::run();}\n')
 f=dest/'Cargo.toml';f.write_text(f.read_text()+'\n[[bin]]\nname="next-probe"\npath="src/bin/next-probe.rs"\n')
 row={'sha':ref,'path':str(dest),'archive_sha256':hashlib.sha256(data).hexdigest()}
 if name=='gro':
  f=dest/'src/linux_batch.rs';f.write_bytes(subprocess.check_output(['git','show','32f3411224a56fc20d2ee350fcf55789c5816b2b:src/linux_batch.rs'],cwd=ROOT))
  row['overlay']='final-GRO-segment ownership only, from 32f3411'
 if name in ['parallel','ktls']:
  path=ROOT/('experiments/parallel/overlay.py'if name=='parallel'else'experiments/ktls-worker/overlay.py')
  spec=importlib.util.spec_from_file_location('overlay_'+name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
  row['overlay_sha256']=m.apply(dest)
 digest=hashlib.sha256()
 for file in sorted([*dest.rglob('*.rs'),*dest.rglob('Cargo.toml'),*dest.rglob('Cargo.lock')]):
  digest.update(file.relative_to(dest).as_posix().encode()+b'\0'+file.read_bytes().replace(b'\r\n',b'\n')+b'\0')
 row['effective_sources_sha256']=digest.hexdigest();state[name]=row
(OUT/'builds.json').write_text(json.dumps(state,indent=2));print(json.dumps(state))
