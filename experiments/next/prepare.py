"""Archive baseline/candidate without modifying their production build surfaces."""
import argparse,hashlib,json,pathlib,shutil,subprocess,tempfile
p=argparse.ArgumentParser();p.add_argument('--candidate',default='HEAD');p.add_argument('--parallel',action='store_true');a=p.parse_args()
root=pathlib.Path(__file__).resolve().parents[2];out=root/'target/next';out.mkdir(parents=True,exist_ok=True)
refs={'baseline':'c4b2554398ffcbf35cdaa9c8797504d8df146bd9','candidate':a.candidate};state={}
if a.parallel: refs={'baseline':a.candidate,'candidate':a.candidate}
for name,ref in refs.items():
 sha=subprocess.check_output(['git','rev-parse',ref],cwd=root,text=True).strip()
 dest=pathlib.Path(tempfile.mkdtemp(prefix=name+'-',dir=out))
 archive=subprocess.check_output(['git','archive',sha],cwd=root)
 subprocess.run(['tar','-x','-C',str(dest)],input=archive,check=True)
 shutil.copyfile(root/'experiments/next/worker_probe.rs',dest/'src/worker_probe.rs')
 lib=dest/'src/lib.rs';lib.write_text(lib.read_text()+'\n#[cfg(target_os="linux")] pub mod worker_probe;\n',encoding='utf-8')
 (dest/'src/bin/next-probe.rs').write_text('fn main(){protun_next::worker_probe::run();}\n',encoding='utf-8')
 manifest=dest/'Cargo.toml';manifest.write_text(manifest.read_text()+'\n[[bin]]\nname="next-probe"\npath="src/bin/next-probe.rs"\n',encoding='utf-8')
 state[name]={'sha':sha,'path':str(dest),'archive_sha256':hashlib.sha256(archive).hexdigest()}
 if a.parallel and name=='candidate':
  import importlib.util
  spec=importlib.util.spec_from_file_location('parallel_overlay',root/'experiments/parallel/overlay.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
  state[name]['experimental_overlay_sha256']=module.apply(dest)
 # Hash the actual effective source, including synthetic harness and any overlay.
 # A git SHA alone does not identify an experiment that transforms an archive.
 files=sorted([*dest.rglob('*.rs'),*dest.rglob('Cargo.toml'),*dest.rglob('Cargo.lock')])
 digest=hashlib.sha256()
 for file in files:
  digest.update(file.relative_to(dest).as_posix().encode()+b'\0'+file.read_bytes().replace(b'\r\n',b'\n')+b'\0')
 state[name]['effective_sources_sha256']=digest.hexdigest()
(out/'builds.json').write_text(json.dumps(state,indent=2),encoding='utf-8');print(json.dumps(state))
