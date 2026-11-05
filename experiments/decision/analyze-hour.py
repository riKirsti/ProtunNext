"""Parse immutable artifacts; pair only within the same run/runner/workload/round."""
import pathlib,json,re,sys,statistics,collections,random
root=pathlib.Path(sys.argv[1]).resolve()
PRIMARY_WORKER=41949204026
PRIMARY_COMPONENT=41949203991
PRIMARY_PGO=41949032738
PRIMARY_BRIDGE=41950384745
APPLE_RUN=41950699980
records=[];failures=[];artifacts=[];seen=set();environments=[]
for folder in sorted(p for p in root.iterdir() if p.is_dir()):
 m=folder/'artifact-manifest.json'
 if not m.exists():continue
 meta=json.loads(m.read_text());aid=meta['artifact_id']
 if aid in seen:continue
 seen.add(aid);artifacts.append(meta|{'folder':folder.name})
 for f in sorted(folder.iterdir()):
  if f.suffix not in ('.log','.jsonl'):continue
  if f.name.startswith('test-bins'):continue
  kernel=None;arch='aarch64' if '-arm' in folder.name else 'x86_64';cpus=None;clock=None;last_case=None
  env=folder/'environment.txt'
  if not env.exists():env=folder/'host.txt'
  if env.exists():
   first=env.read_text(errors='replace').splitlines()[0].split()
   if len(first)>2:kernel=first[2]
   if 'arm64' in first or 'aarch64' in first:arch='aarch64'
  for ln,line in enumerate(f.read_text(errors='replace').splitlines(),1):
   if line.startswith('DECISION_KERNEL '):kernel=line.split()[1]
   if line.startswith('HOUR_CASE '):last_case=line[10:]
   if line.startswith('HOUR_ENV '):
    match=re.search(r'cpus=(\d+) clock=(\S*)',line)
    if match:cpus=int(match[1]);clock=match[2]
    environments.append({'artifact_id':aid,'run':meta['run_id'],'folder':folder.name,'cpus':cpus,'clock':clock,'kernel':kernel})
   if ('HOUR_CASE_FAILED' in line or 'DECISION_CASE_FAILED' in line or 'panicked at' in line or re.match(r'(HOUR_WORKER_FINISHED|DECISION_FINISHED) [1-9]',line) or line.startswith('test result: FAILED')):
    failures.append({'artifact_id':aid,'run':meta['run_id'],'file':f.name,'line':ln,'case':last_case,'text':line})
   pos=line.find('HOUR_JSON ');prefix=10
   if pos<0:pos=line.find('DECISION_JSON ');prefix=14
   payload=line[pos+prefix:] if pos>=0 else line.strip()
   if not payload.startswith('{'):continue
   try:r=json.loads(payload)
   except json.JSONDecodeError:continue
   if r.get('test')=='environment':kernel=r['kernel'].strip();arch=r.get('architecture',arch);continue
   if 'Mbps' not in r and r.get('test') not in ('packet-pool-component','real-tun-boundary-component'):continue
   r.update(artifact_id=aid,run=meta['run_id'],artifact=folder.name,file=f.name,line=ln,kernel=r.get('kernel',kernel),arch=r.get('arch',arch),guest_cpus=cpus,guest_clock=clock)
   records.append(r)

# Three repeated executions on ONE laptop, not three independent machines.
windows=root/'local-windows'
if windows.exists():
 for f in list(windows.glob('framed-1GiB-*.jsonl'))+list(windows.glob('adapter-256MiB-*.jsonl')):
  for ln,line in enumerate(f.read_text(errors='replace').splitlines(),1):
   if not line.startswith('{'):continue
   r=json.loads(line);r.update(artifact_id='local-windows',run='local-windows',artifact='local-windows',file=f.name,line=ln,kernel='Windows 11 reported 26H2, build 26300.9818',arch='x86_64',guest_cpus=None,guest_clock=None);records.append(r)

def summary(values):
 values=[v for v in values if v is not None]
 if not values:return None
 return {'median':statistics.median(values),'min':min(values),'max':max(values)}

pairs=[]
def add(a,b,family,label,metric='Mbps'):
 if not b:return
 speed=100*(a[metric]/b[metric]-1) if metric=='Mbps' else 100*(b[metric]/a[metric]-1)
 item={'family':family,'kernel':a['kernel'],'arch':a['arch'],'transport':a.get('transport'),'payload':a.get('payload',a.get('size',a.get('chunk'))),'window':a.get('window'),'candidate':label,'round':a['round'],'artifact_id':a['artifact_id'],'run':a['run'],'speed_pct':speed,'candidate_value':a[metric],'baseline_value':b[metric]}
 for field,label in [('worker_cpu_ns_per_byte','worker_cpu_pct'),('whole_process_cpu_ns_per_byte','whole_cpu_pct'),('process_peak_rss_kib','rss_pct'),('latency_p99_us','p99_latency_pct'),('thread_cpu_ns_per_byte','component_cpu_pct')]:
  if a.get(field) is not None and b.get(field) and a[field]>0:item[label]=100*(a[field]/b[field]-1)
 pairs.append(item)

