# Verification status

This is development evidence, not live service or release acceptance.
[Implementation coverage](parity-scope.md) and [the requirement ledger](parity.json)
separate executable code from the physical-device/Proton Plus tests deferred by
the user. Historical checkpoints below describe their own source revision.

## November 5 platform TLS revision

The [current selection](tls-selection-20261105.md) keeps unbuffered TLS on
Linux/Android and restores bounded buffered TLS on Windows/Apple. The newer
[CI campaign](benchmarks/decision-hour-20261104.md) and
[Travel rerun](benchmarks/travel-decision-20261105.md) retain successful and
failed transfers, actual LTS versions, repeated comparisons, regression checks
and the experimental io_uring overflow diagnosis. These measurements predate
this platform-selection code change; they justify the selection, not a new
claim of device or Plus acceptance. Validation of this revision is recorded in
the PR checks and current TLS-selection document. Historical checkpoints below
describe their own revisions, including their then-pending checks.

## November 3 performance and platform revision

The [recorded TLS comparison](tls-selection-20261103.md) was completed on Travel
before production changes. It selected unbuffered TLS for sustained bulk traffic,
while recording small-transfer regressions and five unexplained cold-request
failures. It is an adapter comparison, not a measured speedup over v3.0.5.
The two isolated router cohorts and final audit occupied 17 minutes 23 seconds;
Travel remained Direct, with saved configuration, key and installed packages
unchanged and all experiment-owned artifacts removed. Home was untouched.

The revision targets the [current platform contract](supported-platforms.md).
Local Windows checks pass 64 tests including nine TLS regressions; strict full,
no-account and ARM64-musl Clippy and vendor/code-coverage checks also pass.
CI has passed Linux, Windows, Xcode 27, Android native/AAR, real namespace TUN
and ARM64-musl jobs. The macOS 26 job has not obtained a hosted runner, and
the new 6.12/6.18 guest checks are being rerun after correcting their missing
dummy network driver and hardware-acceleration permissions. Those checks are
pending, not passes. No release/device/Plus acceptance or merge is claimed.

The remaining sections preserve earlier checkpoints and their limitations.

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
The exact final source revision and workflow are recorded below.

`verify-code-parity` now passes for 39 functional implementation requirements;
the performance-evidence row is explicitly outside that software gate.
`verify-parity` still exits unsuccessfully because live/device verification is
incomplete. CI checks both outcomes; coverage cannot approve a production release.

The established-session allocation fixture passes 1,024 1,280-byte
encrypt/frame/decode/decrypt cycles with zero allocator calls on its measured
thread after warm-up. It excludes initialization, rekey, TLS, OS I/O and account
work. Host framing timings are not VPN throughput measurements.

## Final coverage checkpoint

[Run 41308673272](https://github.com/riKirsti/ProtunNext/actions/runs/41308673272),
code commit `609e2c4d124104bbdd2dab6db81f00254740a526`, passed **all five jobs**. Windows/Linux/macOS
tests, formatting, full/no-account Clippy, implementation-coverage checks,
expected rejection of unverified release readiness, optimized runtime builds,
generated native Python/Swift contracts, Android API25 Kotlin/service/unit/AAR
and verified Windows driver packaging all succeeded. The isolated Linux job
also passed the real TUN, tracked LAN reply and offline/pending-SYN HTTP tests,
including the no-account build.

The matching local Windows run passed 53 tests. The optimized AArch64-musl CLI
was cross-built from this code revision; SHA256 is
`197cbfa7d74af40ec7dba7be3d72d2b44f22f745a2e06694ae10723a9fa76e27`.
It was not run on a router. This document's later evidence-only commit does
not change the verified runtime code or claim physical/live acceptance.

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

The implementation and subsequent evidence-only documentation changes are available
on [PR1](https://github.com/riKirsti/ProtunNext/pull/1).

## Deferred acceptance

- Live Proton session/fork, production certificate trust/renewal and LocalAgent.
- Applied Plus features, refusals, exits, jails and restrictions on a real account.
- Physical Windows Wintun, Apple utun, Android handover/always-on/revoke and C# host integration.
- Additional Android ABI builds beyond arm64-v8a.
- OpenWrt execution and matched v3.0.5/kernel WireGuard performance measurements.

There is no measured router speedup and no production-readiness assertion.
