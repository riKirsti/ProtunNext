#!/bin/sh
set -u
result=0
run_case() {
 echo "HOUR_CASE $*"
 timeout 12 "$@" || { echo "HOUR_CASE_FAILED $*"; result=1; }
}
for round in 0 1 2 3; do
 cases='udp:portable-full-copy udp:portable-compact-inplace udp:batch-full-copy udp:batch-compact-copy udp:batch-full-inplace udp:batch-compact-inplace udp:batch-offload-compact-inplace udp:uring-compact-inplace udp:uring-offload-compact-inplace tcp:batch-full-copy tcp:batch-compact-inplace stealth:batch-buffered-full-copy stealth:batch-buffered-compact-inplace stealth:batch-unbuffered-compact-inplace'
 if [ "$round" = 1 ] || [ "$round" = 3 ]; then cases='stealth:batch-unbuffered-compact-inplace stealth:batch-buffered-compact-inplace stealth:batch-buffered-full-copy tcp:batch-compact-inplace tcp:batch-full-copy udp:uring-offload-compact-inplace udp:uring-compact-inplace udp:batch-offload-compact-inplace udp:batch-compact-inplace udp:batch-full-inplace udp:batch-compact-copy udp:batch-full-copy udp:portable-compact-inplace udp:portable-full-copy'; fi
 for spec in $cases; do
  oldifs=$IFS; IFS=:; set -- $spec; IFS=$oldifs
  run_case decision-worker "$1" "$2" 1200 "$round" 1.5 32 0 0
 done
 for variant in batch-buffered-compact-inplace batch-unbuffered-compact-inplace; do
  run_case decision-worker stealth "$variant" 64 "$round" 1 1 1 0
  run_case decision-worker stealth "$variant" 2048 "$round" 1 16 0 0
 done
 run_case decision-worker udp batch-offload-compact-inplace 1200 "$round" 1 32 0 1
 run_case decision-worker stealth batch-unbuffered-compact-inplace 1200 "$round" 1 32 0 1
done
for variant in batch-buffered-compact-inplace batch-unbuffered-compact-inplace; do
 n=0
 while [ "$n" -lt 300 ]; do
  run_case decision-worker stealth "$variant" 64 "$n" 0 1 0 0
  n=$((n+1))
 done
done
echo "HOUR_WORKER_FINISHED $result"
exit "$result"
