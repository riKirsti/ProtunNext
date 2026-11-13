# Final performance validation

The [all-stage final selection](../performance-policy.md) retains merged PR #13,
`7c582a28a528701ef3b3874f476903ab10f205a7`. Tested closing source: `2a6aeb9`.
Subsequent changes are documentation/evidence only. Both production fingerprints
are `5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`.
No runtime, dependency, product feature or portable compiler policy changes.

Independent baseline/current builds produce byte-identical executables within
each native host and on Travel. Source and actual executable hashes are checked;
observed differences are measurement variation, not a code or compiler speedup.

**Final decision:** retain serial ring, compact bounded ownership, direct framed/
TLS receive, explicitly negotiated Linux receive coalescing and portable release
defaults. Reject the examined parallel/provider/helper replacements as defaults.
Keep matching target-specific PGO/CPU experiments optional. The current campaign
ends with this PR; unresolved evidence does not schedule another performance PR.

## Final native resource validation

[GitHub Actions run and artifacts](https://github.com/riKirsti/ProtunNext/actions/runs/43332170085)
use normal portable release, Rust 1.99.1 / LLVM 23.1.1, thin LTO, one codegen unit,
panic=abort, unix without Local Agent, vnet and one client CPU. The unchanged
legacy baseline peer, load and server are fixed auxiliaries. Production product
defaults retain Local Agent; these measurements do not qualify account-enabled
CLI/SDK performance. Actual host hardware, kernels, compiler, runtime/effective/
archive/executable hashes and all samples remain in the data.

Each architecture runs all twelve UDP/TCP/Stealth × IPv4 HTTP/IPv6 HTTPS ×
one/eight-inner-flow cells, four baseline A/A then balanced comparison. Each
sample verifies exactly 128 MiB after warmup at requested aggregate 200 Mbps.
The analyzer verifies identities, namespaces, actual coalescing, payload/rate,
timing, controls and errors. Per-transport summaries are geometric means of
four cells and retain their usable counts; any invalid cell makes its aggregate
n/a. Inconclusive controls are never removed to improve a summary.

CPU 100% = one core of client process time, including synchronous kernel calls;
RSS = lifetime peak client-process MiB including setup/warmup, excluding kernel/
auxiliary memory. Mbps = verified application payload rate. Asynchronous kernel
CPU is excluded from process time. These equal-runtime, rate-controlled samples
are validation/characterization, not a new optimization or maximum-speed ranking.

Payloads verified: **192/192**. **24/24 cells pass control/timing screens**.
Analyzer errors: **0**. Every original control remains in the linked data.

| Platform | Transport | CPU % B → current | RSS MiB B → current | Mbps B → current | Usable cells |
|---|---|---:|---:|---:|---:|
| x64 | udp | 6.755 → 6.727 | 8.111 → 8.025 | 200.385 → 200.385 | 4/4 |
| x64 | tcp | 6.741 → 6.754 | 7.552 → 7.509 | 200.385 → 200.386 | 4/4 |
| x64 | stealth | 7.544 → 7.520 | 5.231 → 5.331 | 200.381 → 200.382 | 4/4 |
| arm | udp | 6.035 → 6.083 | 7.553 → 7.445 | 200.397 → 200.398 | 4/4 |
| arm | tcp | 5.677 → 5.799 | 6.644 → 6.558 | 200.399 → 200.398 | 4/4 |
| arm | stealth | 6.860 → 6.887 | 4.725 → 4.678 | 200.393 → 200.392 | 4/4 |

[All x64 samples/identities](final-data/native-x64.json),
[all ARM64 samples/identities](final-data/native-arm.json),
[validation index](final-data/native-index.json). Executable hashes are recomputed
from artifact bytes; saved analyses reproduce locally within 1e-12 floating
arithmetic tolerance. The original Stage 7/8 limits and failures are preserved
under their actual source identities, not relabeled as fixed by these runs.

## Final actual temporary Travel validation

Cudy TR3000, Cortex-A53, Linux 6.12.112, MTU 1280; portable ARM64-musl release,
unix without Local Agent, negotiated coalescing and no PGO/CPU-specialization.
Baseline/current have identical production sources. Each planned 400 MB session
uses four consecutive separately SHA256-verified 100 MB HTTPS connections through
one continuous worker. Four A/A observations precede baseline/current/current/
baseline. CPU % is whole-process; RSS lifetime peak; WAN Mbps includes setup and
laptop validation. Neither noisy controls nor differences between unchanged
builds establish an optimization gain. Full measurements/failures and independent
restoration are in the [physical report](final-travel.md).

| Transport | CPU % B → current | RSS MiB B → current | Mbps B → current | Verified sessions | A/A |
|---|---:|---:|---:|---:|---|
| udp | 59.638 → 57.407 | 9.799 → 9.991 | 234.140 → 225.630 | 7/8 | fails/incomplete |
| tcp | 56.851 → 57.665 | 2.576 → 2.535 | 231.079 → 235.949 | 8/8 | passes |
| stealth | 67.136 → 70.872 | 3.379 → 3.401 | 198.644 → 215.678 | 8/8 | passes |

Verified **23/24 planned sessions**. The first baseline UDP session fails warmup before timing; retain its failed attempts and incomplete control cohort. No replacement cohort or favorable subset is selected. Cleanup and all twelve independent restoration checks pass after each transport.

## Boundaries and campaign closure

The physical tests use unique owned temporary files/interfaces and narrow
socket-bound forwarding rules, with a separate 600-second watchdog per cohort.
Saved configuration/account/key, packages, limits, Direct mode and normal routing
are unchanged; owned files/processes/guard/interfaces are absent afterward.
No Home router, permanent installation, persistent NIC setting or release.

Windows, Apple and Android are build/API/functional checked; closing-stage
download performance is unmeasured. Linux LTS functional checks are distinct from
native-host speed measurements. Account-enabled CLI/SDK capacity, physical NIC/
RTT/loss/thermal behavior, a peer-independent maximum and old delayed-TCP misses
remain explicit limits. Stage 8 did not reproduce those misses and did not prove
a fix. Final platform/erasure check links are also retained in the PR.

The [final policy](../performance-policy.md) reviews each stage, preserves
positive/negative evidence and gives the selected build/TUN guidance. No more
performance PRs are planned unless the user reopens this work. General live/paid
feature release acceptance remains governed by the existing parity gate.

## Every final native workload

### Native Linux x64 / 200 Mbps

| Workload | CPU % B → current | RSS MiB B → current | Verified Mbps B → current | Control/timing status |
|---|---:|---:|---:|---|
| udp-v4-http-1 | 6.240 → 6.134 | 7.941 → 7.876 | 200.075 → 200.075 | usable |
| udp-v4-http-8 | 7.006 → 6.970 | 8.031 → 8.163 | 200.702 → 200.702 | usable |
| udp-v6-https-1 | 6.422 → 6.432 | 8.127 → 7.775 | 200.070 → 200.072 | usable |
| udp-v6-https-8 | 7.417 → 7.449 | 8.351 → 8.300 | 200.694 → 200.694 | usable |
| tcp-v4-http-1 | 6.304 → 6.249 | 7.316 → 7.431 | 200.075 → 200.076 | usable |
| tcp-v4-http-8 | 6.871 → 6.851 | 8.079 → 7.898 | 200.703 → 200.703 | usable |
| tcp-v6-https-1 | 6.583 → 6.667 | 7.283 → 7.286 | 200.074 → 200.071 | usable |
| tcp-v6-https-8 | 7.241 → 7.288 | 7.558 → 7.436 | 200.691 → 200.697 | usable |
| stealth-v4-http-1 | 6.824 → 6.881 | 5.094 → 5.166 | 200.073 → 200.074 | usable |
| stealth-v4-http-8 | 7.729 → 7.660 | 5.735 → 5.886 | 200.697 → 200.697 | usable |
| stealth-v6-https-1 | 7.487 → 7.481 | 5.004 → 5.125 | 200.069 → 200.071 | usable |
| stealth-v6-https-8 | 8.205 → 8.109 | 5.124 → 5.185 | 200.687 → 200.686 | usable |

### Native Linux arm / 200 Mbps

| Workload | CPU % B → current | RSS MiB B → current | Verified Mbps B → current | Control/timing status |
|---|---:|---:|---:|---|
| udp-v4-http-1 | 5.998 → 6.110 | 7.732 → 7.017 | 200.078 → 200.078 | usable |
| udp-v4-http-8 | 5.692 → 5.695 | 7.859 → 8.207 | 200.720 → 200.721 | usable |
| udp-v6-https-1 | 6.555 → 6.651 | 7.089 → 6.904 | 200.076 → 200.077 | usable |
| udp-v6-https-8 | 5.927 → 5.916 | 7.552 → 7.726 | 200.714 → 200.715 | usable |
| tcp-v4-http-1 | 5.549 → 5.704 | 6.843 → 6.261 | 200.079 → 200.078 | usable |
| tcp-v4-http-8 | 5.476 → 5.471 | 6.791 → 6.535 | 200.723 → 200.723 | usable |
| tcp-v6-https-1 | 6.067 → 6.339 | 6.014 → 6.337 | 200.077 → 200.077 | usable |
| tcp-v6-https-8 | 5.635 → 5.717 | 6.972 → 7.135 | 200.717 → 200.717 | usable |
| stealth-v4-http-1 | 6.641 → 6.735 | 4.385 → 4.408 | 200.077 → 200.077 | usable |
| stealth-v4-http-8 | 6.546 → 6.552 | 5.469 → 5.090 | 200.713 → 200.711 | usable |
| stealth-v6-https-1 | 7.485 → 7.496 | 4.462 → 4.463 | 200.075 → 200.074 | usable |
| stealth-v6-https-8 | 6.807 → 6.803 | 4.658 → 4.784 | 200.706 → 200.706 | usable |
