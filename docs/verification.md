# Verification status

Development evidence, not a release acceptance record.

## Executed locally on Windows x86-64

* Rust 1.99.1/MSVC, `cargo test --all-targets --all-features`.
* Actual WireGuard Noise handshake and authenticated IP packet exchange.
* Replay and tampered-tag rejection; clock regression and output bounds.
* Real TCP and nonblocking TLS loopback carrying the WireGuard handshake and encrypted IP exchange.
* Standard TLS certificate verification and wrong-server-name rejection.
* Framing fragmentation, prediction context and reset/error rules.
* Partial writes/backpressure with exact output preservation.
* Packet ownership/bounded queues, config parsing/redaction, PCAP limits,
  NAT-PMP request/refusal, candidate selection and incomplete-parity rejection.
* 1,024 established crypto/framing packet cycles with zero observed per-thread
  allocator calls after warm-up. This excludes startup/rekey/TLS/kernel work.

## Executed in GitHub Actions

[Run 41220824642](https://github.com/riKirsti/ProtunNext/actions/runs/41220824642),
commit `a4e8f75b139bdd8ba6e9de15ec76044a380dd31c`, passed on Windows,
Linux and macOS: all-feature tests, formatting, Clippy, release build and the
expected rejection of incomplete parity. Its isolated Linux network-namespace
job passed the real kernel TUN -> WireGuard UDP -> reference peer -> TUN echo.

The subsequent shared TCP/TLS TUN worker has a new three-transport integration
test. Its success must be recorded from its own workflow run, not inferred from
the earlier UDP result.

## Cross compilation

The development laptop successfully built the release executable for
`aarch64-unknown-linux-musl`. This proves compilation/linking, not execution on
Travel or any target throughput result.

## Not yet verified
* Complete Proton WireGuard TCP or Stealth client integration.
* Real Proton server/account interoperability and all Plus capabilities.
* Cross-platform connection lifecycle and platform bindings.
* AArch64 router throughput, syscall counts or a speedup versus v3.0.5.

`docs/parity.json` is the authoritative incomplete-feature ledger. Its statuses
distinguish local implementation from live verification. Release readiness fails
while any required item is missing; it must not be bypassed to label a prototype
feature-complete.
