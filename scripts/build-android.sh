#!/usr/bin/env bash
# Compile development libraries only; never installs or starts a VPN.
set -euo pipefail
cd "$(dirname "$0")/.."
: "${ANDROID_NDK_HOME:?Set ANDROID_NDK_HOME to the installed Android NDK}"
case "$(uname -s)" in
  Linux) host=linux-x86_64 ;;
  Darwin) host=darwin-x86_64 ;;
  *) echo 'Run this helper on Linux or macOS' >&2; exit 1 ;;
esac
toolchain="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$host/bin"
targets=("${@:-aarch64-linux-android}")
first=''
for target in "${targets[@]}"; do
  case "$target" in
    aarch64-linux-android) clang=aarch64-linux-android; abi=arm64-v8a ;;
    armv7-linux-androideabi) clang=armv7a-linux-androideabi; abi=armeabi-v7a ;;
    x86_64-linux-android) clang=x86_64-linux-android; abi=x86_64 ;;
    i686-linux-android) clang=i686-linux-android; abi=x86 ;;
    *) echo "Unsupported Android target: $target" >&2; exit 1 ;;
  esac
  rustup target add "$target"
  underscore="${target//-/_}"
  uppercase="$(printf '%s' "$underscore" | tr '[:lower:]' '[:upper:]')"
  env "CC_$underscore=$toolchain/${clang}36-clang" \
    "AR_$underscore=$toolchain/llvm-ar" \
    "CARGO_TARGET_${uppercase}_LINKER=$toolchain/${clang}36-clang" \
    cargo build --locked --release --lib --features android --target "$target"
  mkdir -p "target/android-jni/$abi"
  cp "target/$target/release/libprotun_next.so" "target/android-jni/$abi/"
  if [[ -z "$first" ]]; then first="$target"; fi
done
cargo run --locked --features bindgen --bin uniffi-bindgen -- generate \
  --library "target/$first/release/libprotun_next.so" --language kotlin \
  --config uniffi.toml --out-dir target/android-bindings --no-format
gradle -p android --no-daemon testDebugUnitTest assembleRelease
