# Development

ProtunNext is a Rust library and low-level CLI targeting the public functionality
of Protun v3.0.5. Implementation coverage and live/device acceptance are separate.
The machine-readable ledger in `src/requirements.json` powers `status`,
`verify-code-parity` and `verify-parity`; do not mark unverified requirements as
complete to obtain a passing release gate.

## Architecture

The packet engine transfers owned reusable buffers through bounded queues.
WireGuard session handling uses the pinned ring-backed `proton-boringtun`
dependency with a narrow in-place data-crypto extension. Authentication and replay
commit precede delivery. Plaintext and tracked stale tails are cleared on recycle;
destruction clears complete allocations. Control messages, timers, cancellation
and shutdown remain responsive under backpressure.

The production receiver is serial. Complete TCP frames are extracted directly;
borrowed TLS input uses MTU-sized bounded prefetch. Linux/Android use UDP mmsg
batching with negotiated GSO/GRO where available, retaining datagram boundaries.
UDP batch size is 16 and never waits to fill. TLS uses unbuffered rustls on
Linux/Android and buffered rustls on Windows/Apple. Ordinary TLS verifies standard
certificates; Stealth retains its explicit outer policy and authenticates inner
WireGuard before delivery. Never turn Stealth into a fallback for ordinary TLS.

`Connection` manages packet-worker lifecycle and coalesced state/statistics.
The compatibility API supervisor handles peer updates, recovery, TUN replacement,
captures and account integration. The optional account thread uses published
Muon/RCRL/Local Agent SDKs. Private API/DNS/control sockets use a separate private
stack; normal customer traffic bypasses it. Requested paid settings are distinct
from applied settings and server refusals. The host supplies secure persistence.

Platform adapters own only resources explicitly assigned to them. Unix attaches
to a caller-owned descriptor/uplink; Android supplies VpnService protection and
network binding; Apple bridges public packet-flow APIs; Windows uses native
Wintun/IP Helper management. Cleanup must never enumerate/delete unrelated
interfaces or routes. See [deployment](deployment.md) for the host contract.

## Local checks

```sh
python scripts/check_repository.py
python -m unittest discover -s scripts -p 'test_*.py'
python -m unittest discover -s scripts/security -p 'test_*.py'
python scripts/verify-vendor.py
cargo fmt --package protun-next --check
cargo test --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo clippy --locked --all-targets --no-default-features --features unix -- -D warnings
python scripts/check-downstream.py
cargo run --locked -- verify-code-parity
cargo run --locked -- verify-parity
```

The last command deliberately fails until outstanding live/device acceptance is
complete. Passing compilation or an offline fixture is not release approval.
Default tests use synthetic data and do not configure host networking. Real TUN
tests are opt-in; CI runs them in disposable namespaces. Account test helpers are
opt-in and do not log in merely because a feature is enabled.

CI checks Linux hosts, actual supported LTS kernels, ARM64-musl, Windows native
bindings, Apple SDK slices and Android packaging. Release security CI checks
linked erasure code on x64/ARM64 and rejects an isolated wipe-free control. That
control is never deployed or used as a tunnel. The microbenchmark under
`benches/` measures a local packet path and does not establish VPN throughput.

## Changes and privacy

Keep APIs, wire formats, key/session/replay ownership, partial-write ownership,
bounded memory and descriptor ABIs intact. Add meaningful regressions when
changing behavior. See [contribution guidance](../CONTRIBUTING.md).

Do not commit local machine paths, personal emails, real device/account data,
private configurations, captures or build/profiling output. Use documentation
address ranges and clearly marked synthetic fixtures. Keep legitimate project
identifiers and required third-party copyright/license notices intact.
Repository hygiene checks inspect project-owned text and local Markdown links;
they are not a complete secret scanner or a purge of Git/GitHub history.

The vendored snapshot is checksum-pinned by `UPSTREAM.json`; only its three
declared extension files differ from the registry archive. Keep upstream files,
licenses, notices and the patch together. Performance research has concluded;
new experiments require an explicit request rather than a maintenance cleanup.
