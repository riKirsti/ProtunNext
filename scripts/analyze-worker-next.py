"""Audit provenance before producing matched performance tables.

Each directory is one independently allocated host. Repeated rounds within that
host are not independent machines. Report each allocation rather than presenting
a narrow confidence interval from correlated packet samples.
"""
import argparse,json,pathlib,statistics,math
p=argparse.ArgumentParser();p.add_argument('directories',nargs='+');p.add_argument('--against',default='baseline');p.add_argument('--candidate',default='candidate');p.add_argument('--output');a=p.parse_args()
result=[]
for directory in a.directories:
 root=pathlib.Path(directory);env=json.loads((root/'environment.json').read_text());digests=env['binaries']
 assert len(set(digests.values()))==len(digests),'identical executables: refuse performance conclusions'
 rows=[json.loads(line)for line in (root/'samples.jsonl').read_text().splitlines()]
 assert rows and all(r['exit']==0 and len(r['metrics'])==1 for r in rows)
 for r in rows:
  m=r['metrics'][0]
  assert m['effective_sources_sha256']==env['sources'][r['build']]['effective_sources_sha256']
  assert m['counters']['invalid_packets']==0
 cells=sorted({(r['protocol'],r['size'],r['window'],r['flows'])for r in rows})
 for cell in cells:
  pair={}
  for label in [a.against,a.candidate]:
   pair[label]={r['round']:r['metrics'][0]for r in rows if r['build']==label and (r['protocol'],r['size'],r['window'],r['flows'])==cell}
  rounds=sorted(pair[a.against]);assert rounds and rounds==sorted(pair[a.candidate])
  first=[pair[a.against][r]for r in rounds];second=[pair[a.candidate][r]for r in rounds]
  gm=statistics.geometric_mean
  def ratio(get):
   if any(get(m) is None for m in first+second):return None
   return 100*(gm(get(n)/get(b)for b,n in zip(first,second))-1)
  item={'allocation':root.name,'kernel':env['kernel'],'machine':env['machine'],
    'protocol':cell[0],'size':cell[1],'window':cell[2],'flows':cell[3],
    'against':a.against,'candidate':a.candidate,'pairs':len(rounds),
    'baseline_mbps':gm(m['load']['Mbps']for m in first),'candidate_mbps':gm(m['load']['Mbps']for m in second),
    'throughput_change_pct':ratio(lambda m:m['load']['Mbps']),
    'client_cpu_per_byte_change_pct':ratio(lambda m:m['client_cpu_ns_per_byte']),
    'p99_change_pct':ratio(lambda m:m['load']['p99_us']),
    'client_rss_kib_baseline':statistics.median(m['client_rss_kib']for m in first),
    'client_rss_kib_candidate':statistics.median(m['client_rss_kib']for m in second),
    'max_client_cpu_cores':max(m['client_cpu_seconds']/m['seconds']for m in first+second),
    'max_load_cpu_cores':max(m['load']['cpu_seconds']/m['load']['seconds']for m in first+second),
    'peer_busy_polls':True,'failures':0}
  result.append(item)
text=json.dumps(result,indent=2)
if a.output:pathlib.Path(a.output).write_text(text)
print(text)