worker=[r for r in records if r['run']==PRIMARY_WORKER and r.get('test')=='full-worker-controlled-peer' and r['delivered_bytes']>0]
idx={(r['artifact_id'],r['transport'],r['variant'],r['payload'],r['round'],r['vnet'],r['window']):r for r in worker}
comparisons=[('batch-full-copy','portable-full-copy','mmsg-vs-portable'),('batch-compact-inplace','batch-compact-copy','inplace-compact'),('batch-full-inplace','batch-full-copy','inplace-full'),('batch-compact-copy','batch-full-copy','compact-copy'),('batch-compact-inplace','batch-full-inplace','compact-inplace'),('batch-offload-compact-inplace','batch-compact-inplace','outer-UDP-offloads'),('uring-compact-inplace','batch-compact-inplace','uring-no-offloads'),('uring-offload-compact-inplace','batch-offload-compact-inplace','uring-with-offloads'),('batch-unbuffered-compact-inplace','batch-buffered-compact-inplace','unbuffered-TLS13'),('batch-unbuffered-compact-inplace-tls12','batch-buffered-compact-inplace-tls12','unbuffered-TLS12'),('batch-buffered-compact-inplace','batch-buffered-compact-copy','inplace-buffered'),('batch-buffered-full-inplace','batch-buffered-full-copy','inplace-full-buffered'),('batch-buffered-compact-copy','batch-buffered-full-copy','compact-buffered'),('batch-buffered-compact-inplace','batch-buffered-full-inplace','compact-inplace-buffered'),('batch-buffered-full-inplace','batch-full-copy','inplace-full'),('batch-buffered-compact-copy','batch-full-copy','compact-copy'),('batch-compact-inplace','batch-buffered-compact-copy','inplace-compact'),('batch-compact-inplace','batch-buffered-full-inplace','compact-inplace')]
for a,b,label in comparisons:
 for r in worker:
  if r['variant']==a and not r['vnet']:add(r,idx.get((r['artifact_id'],r['transport'],b,r['payload'],r['round'],False,r['window'])),'worker',label)
for r in worker:
 if r['vnet']:add(r,idx.get((r['artifact_id'],r['transport'],r['variant'],r['payload'],r['round'],False,r['window'])),'worker','TUN-vnet-aggregation (1s vs 1.5s samples)')
 if r['variant'].startswith('batch-limit'):
  add(r,idx.get((r['artifact_id'],r['transport'],'batch-limit16-compact-inplace',r['payload'],r['round'],False,r['window'])),'worker',r['variant'].split('-')[1]+'-vs-limit16')

component=[r for r in records if r['run']==PRIMARY_COMPONENT]
udp=[r for r in component if r.get('test')=='udp-socket-component']
idxu={(r['artifact_id'],r['size'],r['round'],r['mode'],r['gso'],r['gro']):r for r in udp}
for r in udp:
 if r['mode']=='batch' and not (r['gso'] or r['gro']):add(r,idxu.get((r['artifact_id'],r['size'],r['round'],'single',False,False)),'socket','mmsg-vs-single')
 if r['mode']=='uring128':add(r,idxu.get((r['artifact_id'],r['size'],r['round'],'batch',r['gso'],r['gro'])),'socket',f'uring128-vs-batch:GSO={r["gso"]},GRO={r["gro"]}')
tls=[r for r in records if r['run'] in (PRIMARY_COMPONENT,APPLE_RUN,'local-windows') and 'tls' in r and 'mode' in r]
idxt={(r['artifact_id'],r['file'],r['round'],r['direction'],r['chunk'],r['tls'],r['mode']):r for r in tls}
for r in tls:
 if r['mode']=='unbuffered':
  b=idxt.get((r['artifact_id'],r['file'],r['round'],r['direction'],r['chunk'],r['tls'],'buffered'))
  add(r,b,'TLS-component',f'{r["direction"]}:{r["tls"]}',metric='seconds')
parallel=[r for r in records if r['run'] in (PRIMARY_PGO,PRIMARY_COMPONENT) and r.get('test')=='shared-session-crypto']
idxp={(r['artifact_id'],r['round'],r['size'],r['workers']):r for r in parallel}
for r in parallel:
 if r['workers']>1:add(r,idxp.get((r['artifact_id'],r['round'],r['size'],1)),'shared-session-primitive',str(r['workers'])+'-workers')
