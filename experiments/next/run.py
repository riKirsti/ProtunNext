"""Matched timing/profiling with fixed separate peer/load processes and CPU sets."""
import argparse,hashlib,itertools,json,os,pathlib,signal,subprocess,time
p=argparse.ArgumentParser();p.add_argument('--profile-only',action='store_true');p.add_argument('--rounds',type=int,default=8);a=p.parse_args()
root=pathlib.Path(__file__).resolve().parents[2];out=root/'target/next/results';out.mkdir(parents=True,exist_ok=True)
state=json.loads((root/'target/next/builds.json').read_text());binaries={name:root/'target/next/bin'/name for name in state}
cpus=sorted(os.sched_getaffinity(0));assert len(cpus)>=4,'need two client CPUs and independent peer/load CPUs'
env=os.environ.copy();env.update(NEXT_PEER_CPUS=str(cpus[0]),NEXT_LOAD_CPUS=str(cpus[1]),NEXT_CLIENT_CPUS=','.join(map(str,cpus[2:4])))
(out/'environment.json').write_text(json.dumps({'sources':state,'allowed_cpus':cpus,'client_cpus':cpus[2:4],'peer_cpu':cpus[0],'load_cpu':cpus[1],'kernel':os.uname().release,'machine':os.uname().machine,'binaries':{k:hashlib.sha256(v.read_bytes()).hexdigest()for k,v in binaries.items()},'scope':'native host; CPU pinning does not isolate interrupts/asynchronous kernel work; synthetic IPv4 UDP echo; fixed frozen-baseline peer and load processes'},indent=2))
def args(binary,proto,size,window,seconds,flows):return ['unshare','--net',str(binary),'client',str(binaries['baseline']),proto,str(size),str(window),str(seconds),str(flows)]
def save(row):
 with (out/'samples.jsonl').open('a')as f:f.write(json.dumps(row)+'\n')
def timed(label,proto,size,window,round_id,flows):
 cmd=args(binaries[label],proto,size,window,3,flows);t=time.monotonic();c=subprocess.Popen(cmd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
 try:
  stdout,stderr=c.communicate(timeout=30)
 finally:
  try:os.killpg(c.pid,signal.SIGKILL)
  except ProcessLookupError:pass
 rows=[json.loads(line.split('NEXT_JSON ',1)[1])for line in stdout.splitlines()if line.startswith('NEXT_JSON ')]
 row={'build':label,'protocol':proto,'size':size,'window':window,'flows':flows,'round':round_id,'exit':c.returncode,'wall':time.monotonic()-t,'stdout':stdout,'stderr':stderr,'metrics':rows};save(row)
 assert c.returncode==0 and len(rows)==1,row
 print(json.dumps({k:row[k]for k in ['build','protocol','size','window','flows','round','exit']}),flush=True)
if not a.profile_only:
 orders=list(itertools.permutations(binaries))
 for rd in range(a.rounds):
  for proto in ['udp','tcp','stealth']:
   for size,window,flows in [(64,1,1),(1200,32,1),(1200,32,8)]:
    for label in orders[rd%len(orders)]:timed(label,proto,size,window,rd,flows)
else:
 # Profile builds are separate from timing builds. Attach only after the fixed
 # peer/load processes exist and the authenticated first request has completed.
 for label in ['baseline','candidate']:
  for proto in ['udp','tcp','stealth']:
   stem=out/(label+'-'+proto);ready=stem.with_suffix('.ready');go=stem.with_suffix('.go');assert not ready.exists()and not go.exists()
   e=env|{'NEXT_PROFILE_READY':str(ready),'NEXT_PROFILE_GO':str(go)}
   binary=root/'target/next/profile-bin'/label
   with stem.with_suffix('.stdout').open('w')as stdout,stem.with_suffix('.stderr').open('w')as stderr:
    c=subprocess.Popen(args(binary,proto,1200,32,10,1),env=e,stdout=stdout,stderr=stderr,start_new_session=True)
    profiler=None
    try:
     deadline=time.monotonic()+20
     while not ready.exists():
      assert c.poll()is None,'client exited before profile readiness'
      assert time.monotonic()<deadline,'profile readiness timeout'
      time.sleep(.05)
     pid=int(ready.read_text());data=stem.with_suffix('.data')
     with stem.with_suffix('.perf-stderr').open('w')as log:
      profiler=subprocess.Popen([os.environ.get('NEXT_PERF','perf'),'record','-e','cpu-clock','-F','499','--call-graph','fp','-p',str(pid),'-o',str(data)],stdout=log,stderr=log)
      time.sleep(.5);assert profiler.poll()is None,'perf failed to attach; retain log'
      go.write_text('go');assert c.wait(timeout=25)==0
      if profiler.poll()is None:profiler.send_signal(signal.SIGINT)
      assert profiler.wait(timeout=10)==0
     report=subprocess.run([os.environ.get('NEXT_PERF','perf'),'report','--stdio','--no-children','--percent-limit','0.1','-i',str(data)],capture_output=True,text=True,check=True)
     stem.with_suffix('.report.txt').write_text(report.stdout)
    finally:
     if c.poll()is None:c.kill();c.wait()
     try:os.killpg(c.pid,signal.SIGKILL)
     except ProcessLookupError:pass
     if profiler is not None and profiler.poll()is None:profiler.kill();profiler.wait()
   print(json.dumps({'profile':label,'protocol':proto,'status':'recorded'}),flush=True)
