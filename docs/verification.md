# Verification status

This is development evidence, not live service or release acceptance.
[Implementation coverage](parity-scope.md) and [the requirement ledger](parity.json)
separate executable code from the physical-device/Proton Plus tests deferred by
the user. No router was contacted or modified in this work.

## Current local checks

Rust1.99.1/MSVC all-target/all-feature tests pass on Windows. They include actual
Noise exchanges over UDP, TCP, authenticated TLS and Stealth, replay/tamper
rejection, partial/vectored writes and bounded backpressure. Private-stack
fixtures execute real TCP/UDP translation, customer collisions, peer-initiated
reserved ports, fragmented private UDP reassembly and alternate-subnet DNS/HTTP.

The account fixture executes published Muon/RCRL code over actual private TCP/IP
stacks with public test tokens and a test Ed25519 identity. It checks successful
certificate issuance/persistence/replacement, request features/duration, session
rejection, fork replacement, cookie delivery and consumed-selector suppression.
It does not contact Proton or prove production LocalAgent trust.

The optional development helpers have bounded INI-parser/error fixtures and real
host-adapter loopback TCP checks. Their authentication test rejects invalid app
configuration before networking. No real login, password or 2FA was used.

Windows and AArch64-musl all-target/all-feature Clippy pass with warnings denied.
The no-account native build also passes Clippy. An optimized AArch64-musl full
runtime cross-build has passed; that establishes linking, not router execution.
The final source revision needs its own final build/check record below.

`verify-code-parity` now passes for 39 functional implementation requirements;
the performance-evidence row is explicitly outside that software gate.
`verify-parity` still exits unsuccessfully because live/device verification is
incomplete. CI checks both outcomes; coverage cannot approve a production release.

The established-session allocation fixture passes 1,024 1,280-byte
encrypt/frame/decode/decrypt cycles with zero allocator calls on its measured
thread after warm-up. It excludes initialization, rekey, TLS, OS I/O and account
work. Host framing timings are not VPN throughput measurements.

## Recorded CI evidence

The last complete five-job passing checkpoint before the latest lifecycle work
is [41271451414](https://github.com/riKirsti/ProtunNext/actions/runs/41271451414),
commit `104fec848a2b98c1e1dd59b30ede1f4754daafbb`. Windows/Linux/macOS tests,
formatting, Clippy and optimized runtime builds passed. Generated Python invoked
real Unix constructor/cache/state/shutdown and Windows initialization; generated
Swift typechecked. Android built the API25 arm64 native library, generated Kotlin,
compiled the service/unit tests and assembled the AAR. Windows packaging verified
the official Wintun archive hash, signature and CPU architecture without loading
or installing the driver.

The Linux namespace job of
[41275189414](https://github.com/riKirsti/ProtunNext/actions/runs/41275189414),
commit `194ef1a21eb572037883a89fe9a35faa5b3918be`, passed three integration tests:
eight TUN transport/filter/EOF scenarios, a tracked LAN reply through real TUN,
and a local HTTP TCP session retained across worker replacement and an unanswered
SYN. The last case also passed without account SDKs. All network changes were
inside disposable namespaces removed by CI. **That workflow was not a full pass:**
host jobs stopped at a module-order formatting difference, now corrected.
Superseded lifecycle runs were cancelled when newer commits were pushed;
a successful individual job is not represented as a successful complete run.

The final workflow for the current coverage commit is recorded after completion;
until then, consult the exact source revision and checks on
[PR1](https://github.com/riKirsti/ProtunNext/pull/1).

## Deferred acceptance

- Live Proton session/fork, production certificate trust/renewal and LocalAgent.
- Applied Plus features, refusals, exits, jails and restrictions on a real account.
- Physical Windows Wintun, Apple utun, Android handover/always-on/revoke and C# host integration.
- Additional Android ABI builds beyond arm64-v8a.
- OpenWrt execution and matched v3.0.5/kernel WireGuard performance measurements.

There is no measured router speedup and no production-readiness assertion.
