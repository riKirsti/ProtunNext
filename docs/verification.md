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

[Run 41221611485](https://github.com/riKirsti/ProtunNext/actions/runs/41221611485),
commit `61cf2a758633c01770f34cc385b579e5a3e7b937`, passed on all three
host platforms, plus real TUN round trips through UDP, framed TCP and standard
certificate-verifying nonblocking TLS in the isolated Linux job.

[Run 41221982426](https://github.com/riKirsti/ProtunNext/actions/runs/41221982426),
commit `71bfa70afd88ff7f1babb0072a156fb7b5a97e6a`, also passed all four jobs.
Its Linux test adds the explicit Stealth outer policy (untrusted test CA and
mismatched outer SNI, but valid inner WG peer) and checks that a wrong inner WG
peer delivers no data. This is real local protocol I/O, not a live Proton server
or paid subscription verification.

The first managed-worker run (`41222476857`, commit `326e90e`) exposed an
IPv6 TUN control packet reaching the IPv4-only reference peer. It was not a
passing integration result. The following change enforces the configured
AllowedIPs before outbound encryption and after authenticated inbound decryption.
Its expanded test checks wrong inner identity, spoofed source, disallowed
destination, requested statistics, repeated shutdown/join and a final TCP reply
followed immediately by EOF. These changes need their own successful CI result.

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
