#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
[[ "$(uname -m)" == arm64 ]] || { echo 'Apple builds require an ARM64 host' >&2; exit 1; }
[[ "$(xcodebuild -version | head -1)" == 'Xcode 27.'* ]] || { echo 'Select Xcode 27 first' >&2; exit 1; }
export MACOSX_DEPLOYMENT_TARGET=26.0 IPHONEOS_DEPLOYMENT_TARGET=26.0
for target in aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim; do
    rustup target add "$target"
    cargo build --locked --release --lib --features apple --target "$target"
done
cargo run --locked --features bindgen --bin uniffi-bindgen -- generate \
    --library target/aarch64-apple-darwin/release/libprotun_next.dylib \
    --language swift --config uniffi.toml --out-dir target/apple-bindings --no-format
swiftc -typecheck target/apple-bindings/ProtunNext.swift -I target/apple-bindings \
    -Xcc -fmodule-map-file=target/apple-bindings/ProtunNextFFI.modulemap
swift build --package-path apple
