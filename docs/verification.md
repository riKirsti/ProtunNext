# Verification status

Development evidence, not a release acceptance record.

## Current compatibility work

[Run 41248700583](https://github.com/riKirsti/ProtunNext/actions/runs/41248700583)
at `2a5d1809e2184a94140c7e5bf94183161c444dd6` passed all five jobs:
Windows/Linux/macOS tests, formatting, Clippy and release builds; isolated Linux
TUN tests; and AArch64 Android API25 native library/Kotlin generation.
Linux also exercises generated Python constructor/cache/state/shutdown callbacks;
macOS typechecks generated Swift and tests utun prefix I/O with real datagrams.

Later changes add the shared worker, encrypted connectivity probes, monotonic
statistics timestamps, reentrant shutdown checks and a private account fixture.
The fixture executes published Muon/RCRL code over actual in-memory IP/TCP
stacks with public test tokens. It verifies certificate JSON/settings/duration,
rejected-auth cache clearing, a replacement fork GET, cookies and suppression of
a consumed selector. It does not simulate a successful paid Proton connection.
The latest CI link and source hash must be checked before treating a newer
checkpoint as verified.

`verify-code-parity` reports missing/partial implementation independently from
the release gate. The matched router benchmark is evidence-only for this code
gate; live service and target acceptance remain deferred in `verify-parity`.
Both gates currently fail intentionally: full code coverage is still unfinished.

The CLI is named `protun-next-cli`; the separate name avoids an MSVC debug-symbol
filename collision with the `protun_next` library. `cargo run` selects it by default.

## Earlier implementation evidence

## Executed locally on Windows x86-64

* Rust 1.99.1/MSVC, `cargo test --all-targets --all-features`.
* Actual WireGuard Noise handshake and authenticated IP packet exchange.
* Replay and tampered-tag rejection; clock regression and output bounds.
* Real TCP and nonblocking TLS loopback carrying the WireGuard handshake and encrypted IP exchange.
* Standard TLS certificate verification and wrong-server-name rejection.
* An 80,000-byte real TLS exchange with vectored plaintext, a full bounded
  pre-handshake buffer, forced 257-byte socket writes and repeated WouldBlock;
  the accepted bytes arrive intact.
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
followed immediately by EOF. [Run 41222836845](https://github.com/riKirsti/ProtunNext/actions/runs/41222836845),
commit `55fcaaa7dc05a8029d90f2124c94ab3cdff00cc6`, passed all four jobs.
The isolated Linux test executes eight scenarios: UDP, TCP, standard TLS,
Stealth, wrong WG identity, authenticated wrong IP source, disallowed outbound
destination, and a valid final TCP reply immediately followed by EOF. It also
checks fresh requested statistics, authenticated tunnel state, and repeated join.

The following TLS batching refinement passes locally (22 all-feature tests);
its own workflow must confirm the cross-platform result. It forwards all
IoSlices to rustls's published vectored writer rather than inheriting the default
single-slice fallback. This is an I/O behavior fix; no throughput speedup has
been measured.

## Cross compilation

The development laptop successfully built the all-feature release executable for
`aarch64-unknown-linux-musl`, including the optional official Local Agent adapter. This proves compilation/linking, not execution on
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
