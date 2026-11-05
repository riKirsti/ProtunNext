#!/usr/bin/env bash
set -euo pipefail
label=${1:?}
export CARGO_TARGET_DIR="$PWD/bench-target"
cargo test --manifest-path reference/Cargo.toml --release --no-default-features --features in-place-crypto,io-uring --lib --tests --no-run --message-format=json > "results/test-bins-$label.jsonl"
python3 - "$label" <<'PY'
import json,subprocess,sys
label=sys.argv[1];seen=set()
with open('results/correctness-'+label+'.log','w') as log:
 for line in open('results/test-bins-'+label+'.jsonl'):
  r=json.loads(line);exe=r.get('executable')
  if not exe or not r.get('profile',{}).get('test') or exe in seen:continue
  seen.add(exe)
  subprocess.run(['sudo','env','PROTUN_REQUIRE_URING=1','unshare','-n','sh','-c','ip link set lo up; exec "$@"','sh',exe,'--test-threads=1'],stdout=log,stderr=log,check=True)
  if r['target']['name']=='linux_tun':
   subprocess.run(['sudo','env','PROTUN_REQUIRE_URING=1','PROTUN_NEXT_NETNS_TEST=1','unshare','-n','sh','-c','ip link set lo up; exec "$@"','sh',exe,'--ignored','--test-threads=1'],stdout=log,stderr=log,check=True)
print('CORRECTNESS_BINARIES',len(seen),label)
PY
