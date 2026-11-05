"""Report paired blocks, same-binary noise, failures and auxiliary saturation."""
import argparse,collections,json,math,pathlib,statistics
P=argparse.ArgumentParser();P.add_argument('folders',nargs='+');P.add_argument('--output',required=True);a=P.parse_args()
def gm(xs):return math.exp(statistics.mean(math.log(x) for x in xs))
def values(rows,key):
 if key=='Mbps':return [r['metrics'][0]['load']['Mbps']for r in rows]
 return [r['metrics'][0]['client_cpu_ns_per_byte']for r in rows]
def ratio(b,base):return {k:100*(gm(values(b,k))/gm(values(base,k))-1)for k in ['Mbps','CPU_ns_per_byte']}
def case(r):return tuple(r[k]for k in ['protocol','size','window','flows','shape','rate_pps','resources'])
report={'method':'Two observations per side in ABBA/BAAB; geometric means within each block; A/A engineering tolerance 5% for throughput AND CPU. No significance claim. Failed/incomplete blocks retained. Auxiliary >=0.85 CPU core flags capacity limitation.','allocations':[],'blocks':[],'failures':[]}
for folder in a.folders:
 f=pathlib.Path(folder);env=json.loads((f/'environment.json').read_text());rows=[json.loads(l)for l in (f/'samples.jsonl').read_text().splitlines()];seen=set()
 valid=[]
 for r in rows:
  assert r['sample'] not in seen;seen.add(r['sample'])
  good=r['exit']==0 and r['status']=='completed' and len(r['metrics'])==1
  if good:
   m=r['metrics'][0];assert m['effective_sources_sha256']==env['sources'][r['build']]['effective_sources_sha256']
   assert (m['protocol'],m['size'],m['window'],m['flows'],m['load']['shape'])==(r['protocol'],r['size'],r['window'],r['flows'],r['shape'])
   assert m['load']['bytes']==m['load']['packets']*r['size'] and m['load']['bytes']>0
   assert math.isclose(m['load']['Mbps'],m['load']['bytes']*8/m['load']['seconds']/1e6,rel_tol=1e-10)
   assert math.isclose(m['client_cpu_ns_per_byte'],m['client_cpu_seconds']*1e9/m['load']['bytes'],rel_tol=1e-10)
   assert m['peer']['effective_sources_sha256']==m['load']['effective_sources_sha256']==env['sources']['erase']['effective_sources_sha256']
   assert m['counters']['invalid_packets']==0 and m['peer']['timed']
   assert m['state']=='TunnelEstablished' and m['counters']['outer_io_errors']==m['counters']['outer_eof']==0
   if r['protocol']=='stealth':
    tls=json.loads(m['peer']['tls'])
    assert tls['version']=='Some(TLSv1_3)' and tls['suite']=='Some(TLS13_AES_256_GCM_SHA384)'
    assert tls['vectored']==r['peer_vectored'] and sum(tls['encrypted_record_buckets_le_2k_le_8k_gt_8k'])>0
   if r['build']=='ktls' and r['protocol']=='stealth':
    assert r['stdout'].count('KTLS_WORKER_READY TLS13_AES_256_GCM_SHA384')==1,'kTLS identity is insufficient without actual handoff'
   valid.append(r)
  else:report['failures'].append({'allocation':str(f),'sample':r['sample'],'build':r['build'],'role':r['role'],'protocol':r['protocol'],'stderr':r['stderr'],'status':r['status'],'exit':r['exit']})
 report['allocations'].append({'folder':str(f),'kernel':env['kernel'],'arch':env['machine'],'samples':len(rows),'valid':len(valid),'binaries':env['binaries']})
 groups=collections.defaultdict(list)
 for r in rows:
  if r['role']=='smoke':continue
  role='aa'if r['role'].startswith('aa-')else r['role']
  groups[(case(r),role,r['block'])].append(r)
 controls={}
 for (cell,role,block),group in groups.items():
  if role!='aa':continue
  complete=len(group)==4 and sorted(r['slot']for r in group)==[0,1,2,3] and all(r in valid for r in group)
  aa=ratio([r for r in group if r['role']=='aa-b'],[r for r in group if r['role']=='aa-a']) if complete else None
  controls[cell]={'complete':complete,'delta':aa,'passes':bool(aa and all(abs(v)<=5 for v in aa.values()))}
 for (cell,role,block),group in groups.items():
  entry={'allocation':str(f),'arch':env['machine'],'kernel':env['kernel'],'case':dict(zip(['protocol','size','window','flows','shape','rate_pps','resources'],cell)),'role':role,'block':block,'slots':len(group),'aa':controls.get(cell)}
  complete=len(group)==4 and sorted(r['slot']for r in group)==[0,1,2,3] and all(r in valid for r in group);entry['complete']=complete
  if complete:
   if role=='aa':base=[r for r in group if r['role']=='aa-a'];b=[r for r in group if r['role']=='aa-b'];labels=['same-erase-A','same-erase-B']
   elif role=='peer-writev-sensitivity':base=[r for r in group if not r['peer_vectored']];b=[r for r in group if r['peer_vectored']];labels=['legacy-peer-writev','corrected-peer-writev']
   elif role=='compare-main':base=[r for r in group if r['build']=='main'];b=[r for r in group if r['build']=='erase'];labels=['main','erase']
   else:base=[r for r in group if r['build']=='erase'];b=[r for r in group if r['build']!='erase'];labels=['erase',b[0]['build']]
   assert len(base)==len(b)==2
   entry.update(baseline=labels[0],candidate=labels[1],delta=ratio(b,base),baseline_Mbps=gm(values(base,'Mbps')),candidate_Mbps=gm(values(b,'Mbps')),baseline_CPU_ns_per_byte=gm(values(base,'CPU')),candidate_CPU_ns_per_byte=gm(values(b,'CPU')))
   aux=[]
   for r in group:
    m=r['metrics'][0];p=m['peer'];l=m['load'];aux.append({'sample':r['sample'],'peer_cores':p['cpu_seconds']/p['seconds'],'load_cores':l['cpu_seconds']/l['seconds'],'client_cores':m['client_cpu_seconds']/m['seconds']})
   entry.update(cpu_utilization=aux,aux_saturated=any(max(v['peer_cores'],v['load_cores'])>=.85 for v in aux))
   entry['rate_achieved']=all(abs(r['metrics'][0]['load']['packets']/r['metrics'][0]['load']['seconds']/cell[-2]-1)<=.05 for r in group) if cell[-2] else None
   entry['capacity_ranking_eligible']=bool(entry['aa'] and entry['aa']['passes'] and not entry['aux_saturated'] and cell[-1]=='two' and cell[-2]==0)
  report['blocks'].append(entry)
path=pathlib.Path(a.output);path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(report,indent=2))
print(json.dumps({'allocations':len(report['allocations']),'samples':sum(x['samples']for x in report['allocations']),'failures':len(report['failures']),'blocks':len(report['blocks']),'complete_blocks':sum(x['complete']for x in report['blocks']),'AA_pass':sum(x['aa']['passes']for x in report['blocks']if x['role']=='aa'),'AA_total':sum(x['role']=='aa'for x in report['blocks'])}))
