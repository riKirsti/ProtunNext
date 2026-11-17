# ProtunNext

Cross-platform Rust VPN runtime with WireGuard UDP, framed TCP and Stealth
transports, account integration and native application bindings.

Source releases are verified through implementation coverage, platform CI and
linked-erasure checks. The [deployment guide](docs/deployment.md) describes the
additional checks for live services and devices. ProtunNext is independent of Proton AG.

## Features

- Authenticated WireGuard sessions with replay protection, bounded packet
  ownership and in-place data encryption/decryption.
- UDP batching, framed TCP and platform-selected rustls I/O; explicit Linux
  receive coalescing for compatible caller-owned TUN descriptors.
- Managed connection lifecycle, peer updates, recovery, statistics and captures.
- Optional Proton account, certificate and Local Agent integration. Applied
  settings and subscription capabilities remain subject to server policy.
- Rust library and CLI, generated bindings, Android VpnService integration and
  an Apple NetworkExtension packet-flow adapter.

## Documentation

- [Development](docs/development.md): architecture, APIs, tests and contribution workflow.
- [Build](docs/building.md): toolchains, features and platform artifacts.
- [Deploy](docs/deployment.md): host integration, packaging and operational boundaries.
- [Contributing](CONTRIBUTING.md) and [security reporting](SECURITY.md).

## Quick start

Install stable Rust with Cargo, rustfmt and Clippy, plus the native compiler and
SDK for your platform. The Proton registry is configured in `.cargo/config.toml`.
Ordinary builds/tests require no Proton account credentials.

```sh
cargo test --locked --all-targets --all-features
cargo build --locked --release --lib --bin protun-next-cli
cargo run --locked -- check-config /path/to/private-config.conf
```

The CLI validates configuration and offers low-level Linux workers. It does not
create/configure a VPN interface or install routes/DNS for the host. Use the
managed library API for application integration; see the deployment guide.

## Repository layout

| Directory | Purpose |
|---|---|
| `src/` | Runtime, public APIs, platform adapters and release requirements ledger |
| `android/`, `apple/` | Native application integration |
| `tests/` | Functional, transport, platform and FFI regressions |
| `scripts/` | Build, packaging, provenance, repository and security checks |
| `examples/`, `benches/` | Linked erasure fixture and developer microbenchmark |
| `vendor/` | Pinned dependency source, narrow patch and upstream notices |
| `docs/` | Development, build and deployment guides |

## License

GPL-3.0-or-later; see [LICENSE](LICENSE). Vendored code retains its own
[license and attribution](vendor/proton-boringtun/LICENSE.md).
