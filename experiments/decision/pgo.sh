#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR="$PWD/bench-target"
args=(--manifest-path reference/Cargo.toml --release --no-default-features --features in-place-crypto,io-uring --bin decision-worker)
cargo build "${args[@]}"
cp bench-target/release/decision-worker bench-target/release/worker-baseline
export RUSTFLAGS="-Cprofile-generate=$PWD/pgo-profiles"
cargo build "${args[@]}"
mkdir -p pgo-profiles results
for proto in udp tcp stealth; do
  if [ "$proto" = stealth ]; then variant=batch-unbuffered-compact-inplace; else variant=batch-offload-compact-inplace; fi
  for size in 256 1000; do
    sudo -n unshare -n bench-target/release/decision-worker "$proto" "$variant" "$size" 99 0.5 32 0 0 >> results/full-worker-pgo-training.log 2>&1
  done
done
tool="$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/host: //p')/bin/llvm-profdata"
"$tool" merge -o "$PWD/worker.profdata" pgo-profiles/*.profraw
export RUSTFLAGS="-Cprofile-use=$PWD/worker.profdata -Cllvm-args=-pgo-warn-missing-function"
cargo build "${args[@]}"
cp bench-target/release/decision-worker bench-target/release/worker-pgo
sha256sum bench-target/release/worker-baseline bench-target/release/worker-pgo worker.profdata > results/full-worker-pgo-hashes.txt
for round in 0 1 2 3; do
 for proto in udp tcp stealth; do
  if [ "$proto" = stealth ]; then variant=batch-unbuffered-compact-inplace; else variant=batch-offload-compact-inplace; fi
  for size in 64 1200; do
   if [ $((round%2)) = 0 ]; then builds='baseline pgo'; else builds='pgo baseline'; fi
   for build in $builds; do
    echo "PGO_CASE $build $round $proto $size" >> results/full-worker-pgo-$build.log
    sudo -n unshare -n "bench-target/release/worker-$build" "$proto" "$variant" "$size" "$round" 1 32 0 0 >> "results/full-worker-pgo-$build.log" 2>&1
   done
  done
 done
done
bash experiments/decision/correctness.sh profile-used
