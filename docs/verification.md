# Verification status

Development evidence, not a release acceptance record.

## Executed locally on Windows x86-64

* Rust 1.99.1/MSVC, `cargo test --all-targets`.
* Actual WireGuard Noise handshake and authenticated IP packet exchange.
* Replay and tampered-tag rejection; clock regression and output bounds.
* Real TCP loopback carrying the WireGuard handshake and encrypted IP exchange.
* Framing fragmentation, prediction context and reset/error rules.
* Partial writes/backpressure with exact output preservation.
* Packet ownership/bounded queues, config parsing/redaction, PCAP limits,
  NAT-PMP request/refusal, candidate selection and incomplete-parity rejection.
* 1,024 established crypto/framing packet cycles with zero observed per-thread
  allocator calls after warm-up. This excludes startup/rekey/TLS/kernel work.

## Not yet verified

* CI Linux/macOS execution and Linux privileged TUN round trip until the actual
  workflow run succeeds; a workflow file is not a successful test result.
* Complete Proton WireGuard TCP or Stealth client integration.
* Real Proton server/account interoperability and all Plus capabilities.
* Cross-platform connection lifecycle and platform bindings.
* AArch64 router throughput, syscall counts or a speedup versus v3.0.5.

`docs/parity.json` is the authoritative incomplete-feature ledger. Its statuses
distinguish local implementation from live verification. Release readiness fails
while any required item is missing; it must not be bypassed to label a prototype
feature-complete.