pgo=[r for r in records if r['run']==PRIMARY_PGO and r.get('test')=='full-worker-controlled-peer' and 'full-worker-pgo-' in r['file'] and r['round']!=99]
idxg={(r['artifact_id'],r['round'],r['payload'],r['transport'],r['file']):r for r in pgo}
for r in pgo:
 if r['file']=='full-worker-pgo-pgo.log':add(r,idxg.get((r['artifact_id'],r['round'],r['payload'],r['transport'],'full-worker-pgo-baseline.log')),'PGO-whole-fixture','profile-use-vs-baseline')
bridge=[r for r in records if r['run']==PRIMARY_BRIDGE and r.get('test')=='full-worker-controlled-peer']
idxb={(r['artifact_id'],r['round'],r['payload'],r['transport'],r['file']):r for r in bridge}
for r in bridge:
 if r['file']=='bridge-production.log':add(r,idxb.get((r['artifact_id'],r['round'],r['payload'],r['transport'],'bridge-reference.log')),'production-bridge','PR2-f53-vs-selected-reference')

groups=collections.defaultdict(list)
for r in pairs:groups[tuple((k,r.get(k)) for k in ['family','kernel','arch','transport','payload','window','candidate'])].append(r)
stats=[]
for key,rows in groups.items():
 g=dict(key);hosts=collections.defaultdict(list)
 for r in rows:hosts[r['artifact_id']].append(r['speed_pct'])
 rng=random.Random(20261104);draws=[];h=list(hosts)
 for _ in range(3000):
  sample=[]
  for host in rng.choices(h,k=len(h)):
   v=hosts[host];sample.extend(rng.choices(v,k=len(v)))
  draws.append(statistics.median(sample))
 draws.sort()
 g.update(pairs=len(rows),runner_allocations=len(h),speed=summary([r['speed_pct'] for r in rows]),worker_cpu=summary([r.get('worker_cpu_pct') for r in rows]),whole_cpu=summary([r.get('whole_cpu_pct') for r in rows]),rss=summary([r.get('rss_pct') for r in rows]),p99_latency=summary([r.get('p99_latency_pct') for r in rows]),bootstrap95=[draws[74],draws[2924]],host_medians=[statistics.median(hosts[t]) for t in h])
 stats.append(g)
cold=collections.defaultdict(list)
for r in records:
 if r['run']==PRIMARY_WORKER and r.get('test')=='full-worker-controlled-peer' and r['delivered_bytes']==0:cold[(r['kernel'],r['arch'],r['variant'])].append(r)
cold_summary=[{'kernel':k[0],'arch':k[1],'variant':k[2],'successful_starts':len(v),'runners':len({r['artifact_id'] for r in v}),'cold_ms':summary([r['cold_echo_ms'] for r in v])} for k,v in cold.items()]
out={'primary_runs':{'worker':PRIMARY_WORKER,'components':PRIMARY_COMPONENT,'PGO':PRIMARY_PGO},'artifacts':artifacts,'environments':environments,'records':records,'failures':failures,'pairs':pairs,'statistics':stats,'cold':cold_summary}
(root/'analysis.json').write_text(json.dumps(out,indent=2))
print(json.dumps({'artifacts':len(artifacts),'records':len(records),'primary_worker_samples':len(worker),'primary_pgo_samples':len(pgo),'comparisons':len(stats),'failure_lines':len(failures),'cold':cold_summary},indent=2))
lines=['Paired median changes; percentages are not additive. Positive throughput is faster; negative CPU/RSS/latency is lower. CI is exploratory hierarchical bootstrap with few runner allocations.','', '| Family | Kernel / arch | Transport / bytes / window | Candidate | Pairs / runners | Throughput change | Worker CPU/B | 95% interval |','|---|---|---|---|---:|---:|---:|---:|']
for r in sorted(stats,key=lambda r:(r['family'],r['kernel'] or '',r['arch'],r['transport'] or '',r['payload'] or 0,r['candidate'])):
 cpu='—' if r['worker_cpu'] is None else f'{r["worker_cpu"]["median"]:+.2f}%'
 lines.append(f'| {r["family"]} | {r["kernel"]} / {r["arch"]} | {r["transport"] or "—"} / {r["payload"]} / {r["window"] or "—"} | {r["candidate"]} | {r["pairs"]} / {r["runner_allocations"]} | {r["speed"]["median"]:+.2f}% | {cpu} | [{r["bootstrap95"][0]:+.2f}, {r["bootstrap95"][1]:+.2f}]% |')
(root/'PAIRED-RESULTS.md').write_text('\n'.join(lines)+'\n')
