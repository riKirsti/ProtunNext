# Stage 8: test-peer capacity screen

[Run and artifacts](https://github.com/riKirsti/ProtunNext/actions/runs/43325789845), source `aa43d3dab035092e0f9ff21c3f811435ef225017`.

## Decision

**Reject both prototypes as a new default or a maximum-client-capacity acceptance fixture.** Retain the existing legacy helper. In-place processing alone produces small, inconsistent changes. A bounded reader materially raises x64 fixture throughput but regresses ARM64, and its wire thread still saturates on both architectures. The maximum-speed client ranking remains **inconclusive**. No new client compiler comparison is run: the reviewed plan requires helper headroom first. Existing Stage 7 efficiency evidence remains unchanged.

The experimental modes remain opt-in, benchmark-only, so these rejected approaches and their evidence are reproducible. No production runtime, dependency, crypto backend, portable compiler default, release or persistent router change is proposed. This stage does not claim that the throughput of an unchanged client was optimized.

## Method

Native GitHub-hosted Ubuntu 24.04 x64 and ARM64; actual kernels/CPU hardware/compiler and complete source/binary identities are retained in the linked data. Rust 1.99.1 / LLVM 23.1.1, normal portable release, thin LTO, one codegen unit, panic=abort, unix without Local Agent, existing ring and serial client.

Each architecture tests two independent helper strategies against legacy, each with UDP, TCP and Stealth, IPv4 HTTP and one inner flow. Every cell contains four baseline A/A observations followed by ABBA, each transferring exactly 2 GiB after warmup. All 96/96 measured payloads verify; all 12 cells pass existing A/A and timing screens. The client and all helper roles use the same single frozen baseline binary within each host; report labels main/erase identify helper conditions, not different clients. Binary digests are checked against artifact bytes and all three metric roles. Both archive runtime fingerprints match `5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`.

Short correctness smokes also exercise all transports with IPv6 HTTPS/eight flows and delayed Stealth startup. Those are not sustained IPv6/eight-flow performance measurements. No full tuned-client capacity matrix follows a failed helper-headroom screen.

Client CPU allowance remains CPU2. Legacy/in-place wire uses CPU0, load CPU1, server CPU3. Reader mode moves TUN reads to CPU3, keeps wire on CPU0, and shares CPU1 between load and server. The bounded reader reserves 64 × 65,000 bytes (3.967 MiB) of payload storage plus queue/thread overhead; this is auxiliary memory, not client RSS. It erases initialized plaintext before recycling and full storage on destruction. No session or key is moved to that reader.

CPU % is process CPU/wall time, 100% = one core; client RSS is lifetime peak process MiB including warmup/setup. Kernel and auxiliary memory are excluded from client RSS. Auxiliary wire, reader, server and load CPU/RSS are listed separately. Actual auxiliary-core busy time includes asynchronous kernel work, with the server/load sum counted against their shared core. The unchanged 0.85 auxiliary screen is a benchmark headroom check, not a client CPU limit. Every condition is still auxiliary-limited, so throughput values describe this fixture only. No maximum-capacity ranking is accepted.

## Client observations

Values are geometric means of the two balanced comparison observations per condition, B = legacy helper and C = the stated experimental helper. A/A observations remain in the data; no values are pooled between independent strategy cohorts.

| Platform | Helper candidate | Transport | Client CPU % B → C | Client RSS MiB B → C | Verified Mbps B → C | Mbps change | A/A |
|---|---|---|---:|---:|---:|---:|---|
| x64 | inplace | udp | 71.197 → 71.998 | 8.497 → 7.982 | 3913.897 → 3909.847 | -0.10% | usable |
| x64 | inplace | tcp | 74.244 → 74.183 | 8.064 → 7.719 | 4120.227 → 4093.765 | -0.64% | usable |
| x64 | inplace | stealth | 74.712 → 75.376 | 5.310 → 5.178 | 3824.016 → 3844.742 | +0.54% | usable |
| x64 | reader | udp | 71.090 → 92.640 | 8.095 → 8.626 | 3877.132 → 5476.784 | +41.26% | usable |
| x64 | reader | tcp | 73.991 → 96.181 | 7.837 → 7.656 | 4171.342 → 5769.198 | +38.31% | usable |
| x64 | reader | stealth | 74.668 → 96.660 | 5.378 → 5.410 | 3829.996 → 5206.120 | +35.93% | usable |
| arm | inplace | udp | 85.404 → 86.019 | 7.399 → 6.929 | 3083.720 → 3094.857 | +0.36% | usable |
| arm | inplace | tcp | 84.758 → 85.625 | 6.824 → 7.222 | 3240.040 → 3288.758 | +1.50% | usable |
| arm | inplace | stealth | 85.808 → 88.232 | 4.533 → 4.529 | 2668.987 → 2726.511 | +2.16% | usable |
| arm | reader | udp | 85.689 → 75.868 | 7.345 → 7.597 | 3104.922 → 2668.093 | -14.07% | usable |
| arm | reader | tcp | 83.614 → 74.416 | 6.646 → 7.097 | 3221.006 → 2801.719 | -13.02% | usable |
| arm | reader | stealth | 87.630 → 77.281 | 4.472 → 4.453 | 2699.699 → 2373.068 | -12.10% | usable |

## Auxiliary observations

Arithmetic means of the two measured comparison observations; CPU percentages are per role. Peer RSS includes wire, HTTP server and optional reader; load is a separate process. The maximum auxiliary fraction includes host busy time and shared roles.

| Platform | Cohort | Transport | Condition | Wire CPU % | Reader CPU % | Server CPU % | Load CPU % | Peer RSS MiB | Load RSS MiB | Max auxiliary core fraction |
|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| x64 | inplace | udp | legacy | 99.660 | 0.000 | 5.968 | 12.759 | 10.279 | 3.816 | 1.0000 |
| x64 | inplace | udp | inplace | 99.461 | 0.000 | 6.182 | 12.813 | 10.252 | 3.906 | 0.9978 |
| x64 | inplace | tcp | legacy | 99.677 | 0.000 | 5.988 | 13.229 | 10.268 | 3.834 | 1.0000 |
| x64 | inplace | tcp | inplace | 99.466 | 0.000 | 6.018 | 13.271 | 10.299 | 3.951 | 0.9976 |
| x64 | inplace | stealth | legacy | 99.640 | 0.000 | 5.719 | 12.494 | 10.930 | 3.828 | 1.0000 |
| x64 | inplace | stealth | inplace | 99.737 | 0.000 | 5.688 | 12.556 | 10.930 | 3.838 | 1.0000 |
| x64 | reader | udp | legacy | 99.617 | 0.000 | 6.289 | 12.533 | 10.248 | 3.846 | 1.0000 |
| x64 | reader | udp | reader | 99.323 | 57.890 | 9.472 | 15.412 | 13.207 | 3.928 | 0.9968 |
| x64 | reader | tcp | legacy | 99.610 | 0.000 | 6.282 | 13.120 | 10.238 | 3.951 | 0.9976 |
| x64 | reader | tcp | reader | 99.302 | 60.162 | 9.681 | 15.908 | 13.184 | 3.807 | 0.9967 |
| x64 | reader | stealth | legacy | 99.639 | 0.000 | 5.748 | 12.366 | 10.898 | 3.865 | 1.0000 |
| x64 | reader | stealth | reader | 98.942 | 55.752 | 8.853 | 14.696 | 13.730 | 3.881 | 0.9969 |
| arm | inplace | udp | legacy | 99.756 | 0.000 | 4.644 | 9.066 | 9.305 | 2.928 | 1.0000 |
| arm | inplace | udp | inplace | 99.780 | 0.000 | 4.586 | 9.164 | 9.295 | 2.920 | 1.0000 |
| arm | inplace | tcp | legacy | 99.764 | 0.000 | 5.144 | 9.481 | 9.301 | 2.922 | 1.0000 |
| arm | inplace | tcp | inplace | 99.736 | 0.000 | 5.227 | 9.568 | 9.307 | 2.930 | 1.0000 |
| arm | inplace | stealth | legacy | 99.818 | 0.000 | 3.530 | 8.300 | 9.895 | 2.928 | 1.0000 |
| arm | inplace | stealth | inplace | 99.670 | 0.000 | 3.728 | 8.075 | 9.895 | 2.922 | 1.0000 |
| arm | reader | udp | legacy | 99.783 | 0.000 | 4.551 | 9.088 | 9.309 | 2.932 | 1.0000 |
| arm | reader | udp | reader | 99.532 | 43.752 | 4.078 | 9.149 | 12.170 | 2.932 | 0.9984 |
| arm | reader | tcp | legacy | 99.761 | 0.000 | 4.790 | 9.466 | 9.307 | 2.928 | 1.0000 |
| arm | reader | tcp | reader | 99.741 | 46.818 | 4.280 | 9.571 | 12.223 | 2.928 | 0.9984 |
| arm | reader | stealth | legacy | 99.803 | 0.000 | 3.614 | 7.897 | 9.895 | 2.926 | 1.0000 |
| arm | reader | stealth | reader | 99.777 | 39.429 | 3.641 | 8.261 | 12.941 | 2.928 | 0.9986 |

## Evidence and limits

[All 96 samples, plans, analyses and identities](stage8-data/helper-screen.json); [x64 artifact/source manifest](stage8-data/helper-x64-manifest.json); [ARM64 artifact/source manifest](stage8-data/helper-arm-manifest.json). Each artifact digest and executable digest is recomputed locally, and each saved analysis is reproduced with the current analyzer (floating arithmetic tolerance 1e-12). Both native jobs and 57 injected Rust library regressions per architecture pass. The 24 download-analyzer and 12 audit-analyzer tests pass, including shared-core saturation, wrong placement, absent reader, nonfinite and incomplete CPU accounting.

[Frozen system profiles and delayed-TCP diagnosis](stage8-diagnostics.md) are separate from these normal-release values. [Diagnostic counter summary](stage8-data/tcp-diagnostics.json) retains per-case rates, final qdisc counters, TCP counters and artifact hashes. The prior TCP misses remain unresolved; this diagnostic run did not reproduce them.

[Temporary physical Travel tests](stage8-travel.md) report fresh portable PGO separately. Windows, Apple, Android, supported-LTS guests, account-enabled Local Agent and physical NIC performance are not measured in this stage. Standard build/API/erasure regressions do not supply missing performance evidence. Loopback results do not establish WAN, RTT/loss or thermal behavior.
