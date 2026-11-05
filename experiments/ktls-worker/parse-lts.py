"""Validate guest identities/results and preserve matched rows for analysis."""
import hashlib,json,pathlib
root=pathlib.Path(__file__).resolve().parents[2];out=root/'target/next/results'
state=json.loads((root/'target/next/builds.json').read_text());rows=[];current=None;stdout=[]
text=(out/'guest.log').read_text().replace('\r','')
assert 'WORKER_KERNEL 6.18.60\n' in text
for line in text.splitlines():
 if line.startswith('WORKER_CASE '):
  assert current is None,'case ended without a result'
  _,build,proto,size,window,flows,rd=line.split();current={'build':build,'protocol':proto,'size':int(size),'window':int(window),'flows':int(flows),'round':int(rd)};stdout=[]
 elif current is not None:
  stdout.append(line)
  if line.startswith('NEXT_JSON '):
   m=json.loads(line[len('NEXT_JSON '):]);assert m['effective_sources_sha256']==state[current['build']]['effective_sources_sha256']
   assert m['kernel']=='6.18.60' and m['counters']['invalid_packets']==0
   if current['build']=='candidate' and current['protocol']=='stealth':assert any('KTLS_WORKER_READY TLS13_AES_256_GCM_SHA384' in l for l in stdout)
   rows.append(current|{'exit':0,'stdout':'\n'.join(stdout),'stderr':'','metrics':[m]});current=None
assert current is None and len(rows)==36
digests={name:hashlib.sha256((root/'target/next/bin'/name).read_bytes()).hexdigest()for name in state}
assert len(set(digests.values()))==2
(out/'samples.jsonl').write_text(''.join(json.dumps(r)+'\n'for r in rows))
(out/'environment.json').write_text(json.dumps({'sources':state,'binaries':digests,'kernel':'6.18.60','machine':'x86_64','allowed_cpus':[0,1,2,3],'client_cpus':[2,3],'peer_cpu':0,'load_cpu':1,'scope':'actual 6.18.60 x64 KVM guest; fixed peer; bounded kTLS worker prototype; one host allocation'},indent=2))
