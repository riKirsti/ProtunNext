"""Predeclared A/A and ABBA audit; raw failures are retained, never discarded."""
import argparse,hashlib,json,os,pathlib,random,signal,subprocess,time
P=argparse.ArgumentParser();P.add_argument('--smoke',action='store_true');P.add_argument('--quick',action='store_true');a=P.parse_args()
ROOT=pathlib.Path(__file__).resolve().parents[2];OUT=ROOT/'target/audit-v2/results';OUT.mkdir(parents=True,exist_ok=True)
state=json.loads((ROOT/'target/audit-v2/builds.json').read_text())
bins={k:ROOT/'target/audit-v2/bin'/k for k in state}
if a.smoke:bins={'erase':bins['erase']}
cpus=sorted(os.sched_getaffinity(0));assert len(cpus)>=4
digests={k:hashlib.sha256(v.read_bytes()).hexdigest()for k,v in bins.items()}
assert len(set(digests.values()))==len(digests)
for name,binary in bins.items():
 assert json.loads(subprocess.check_output([str(binary),'identity'],text=True))['effective_sources_sha256']==state[name]['effective_sources_sha256']
environment={'sources':state,'binaries':digests,'kernel':os.uname().release,'machine':os.uname().machine,'cpus':cpus,'fixed_endpoint':'erase','controls':'A/A uses the SAME erase binary; predeclared 5% throughput/CPU tolerance, not statistical significance','resource_policies':{'two':cpus[2:4],'shared':cpus},'aux_saturation_threshold':0.85}
(OUT/'environment.json').write_text(json.dumps(environment,indent=2))
sequence=sum(1 for _ in (OUT/"samples.jsonl").open()) if (OUT/"samples.jsonl").exists() else 0
def save(row):
 with (OUT/'samples.jsonl').open('a')as f:f.write(json.dumps(row)+'\n')
def sample(build,proto,case,role,block,slot,resources='two'):
 global sequence
 size,window,flows,shape,rate=case;sequence+=1
 env=os.environ.copy();env.update(NEXT_PEER_CPUS=str(cpus[0]),NEXT_LOAD_CPUS=str(cpus[1]),NEXT_CLIENT_CPUS=','.join(map(str,environment['resource_policies'][resources])),NEXT_LOAD_SHAPE=shape,NEXT_WARM_SECONDS='1')
 if rate:env['NEXT_RATE_PPS']=str(rate)
 else:env.pop('NEXT_RATE_PPS',None)
 seconds=1 if a.smoke else 5
 cmd=['unshare','--net',str(bins[build]),'client',str(ROOT/'target/audit-v2/bin/erase'),proto,str(size),str(window),str(seconds),str(flows)]
 start=time.monotonic();proc=subprocess.Popen(cmd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
 stdout=stderr='';status='completed';code=None
 try:
  stdout,stderr=proc.communicate(timeout=40);code=proc.returncode
 except subprocess.TimeoutExpired:
  status='timeout';os.killpg(proc.pid,signal.SIGKILL);stdout,stderr=proc.communicate()
 finally:
  try:os.killpg(proc.pid,signal.SIGKILL)
  except ProcessLookupError:pass
 metrics=[json.loads(l[10:])for l in stdout.splitlines()if l.startswith('NEXT_JSON ')]
 row={'sample':sequence,'build':build,'protocol':proto,'size':size,'window':window,'flows':flows,'shape':shape,'rate_pps':rate,'role':role,'block':block,'slot':slot,'resources':resources,'exit':code,'status':status,'wall':time.monotonic()-start,'stdout':stdout,'stderr':stderr,'metrics':metrics};save(row)
 assert code==0 and len(metrics)==1,row
 m=metrics[0];assert m['effective_sources_sha256']==state[build]['effective_sources_sha256']
 assert m['peer']['effective_sources_sha256']==m['load']['effective_sources_sha256']==state['erase']['effective_sources_sha256']
 assert m['peer']['timed'] and m['counters']['invalid_packets']==0
 assert 0< m['load']['bytes'] and 0<m['client_cpu_ns_per_byte']
 print(json.dumps({k:row[k]for k in ['sample','build','protocol','role','block','slot','resources','status']}),flush=True)
if a.smoke:
 for proto in ['udp','tcp','stealth']:
  for shape in ['burst','sliding']:sample('erase',proto,(1200,32,1,shape,0),'smoke',proto+shape,0)
else:
 cases=[(1200,32,1,'burst',0),(1200,32,1,'sliding',0),(1200,32,8,'sliding',0),(1200,32,1,'burst',10000),(64,1,1,'burst',0)]
 if a.quick:cases=cases[:2]
 rng=random.Random(20261105)
 for proto in ['udp','tcp','stealth']:
  for ci,case in enumerate(cases):
   block=f'{proto}-{ci}'
   # Explicit same-executable control labels; differing hashes are NOT required
   # for A/A, but are required for every implementation comparison.
   for slot in range(4):sample('erase',proto,case,'aa-'+('a'if slot in [0,3]else'b'),block,slot)
   candidates=['main','gro','parallel']+(['ktls']if proto=='stealth'else[]);rng.shuffle(candidates)
   for variant in candidates:
    order=['erase',variant,variant,'erase']
    if rng.getrandbits(1):order=[variant,"erase","erase",variant]
    for slot,build in enumerate(order):sample(build,proto,case,'compare-'+variant,block,slot)
 # Sensitivity: give the client access to all available CPUs, rather than
 # silently extrapolating a two-CPU clamp to general multicore machines.
 for proto in ['udp','tcp','stealth']:
  for shape in ['burst','sliding']:
   case=(1200,32,1,shape,0);block='shared-'+proto+'-'+shape
   for slot,build in enumerate(['erase','parallel','parallel','erase']):sample(build,proto,case,'parallel-cpu-sensitivity',block,slot,'shared')
