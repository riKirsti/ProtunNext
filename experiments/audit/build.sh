#!/usr/bin/env bash
# Reuse the audited two-archive build; keep variant outputs separate.
set -euo pipefail
cd "$(dirname "$0")/../.."
target=${1:?Rust target required}
python3 experiments/audit/prepare.py --baseline "${BASELINE:-f7bf03f5b4dc3981eaadedbf3a7841c5ed5ee5c1}"
mkdir -p target/erasure-audit/bin
{ rustc -vV; cargo -vV; printf 'target=%s\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=%s\n' "$target" "${RUSTFLAGS-}"; } > target/erasure-audit/toolchain.txt
# Fail on candidate fixture regressions before paying for two release builds.
for name in erase main; do
  export CARGO_TARGET_DIR="$PWD/target/erasure-audit/cargo-$name"
  export NEXT_BUILD_ID="$(python3 -c "import json; print(json.load(open('target/erasure-audit/builds.json'))['$name']['effective_sources_sha256'])")"
  source="$(python3 -c "import json; print(json.load(open('target/erasure-audit/builds.json'))['$name']['path'])")"
  if [ "$name" = erase ]; then
    cargo test --locked --lib --no-default-features --features unix --manifest-path "$source/Cargo.toml" > target/erasure-audit/probe-regressions.log 2>&1
  fi
  cargo build --locked --release --target "$target" --no-default-features --features unix --bin erasure-probe --manifest-path "$source/Cargo.toml" > "target/erasure-audit/build-$name.log" 2>&1
  cp "$CARGO_TARGET_DIR/$target/release/erasure-probe" "target/erasure-audit/bin/$name"
done
