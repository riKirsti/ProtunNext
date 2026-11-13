# Build

## Requirements

- Stable Rust with Cargo, rustfmt and Clippy, using the toolchain exercised in CI.
- A native C compiler/linker and the platform SDK. Windows needs the Visual Studio
  C++ build tools and Windows SDK; Unix needs the usual native build toolchain.
- Python 3 for repository, packaging and verification scripts.
- Network access to crates.io and the public Proton registry configured in
  `.cargo/config.toml`. Ordinary builds/tests require no account credentials.

```sh
rustup component add rustfmt clippy
cargo build --locked --release --lib --bin protun-next-cli
cargo test --locked --all-targets --all-features
```

Native outputs are under `target/release/`: the CLI, Rust library, shared library
and static library. The platform determines their names/extensions. Release uses
thin LTO, one codegen unit and the existing abort panic policy. Keep portable
defaults; do not distribute host-native binaries to an arbitrary CPU or reuse
unqualified benchmark profiles.

## Features and bindings

Default features include `local-agent` and `uniffi`. Convenience bundles are
`unix`/`linux`, `windows`, `apple` and `android`. `control-plane` provides private
network services; `test_utils`/`real-connection-tests` are development helpers;
`debug-counters` adds scoped diagnostics. These features are not interchangeable
product capabilities. A no-account native build retains private services with:

```sh
cargo build --locked --release --no-default-features --features unix --lib
```

Do not disable default account features solely to claim product performance.
Generate bindings from the actual library you package; `uniffi.toml` contains the
language naming configuration. For example, on Linux:

```sh
cargo run --locked --features bindgen --bin uniffi-bindgen -- generate --library target/release/libprotun_next.so --language python --out-dir target/generated --no-format
```

Use `.dll` on Windows or `.dylib` on macOS and select the needed language.
`scripts/check-downstream.py` compiles public API use from a separate application.

## Linux / ARM64-musl

The supported Linux contract is x64/ARM64 on the 6.12.x and 6.18.x LTS families.
Install the Rust target and a matching musl C compiler/linker before cross builds:

```sh
rustup target add aarch64-unknown-linux-musl
cargo build --locked --release --lib --bin protun-next-cli --target aarch64-unknown-linux-musl
```

Configure Cargo's target linker and the target `CC`/`AR` environment for your host
toolchain. The native ARM64 CI job demonstrates the musl build. To run actual
LTS functional guests on a disposable Linux build host:

```sh
bash scripts/test-linux-lts.sh 6.12.115
bash scripts/test-linux-lts.sh 6.18.60
```

That helper requires QEMU, BusyBox, musl tools, kernel build tools and downloads.
It does not qualify physical device behavior or other kernel families.

## Windows

Target native x64. The runtime contract is Windows 11 25H2/26H2; architecture
and supported build families are checked before Wintun creation. PowerShell 7.2+
is needed for packaging:

```powershell
cargo build --locked --release --lib --bin protun-next-cli
./scripts/package-windows.ps1
```

The packaging script downloads the pinned official Wintun archive, verifies its
SHA256, Authenticode signature and PE architecture, and writes a development
bundle to `target/windows-package/` with the driver license. It does not load or
install a VPN. `-BuildDirectory` and `-OutputDirectory` select alternate paths.

## Android

Android 16/17, minimum/NDK API 36, compile/target SDK 37; JDK 17, Gradle 9.3.1,
AGP 9.1.1 with built-in Kotlin. CI uses NDK 30.0.16248370, platform `android-37.0`
and build tools 36.0.0. Set `ANDROID_NDK_HOME` to your installed NDK:

```sh
bash scripts/build-android.sh aarch64-linux-android
```

The helper compiles native code, generates Kotlin from the Android library,
runs library tests and assembles `android/build/outputs/aar/`. Native libraries
are copied under `target/android-jni/`. CI covers arm64-v8a; other helper targets
are not device-qualified. The consuming app sets target SDK 37 and supplies the
required 16 KB deployment environment. See [Android integration](../android/README.md).

## Apple

Use an ARM64 Mac and Xcode 27. Runtime/deployment floor is macOS/iOS/iPadOS 26,
with 26/27 as the supported families; Intel/Rosetta targets are not supplied.

```sh
bash scripts/build-apple.sh
swift build --package-path apple
```

The helper builds ARM64 macOS, iOS and simulator libraries, generates Swift into
`target/apple-bindings/`, and typechecks the adapter. Compiling SDK slices does
not verify an entitled VPN provider. See [Apple integration](../apple/README.md).
